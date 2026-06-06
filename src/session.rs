//! The resolved run plan.
//!
//! [`Session`] is the single source of truth the [`crate::app`] consumes. It
//! merges, in increasing precedence: built-in defaults → persisted
//! [`Config`] → a `--preset` → explicit CLI flags. The result is concrete
//! durations, display preferences, and an explicit, ordered list of phases.

use std::time::Duration;

use crate::cli::Cli;
use crate::config::Config;
use crate::theme::DEFAULT_THEME;
use crate::{Phase, git};

/// A fully resolved session ready to execute.
#[derive(Debug, Clone)]
pub struct Session {
    pub work: Duration,
    pub short_break: Duration,
    pub long_break: Duration,
    pub cycles: u64,
    pub long_break_enabled: bool,
    pub long_break_every: u64,
    /// Increment used by the interactive extend/shrink controls (1 min, or 1 s
    /// in `--seconds` mode).
    pub step: Duration,
    pub sound: bool,
    pub notifications: bool,
    pub color: bool,
    pub plain: bool,
    /// Wait for a keypress between phases instead of auto-advancing.
    pub auto_advance: bool,
    pub theme: String,
    pub fps: u32,
    /// Resolved interface language code (e.g. `"en"`, `"de"`).
    pub lang: String,
    pub label: Option<String>,
}

/// A named bundle of timer defaults selectable with `--preset`.
#[derive(Debug, Clone, Copy)]
pub struct Preset {
    pub work: u64,
    pub brk: u64,
    pub long: u64,
    pub long_every: u64,
    pub cycles: u64,
    pub long_enabled: bool,
}

/// All preset names, in display order.
pub const PRESET_NAMES: &[&str] = &["classic", "deep", "short", "sprint"];

/// Look up a preset by name (case-insensitive).
pub fn preset(name: &str) -> Option<Preset> {
    match name.to_ascii_lowercase().as_str() {
        // The textbook Pomodoro: four 25/5 rounds with a long break at the end.
        "classic" => Some(Preset { work: 25, brk: 5, long: 15, long_every: 4, cycles: 4, long_enabled: true }),
        // Long, deliberate focus blocks.
        "deep" => Some(Preset { work: 50, brk: 10, long: 20, long_every: 3, cycles: 3, long_enabled: true }),
        // Short, snappy rounds.
        "short" => Some(Preset { work: 15, brk: 3, long: 10, long_every: 4, cycles: 6, long_enabled: true }),
        // A single quick sprint.
        "sprint" => Some(Preset { work: 20, brk: 5, long: 15, long_every: 4, cycles: 1, long_enabled: false }),
        _ => None,
    }
}

impl Session {
    /// Combine defaults, config, an optional preset, and CLI flags.
    pub fn resolve(cli: &Cli, config: &Config) -> Session {
        let preset = cli.preset.as_deref().and_then(preset);

        // Base timer values: config, overridden by a preset if one was chosen.
        let base_work = preset.map_or(config.work_minutes, |p| p.work);
        let base_break = preset.map_or(config.break_minutes, |p| p.brk);
        let base_long = preset.map_or(config.long_break_minutes, |p| p.long);
        let base_cycles = preset.map_or(config.cycles, |p| p.cycles);
        let base_long_every = preset.map_or(config.long_break_every, |p| p.long_every);
        let base_long_enabled = preset.map_or(config.long_break, |p| p.long_enabled);

        // Explicit CLI flags win over everything.
        let work = cli.work.unwrap_or(base_work);
        let short = cli.brk.unwrap_or(base_break);
        let long = cli.long_break.unwrap_or(base_long);

        // A unit of "1" means 1 minute normally, or 1 second in --seconds mode.
        let unit = if cli.seconds { 1 } else { 60 };
        let dur = |minutes: u64| Duration::from_secs(minutes.saturating_mul(unit));

        let long_break_enabled =
            cli.long || cli.long_break.is_some() || cli.long_every.is_some() || base_long_enabled;

        let label = cli.label.clone().or_else(|| {
            if cli.git_label || config.git_label {
                git::current_branch()
            } else {
                None
            }
        });

        let color =
            !cli.no_color && std::env::var_os("NO_COLOR").is_none() && std::io::IsTerminal::is_terminal(&std::io::stdout());

        Session {
            work: dur(work),
            short_break: dur(short),
            long_break: dur(long),
            cycles: cli.cycles.unwrap_or(base_cycles).clamp(1, 10_000),
            long_break_enabled,
            long_break_every: cli.long_every.unwrap_or(base_long_every).max(1),
            step: Duration::from_secs(unit),
            sound: !cli.no_sound && config.sound,
            notifications: !cli.no_notify && config.notifications,
            color,
            plain: cli.plain,
            // --wait forces manual advance; otherwise honour the config.
            auto_advance: !cli.wait && config.auto_advance,
            theme: cli.theme.clone().unwrap_or_else(|| {
                if config.theme.is_empty() { DEFAULT_THEME.to_string() } else { config.theme.clone() }
            }),
            fps: cli.fps.unwrap_or(config.fps),
            lang: crate::i18n::I18n::detect(cli.lang.as_deref(), Some(&config.language))
                .code()
                .to_string(),
            label,
        }
    }

    /// The ordered phases for the whole run.
    ///
    /// Each cycle is a focus block followed by a break. The break is a long
    /// break when long breaks are enabled and the just-finished focus block is a
    /// multiple of `long_break_every`.
    ///
    /// The trailing break of the final cycle is normally omitted — there is
    /// nothing left to rest *for*. The one exception is an *earned* long break:
    /// if the final focus block lands on a long-break boundary, the long break
    /// is kept as the session finale (the classic "four rounds, then a long
    /// break" flow). This is what the `classic`/`deep` presets advertise.
    pub fn phases(&self) -> Vec<(Phase, Duration)> {
        let mut plan = Vec::with_capacity(self.cycles.saturating_mul(2) as usize);
        for cycle in 1..=self.cycles {
            plan.push((Phase::Focus, self.work));

            let is_long = self.long_break_enabled && cycle % self.long_break_every == 0;
            if cycle == self.cycles {
                if is_long {
                    plan.push((Phase::LongBreak, self.long_break));
                }
                continue;
            }
            if is_long {
                plan.push((Phase::LongBreak, self.long_break));
            } else {
                plan.push((Phase::ShortBreak, self.short_break));
            }
        }
        plan
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn session(cycles: u64, long_every: u64, long: bool) -> Session {
        Session {
            work: Duration::from_secs(60),
            short_break: Duration::from_secs(30),
            long_break: Duration::from_secs(90),
            cycles,
            long_break_enabled: long,
            long_break_every: long_every,
            step: Duration::from_secs(60),
            sound: false,
            notifications: false,
            color: false,
            plain: false,
            auto_advance: true,
            theme: "coffee".into(),
            fps: 15,
            lang: "en".into(),
            label: None,
        }
    }

    fn kinds(s: &Session) -> Vec<Phase> {
        s.phases().into_iter().map(|(p, _)| p).collect()
    }

    #[test]
    fn single_cycle_has_no_trailing_break() {
        assert_eq!(kinds(&session(1, 4, true)), vec![Phase::Focus]);
    }

    #[test]
    fn trailing_short_break_is_omitted() {
        // 2 cycles, no long break -> F, sb, F (last short break dropped).
        assert_eq!(
            kinds(&session(2, 4, false)),
            vec![Phase::Focus, Phase::ShortBreak, Phase::Focus]
        );
    }

    #[test]
    fn classic_ends_on_an_earned_long_break() {
        // 4 cycles, long every 4 -> F sb F sb F sb F LB (the earned finale).
        assert_eq!(
            kinds(&session(4, 4, true)),
            vec![
                Phase::Focus,
                Phase::ShortBreak,
                Phase::Focus,
                Phase::ShortBreak,
                Phase::Focus,
                Phase::ShortBreak,
                Phase::Focus,
                Phase::LongBreak,
            ]
        );
    }

    #[test]
    fn long_break_lands_mid_session_too() {
        // 6 cycles, long every 4 -> long break after cycle 4, none trailing.
        let k = kinds(&session(6, 4, true));
        assert_eq!(k.iter().filter(|p| **p == Phase::LongBreak).count(), 1);
        assert_eq!(k.last(), Some(&Phase::Focus)); // 6 % 4 != 0 -> trailing dropped
    }
}
