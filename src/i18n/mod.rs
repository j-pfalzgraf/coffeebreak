//! Internationalisation.
//!
//! coffeebreak is fully localised and defaults to English. The design is small
//! and dependency-free:
//!
//! * [`Msg`] is a compile-time catalogue of every user-facing string. Its
//!   [`Msg::en`] method is the **canonical English** source — being an exhaustive
//!   `match`, the compiler guarantees no message is ever missing in English.
//! * Each other locale is a plain `&[(Msg, &str)]` table (see the `de`, `es`, …
//!   submodules). Any message a translation omits falls back to English, so a
//!   partial translation is always safe.
//! * [`I18n`] is the object threaded through the app. It holds the active locale
//!   and resolves messages, interpolates `{named}` arguments, and pluralises
//!   counted nouns. It is `Copy` (just a language code), so there is no lifetime
//!   plumbing.
//!
//! Locale is chosen with this precedence: `--lang` flag → `language` config key →
//! the `LC_ALL`/`LC_MESSAGES`/`LANG`/`LANGUAGE` environment variables → English.

mod de;
mod es;
mod fr;
mod it;
mod pt;

use crate::Phase;

/// A localisation table: pairs of message id and translated text.
pub type Table = &'static [(Msg, &'static str)];

/// The locales coffeebreak ships, as `(code, native name)`, English first.
pub const LANGUAGES: &[(&str, &str)] = &[
    ("en", "English"),
    ("de", "Deutsch"),
    ("es", "Español"),
    ("fr", "Français"),
    ("it", "Italiano"),
    ("pt", "Português"),
];

/// The default locale code.
pub const DEFAULT_LANG: &str = "en";

/// All supported locale codes (for CLI validation).
pub const LANG_CODES: &[&str] = &["en", "de", "es", "fr", "it", "pt"];

/// Every user-facing message, identified at compile time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Msg {
    // Phase labels (status line / plain mode).
    PhaseFocus,
    PhaseShortBreak,
    PhaseLongBreak,
    // Phase announcements (desktop notifications).
    AnnounceFocus,
    AnnounceShortBreak,
    AnnounceLongBreak,

    // Pluralised nouns: each has a one/other form.
    PomodoroOne,
    PomodoroOther,
    CycleOne,
    CycleOther,
    DayOne,
    DayOther,

    // Live UI.
    ControlsHint,
    Paused,
    Left,
    CycleOf,

    // Footers / run summaries.
    DoneFooter,
    StoppedFooter,
    CelebrateMsg,
    PlanSummary,

    // Statistics.
    StatsTitle,
    StatsEmpty,
    StatsToday,
    StatsAllTime,
    StatsStreak,
    StatsBestDay,
    StatsLongestStreak,
    StatsGoal,
    StatsLast14,
    StatsHeatmap,
    HeatLess,
    HeatMore,
    GoalShort,
    GoalReached,
    MinFocus,
    Over,

    // doctor command.
    DoctorTitle,
    DoctorTerminal,
    DoctorTtyYes,
    DoctorTtyNo,
    DoctorColor,
    DoctorColorYes,
    DoctorColorNo,
    DoctorLang,
    DoctorConfig,
    DoctorConfigExists,
    DoctorConfigMissing,
    DoctorData,
    DoctorDataOk,
    DoctorDataNo,
    DoctorNotify,
    DoctorNotifyYes,
    DoctorNotifyNo,
    DoctorSound,
    DoctorSoundChime,
    DoctorSoundBell,

    // themes / presets / languages commands.
    ThemesTitle,
    ThemesHint,
    PresetsTitle,
    PresetsHint,
    PresetCadence,
    PresetLong,
    LanguagesTitle,
    LanguagesHint,

    // config command.
    ConfigCreated,
    ConfigExists,

    // self update / uninstall.
    UpdateCurrent,
    UpdateSource,
    UpdateUpToDate,
    UpdateNewer,
    UpdateRunHint,
    UpdateDone,
    UpdateAlready,
    UninstallIntro,
    UninstallItemBinary,
    UninstallItemConfig,
    UninstallItemData,
    UninstallConfirm,
    UninstallAborted,
    UninstallRemoved,
    UninstallBinFail,
    UninstallDone,
    ConfirmYesNo,
    ConfirmAffirmative,
    NotATerminal,

    // errors / warnings.
    WordError,
    WarnStatsSave,
    WarnStatsRead,
    WarnCtrlc,

    // CLI help — top level.
    HelpAbout,
    HelpLongAbout,
    HelpAfter,
    // CLI help — subcommands.
    HelpStats,
    HelpConfig,
    HelpThemes,
    HelpPresets,
    HelpLanguages,
    HelpDoctor,
    HelpCompletions,
    HelpMan,
    HelpSelf,
    HelpSelfUpdate,
    HelpSelfUninstall,
    HelpConfigInit,
    HelpConfigPath,
    HelpConfigShow,
    HelpUpdateCheck,
    HelpUninstallYes,
    HelpCompletionsShell,
    // CLI help — arguments.
    HelpWork,
    HelpBreak,
    HelpCycles,
    HelpGoal,
    HelpPreset,
    HelpLong,
    HelpLongBreak,
    HelpLongEvery,
    HelpLabel,
    HelpGitLabel,
    HelpTheme,
    HelpFps,
    HelpPlain,
    HelpNoColor,
    HelpNoSound,
    HelpNoNotify,
    HelpStatsFlag,
    HelpLang,
}

impl Msg {
    /// The canonical English text. Exhaustive by design.
    pub fn en(self) -> &'static str {
        use Msg::*;
        match self {
            PhaseFocus => "FOCUS",
            PhaseShortBreak => "BREAK",
            PhaseLongBreak => "LONG BREAK",
            AnnounceFocus => "Time to focus.",
            AnnounceShortBreak => "Short break — step away from the keyboard.",
            AnnounceLongBreak => "Long break — you earned it.",

            PomodoroOne => "pomodoro",
            PomodoroOther => "pomodoros",
            CycleOne => "cycle",
            CycleOther => "cycles",
            DayOne => "day",
            DayOther => "days",

            ControlsHint => "space pause · s skip · +/- adjust · q quit",
            Paused => "PAUSED",
            Left => "left",
            CycleOf => "cycle {n} of {total}",

            DoneFooter => "Done! {count} completed. ☕",
            StoppedFooter => "Stopped — {count} completed this session.",
            CelebrateMsg => "Session complete — {count} done!",
            PlanSummary => "{count} · focus {work} / break {brk}",

            StatsTitle => "☕ coffeebreak — statistics",
            StatsEmpty => "No pomodoros completed yet — run `coffeebreak` to start! ☕",
            StatsToday => "Today:",
            StatsAllTime => "All time:",
            StatsStreak => "Current streak:",
            StatsBestDay => "Best day:",
            StatsLongestStreak => "Longest streak:",
            StatsGoal => "Daily goal:",
            StatsLast14 => "Last 14 days",
            StatsHeatmap => "Last 12 weeks",
            HeatLess => "less",
            HeatMore => "more",
            GoalShort => "goal",
            GoalReached => "goal reached!",
            MinFocus => "min focus",
            Over => "over",

            DoctorTitle => "☕ coffeebreak — diagnostics",
            DoctorTerminal => "Terminal",
            DoctorTtyYes => "interactive (animated UI)",
            DoctorTtyNo => "not a TTY (plain output)",
            DoctorColor => "Truecolour",
            DoctorColorYes => "supported",
            DoctorColorNo => "not detected (set COLORTERM=truecolor)",
            DoctorLang => "Language",
            DoctorConfig => "Config file",
            DoctorConfigExists => "present",
            DoctorConfigMissing => "not created (run: coffeebreak config init)",
            DoctorData => "Data directory",
            DoctorDataOk => "writable",
            DoctorDataNo => "not writable",
            DoctorNotify => "Notifications",
            DoctorNotifyYes => "available",
            DoctorNotifyNo => "no notification service detected",
            DoctorSound => "Sound",
            DoctorSoundChime => "rodio chime (sound feature)",
            DoctorSoundBell => "terminal bell",

            ThemesTitle => "Available themes:",
            ThemesHint => "Use one with: coffeebreak --theme <name>",
            PresetsTitle => "Available presets:",
            PresetsHint => "Use one with: coffeebreak --preset <name>",
            PresetCadence => "{work} min focus / {brk} min break · {count}",
            PresetLong => ", long break {long} min every {every}",
            LanguagesTitle => "Available languages:",
            LanguagesHint => "Use one with: coffeebreak --lang <code>",

            ConfigCreated => "Created config:",
            ConfigExists => "Config already exists at",

            UpdateCurrent => "Current version: {version}",
            UpdateSource => "Source: {url}",
            UpdateUpToDate => "coffeebreak {version} is up to date.",
            UpdateNewer => "A newer version is available: {current} -> {latest}",
            UpdateRunHint => "Run `coffeebreak self update` to upgrade.",
            UpdateDone => "✓ Updated to {version}.",
            UpdateAlready => "Already up to date ({version}).",
            UninstallIntro => "This will remove coffeebreak and its data:",
            UninstallItemBinary => "binary",
            UninstallItemConfig => "config",
            UninstallItemData => "data",
            UninstallConfirm => "Remove all of the above?",
            UninstallAborted => "Aborted. Nothing was removed.",
            UninstallRemoved => "✓ Removed {path}",
            UninstallBinFail => "Could not remove the binary automatically ({error}).",
            UninstallDone => "coffeebreak uninstalled. ☕ Thanks for the focus sessions!",
            ConfirmYesNo => "[y/N]",
            // The accepted affirmative key, matching the letter shown in ConfirmYesNo.
            ConfirmAffirmative => "y",
            NotATerminal => "not a terminal; re-run with --yes to confirm non-interactively",

            WordError => "error",
            WarnStatsSave => "could not save stats ({error})",
            WarnStatsRead => "ignoring unreadable stats ({error})",
            WarnCtrlc => {
                "could not install Ctrl+C handler ({error}); stats may not be saved if you interrupt the session"
            }

            HelpAbout => "A Pomodoro focus timer for your terminal ☕",
            HelpLongAbout => {
                "coffeebreak runs Pomodoro focus/break cycles with a live, animated coffee cup whose \
                 steam and fill track the time, large countdown digits, a gradient progress bar, \
                 desktop notifications, and a developer quote at each break."
            }
            HelpAfter => {
                "Examples:\n  \
                 coffeebreak                         Classic 25/5, one cycle\n  \
                 coffeebreak --preset classic        Four 25/5 rounds, ending on a long break\n  \
                 coffeebreak -w 50 -b 10 --cycles 3  Deep work: three 50/10 rounds\n  \
                 coffeebreak --theme ocean           Use the ocean colour theme\n  \
                 coffeebreak --lang de               Run in German\n  \
                 coffeebreak --stats                 Show your focus statistics\n\n\
                 During a session:\n  \
                 space / p   pause or resume        s / n   skip the current phase\n  \
                 + / =       add a minute           - / _   remove a minute\n  \
                 q / Esc     quit (stats are saved)"
            }
            HelpStats => "Show focus statistics (today, all-time, streak, best day)",
            HelpConfig => "Inspect or create the configuration file",
            HelpThemes => "List the available colour themes with a preview",
            HelpPresets => "List the available timer presets",
            HelpLanguages => "List the available interface languages",
            HelpDoctor => "Run environment diagnostics (terminal, locale, config, …)",
            HelpCompletions => "Generate a shell completion script (bash, zsh, fish, …)",
            HelpMan => "Print a roff man page to stdout",
            HelpSelf => "Manage the installed coffeebreak binary (update / uninstall)",
            HelpSelfUpdate => "Update coffeebreak to the latest GitHub release",
            HelpSelfUninstall => "Remove the coffeebreak binary and its config/data directories",
            HelpConfigInit => "Write a default config file (does nothing if one already exists)",
            HelpConfigPath => "Print the path to the config file",
            HelpConfigShow => "Print the effective configuration",
            HelpUpdateCheck => "Only check whether a newer version exists; do not install",
            HelpUninstallYes => "Skip the confirmation prompt",
            HelpCompletionsShell => "The shell to generate completions for",
            HelpWork => "Focus block length in minutes (default 25)",
            HelpBreak => "Break length in minutes (default 5)",
            HelpCycles => "Number of focus→break cycles to run (default 1)",
            HelpGoal => "Daily pomodoro goal shown in stats (0 = off)",
            HelpPreset => "Start from a named preset: classic, deep, short, sprint",
            HelpLong => "Enable a long break after every N focus blocks",
            HelpLongBreak => "Long break length in minutes (implies --long; default 15)",
            HelpLongEvery => "How many focus blocks before a long break (default 4)",
            HelpLabel => "Optional label for this session (shown in the status line)",
            HelpGitLabel => "Use the current git branch as the session label",
            HelpTheme => "Colour theme: coffee, ocean, forest, grape, mono",
            HelpFps => "Animation frames per second (2–60; default 15)",
            HelpPlain => "Plain, non-animated line output (also used automatically when piped)",
            HelpNoColor => "Disable coloured output",
            HelpNoSound => "Silence the audible cue on phase change",
            HelpNoNotify => "Do not send desktop notifications",
            HelpStatsFlag => "Show today's and all-time statistics, then exit",
            HelpLang => "Interface language: en, de, es, fr, it, pt",
        }
    }
}

/// The active-locale resolver threaded through the program.
#[derive(Debug, Clone, Copy)]
pub struct I18n {
    lang: &'static str,
}

impl I18n {
    /// Construct for a locale code, normalising and falling back to English for
    /// anything unsupported (e.g. `"de_DE.UTF-8"` → `"de"`, `"xx"` → `"en"`).
    pub fn new(code: &str) -> I18n {
        I18n { lang: normalize(code) }
    }

    /// Resolve the locale from (in precedence order) the CLI flag, the config
    /// value, the environment, then English.
    pub fn detect(cli: Option<&str>, config: Option<&str>) -> I18n {
        if let Some(c) = cli.filter(|c| !c.is_empty()) {
            return I18n::new(c);
        }
        if let Some(c) = config.filter(|c| !c.is_empty()) {
            return I18n::new(c);
        }
        for var in ["LC_ALL", "LC_MESSAGES", "LANG", "LANGUAGE"] {
            if let Some(val) = std::env::var_os(var) {
                let val = val.to_string_lossy();
                if let Some(code) = val.split(':').next() {
                    let norm = normalize(code);
                    if norm != DEFAULT_LANG || is_supported(code) {
                        return I18n { lang: norm };
                    }
                }
            }
        }
        I18n::new(DEFAULT_LANG)
    }

    /// The active locale code (e.g. `"en"`).
    pub fn code(&self) -> &'static str {
        self.lang
    }

    /// The active locale's native name (e.g. `"Deutsch"`).
    pub fn name(&self) -> &'static str {
        LANGUAGES
            .iter()
            .find(|(c, _)| *c == self.lang)
            .map(|(_, n)| *n)
            .unwrap_or("English")
    }

    /// Resolve a message in the active locale, falling back to English.
    pub fn t(&self, msg: Msg) -> &'static str {
        if self.lang == DEFAULT_LANG {
            return msg.en();
        }
        table(self.lang)
            .iter()
            .find(|(k, _)| *k == msg)
            .map(|(_, v)| *v)
            .unwrap_or_else(|| msg.en())
    }

    /// Resolve a message and substitute every `{name}` placeholder from `args`.
    pub fn tf(&self, msg: Msg, args: &[(&str, &str)]) -> String {
        let mut s = self.t(msg).to_string();
        for (key, val) in args {
            s = s.replace(&format!("{{{key}}}"), val);
        }
        s
    }

    /// Pluralise a counted noun, e.g. `count(1, Noun::Pomodoro)` → `"1 pomodoro"`,
    /// `count(3, …)` → `"3 pomodoros"`.
    pub fn count(&self, n: u64, noun: Noun) -> String {
        let (one, other) = noun.keys();
        let word = self.t(if n == 1 { one } else { other });
        format!("{n} {word}")
    }

    /// The localised status label for a phase.
    pub fn phase_label(&self, phase: Phase) -> &'static str {
        self.t(match phase {
            Phase::Focus => Msg::PhaseFocus,
            Phase::ShortBreak => Msg::PhaseShortBreak,
            Phase::LongBreak => Msg::PhaseLongBreak,
        })
    }

    /// The localised notification sentence for entering a phase.
    pub fn phase_announce(&self, phase: Phase) -> &'static str {
        self.t(match phase {
            Phase::Focus => Msg::AnnounceFocus,
            Phase::ShortBreak => Msg::AnnounceShortBreak,
            Phase::LongBreak => Msg::AnnounceLongBreak,
        })
    }
}

/// A noun with singular/plural message forms.
#[derive(Debug, Clone, Copy)]
pub enum Noun {
    Pomodoro,
    Cycle,
    Day,
}

impl Noun {
    fn keys(self) -> (Msg, Msg) {
        match self {
            Noun::Pomodoro => (Msg::PomodoroOne, Msg::PomodoroOther),
            Noun::Cycle => (Msg::CycleOne, Msg::CycleOther),
            Noun::Day => (Msg::DayOne, Msg::DayOther),
        }
    }
}

/// The translation table for a locale code, or an empty table (→ English).
fn table(code: &str) -> Table {
    match code {
        "de" => de::ENTRIES,
        "es" => es::ENTRIES,
        "fr" => fr::ENTRIES,
        "it" => it::ENTRIES,
        "pt" => pt::ENTRIES,
        _ => &[],
    }
}

/// Whether a (possibly region-tagged) code maps to a shipped locale.
fn is_supported(code: &str) -> bool {
    let base = base_code(code);
    LANG_CODES.contains(&base.as_str())
}

/// Normalise an arbitrary locale string to a supported code, or English.
fn normalize(code: &str) -> &'static str {
    let base = base_code(code);
    LANG_CODES.iter().copied().find(|c| *c == base).unwrap_or(DEFAULT_LANG)
}

/// Extract the base language from a locale string: `"de_DE.UTF-8"` → `"de"`.
fn base_code(code: &str) -> String {
    code.split(['_', '-', '.', '@'])
        .next()
        .unwrap_or("")
        .to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn english_is_the_default_and_always_present() {
        let i = I18n::new("en");
        assert_eq!(i.code(), "en");
        assert_eq!(i.t(Msg::PhaseFocus), "FOCUS");
    }

    #[test]
    fn unsupported_falls_back_to_english() {
        assert_eq!(I18n::new("xx").code(), "en");
        assert_eq!(I18n::new("de_DE.UTF-8").code(), "de");
        assert_eq!(I18n::new("PT-br").code(), "pt");
    }

    #[test]
    fn interpolation_replaces_named_args() {
        let i = I18n::new("en");
        assert_eq!(i.tf(Msg::CycleOf, &[("n", "2"), ("total", "4")]), "cycle 2 of 4");
    }

    #[test]
    fn plurals_pick_the_right_form() {
        let i = I18n::new("en");
        assert_eq!(i.count(1, Noun::Pomodoro), "1 pomodoro");
        assert_eq!(i.count(3, Noun::Pomodoro), "3 pomodoros");
        assert_eq!(i.count(1, Noun::Day), "1 day");
    }

    #[test]
    fn detect_precedence_cli_over_config_over_env() {
        assert_eq!(I18n::detect(Some("fr"), Some("de")).code(), "fr");
        assert_eq!(I18n::detect(None, Some("de")).code(), "de");
        assert_eq!(I18n::detect(Some(""), Some("it")).code(), "it");
    }

    #[test]
    fn every_translation_key_exists_in_english() {
        // Sanity: en() is exhaustive (compiles), and each locale table only uses
        // valid Msg values (type-checked). Here we just ensure tables resolve.
        for (code, _) in LANGUAGES {
            let i = I18n::new(code);
            let _ = i.t(Msg::PhaseFocus);
            let _ = i.t(Msg::DoneFooter);
        }
    }

    #[test]
    fn locale_tables_are_clean() {
        // No locale table may contain a duplicate key or an empty translation;
        // either would silently shadow or blank a message.
        let tables: &[(&str, Table)] = &[
            ("de", de::ENTRIES),
            ("es", es::ENTRIES),
            ("fr", fr::ENTRIES),
            ("it", it::ENTRIES),
            ("pt", pt::ENTRIES),
        ];
        for (code, table) in tables {
            let mut seen = std::collections::HashSet::new();
            for (key, value) in *table {
                assert!(!value.trim().is_empty(), "{code}: empty translation for {key:?}");
                assert!(seen.insert(format!("{key:?}")), "{code}: duplicate key {key:?}");
            }
        }
    }

    #[test]
    fn placeholder_tokens_survive_translation() {
        // A counted footer must keep its {count} slot in every locale.
        for (code, _) in LANGUAGES {
            let i = I18n::new(code);
            let out = i.tf(Msg::DoneFooter, &[("count", "3 x")]);
            assert!(out.contains("3 x"), "{code}: DoneFooter dropped its placeholder");
        }
    }
}
