//! The resolved run plan.
//!
//! [`Session`] is the single source of truth the [`crate::timer`] consumes. It
//! merges built-in defaults, the persisted [`crate::config::Config`], and the
//! CLI flags (in that order of increasing precedence) into concrete durations
//! and an explicit, ordered list of [`Phase`]s.

use std::time::Duration;

use crate::cli::Cli;
use crate::config::Config;
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
    pub sound: bool,
    pub notifications: bool,
    pub color: bool,
    pub label: Option<String>,
}

impl Session {
    /// Combine config + CLI flags into a runnable plan.
    pub fn resolve(cli: &Cli, config: &Config) -> Session {
        // A unit of "1" means 1 minute normally, or 1 second in --seconds mode.
        let unit = if cli.seconds { 1 } else { 60 };
        let dur = |minutes: u64| Duration::from_secs(minutes.saturating_mul(unit));

        let work = cli.work.unwrap_or(config.work_minutes);
        let short = cli.brk.unwrap_or(config.break_minutes);
        let long = cli.long_break.unwrap_or(config.long_break_minutes);

        // --long-break or --long-every implies the long break is enabled.
        let long_break_enabled =
            cli.long || cli.long_break.is_some() || cli.long_every.is_some() || config.long_break;

        let label = cli.label.clone().or_else(|| {
            if cli.git_label || config.git_label {
                git::current_branch()
            } else {
                None
            }
        });

        Session {
            work: dur(work),
            short_break: dur(short),
            long_break: dur(long),
            // Clamp to a sane range: at least 1, and an upper bound that keeps
            // the run finite and the phase-list allocation from overflowing.
            cycles: cli.cycles.unwrap_or(config.cycles).clamp(1, 10_000),
            long_break_enabled,
            long_break_every: cli.long_every.unwrap_or(config.long_break_every).max(1),
            sound: !cli.no_sound && config.sound,
            notifications: !cli.no_notify && config.notifications,
            color: !cli.no_color,
            label,
        }
    }

    /// The ordered phases for the whole run.
    ///
    /// Each cycle is a focus block followed by a break. The break is a long
    /// break when long breaks are enabled and the just-finished focus block is a
    /// multiple of `long_break_every`. The trailing break of the final cycle is
    /// omitted — there is nothing left to rest *for*.
    pub fn phases(&self) -> Vec<(Phase, Duration)> {
        let mut plan = Vec::with_capacity(self.cycles.saturating_mul(2) as usize);
        for cycle in 1..=self.cycles {
            plan.push((Phase::Focus, self.work));

            let is_last = cycle == self.cycles;
            if is_last {
                continue;
            }

            if self.long_break_enabled && cycle % self.long_break_every == 0 {
                plan.push((Phase::LongBreak, self.long_break));
            } else {
                plan.push((Phase::ShortBreak, self.short_break));
            }
        }
        plan
    }
}
