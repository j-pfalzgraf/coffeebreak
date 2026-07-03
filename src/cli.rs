//! Command-line interface (clap derive).
//!
//! Running `coffeebreak` with no subcommand starts a timer. Subcommands group
//! everything else: `stats`, `config`, `themes`, `presets`, `completions`,
//! `man`, and `self`. Help is colourised and carries examples plus the
//! interactive key bindings.

use clap::builder::PossibleValuesParser;
use clap::builder::styling::{AnsiColor, Effects, Styles};
use clap::{CommandFactory, FromArgMatches, Parser, Subcommand, ValueEnum};
use clap_complete::Shell;

use crate::i18n::{I18n, LANG_CODES, Msg};
use crate::session::PRESET_NAMES;
use crate::theme::THEME_CHOICES;

/// Colour styling for help output.
fn help_styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Yellow.on_default().effects(Effects::BOLD))
        .usage(AnsiColor::Yellow.on_default().effects(Effects::BOLD))
        .literal(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
        .placeholder(AnsiColor::Green.on_default())
        .valid(AnsiColor::Green.on_default())
        .invalid(AnsiColor::Red.on_default())
        .error(AnsiColor::Red.on_default().effects(Effects::BOLD))
}

#[derive(Parser, Debug)]
#[command(
    name = "coffeebreak",
    version,
    about = "A Pomodoro focus timer for your terminal ☕",
    long_about = "coffeebreak runs Pomodoro focus/break cycles with a live, animated coffee \
                  cup whose steam and fill track the time, large countdown digits, a gradient \
                  progress bar, desktop notifications, and a developer quote at each break.",
    styles = help_styles(),
)]
pub struct Cli {
    // --- Timer options ------------------------------------------------------
    /// Focus block length in minutes (default 25).
    #[arg(
        short = 'w',
        long = "work",
        value_name = "MIN",
        help_heading = "Timer options"
    )]
    pub work: Option<u64>,

    /// Break length in minutes (default 5).
    #[arg(
        short = 'b',
        long = "break",
        value_name = "MIN",
        help_heading = "Timer options"
    )]
    pub brk: Option<u64>,

    /// Number of focus→break cycles to run (default 1).
    #[arg(long, value_name = "N", help_heading = "Timer options")]
    pub cycles: Option<u64>,

    /// Daily pomodoro goal shown in stats (0 = off).
    #[arg(long, value_name = "N", help_heading = "Timer options")]
    pub goal: Option<u64>,

    /// Wait for a keypress between phases instead of auto-advancing.
    #[arg(long, help_heading = "Timer options")]
    pub wait: bool,

    /// Start from a named preset: classic, deep, short, sprint.
    #[arg(
        long,
        value_name = "NAME",
        ignore_case = true,
        value_parser = PossibleValuesParser::new(PRESET_NAMES),
        help_heading = "Timer options"
    )]
    pub preset: Option<String>,

    /// Enable a long break after every N focus blocks.
    #[arg(long, help_heading = "Timer options")]
    pub long: bool,

    /// Long break length in minutes (implies --long; default 15).
    #[arg(
        long = "long-break",
        value_name = "MIN",
        help_heading = "Timer options"
    )]
    pub long_break: Option<u64>,

    /// How many focus blocks before a long break (default 4).
    #[arg(long = "long-every", value_name = "N", help_heading = "Timer options")]
    pub long_every: Option<u64>,

    // --- Session metadata ---------------------------------------------------
    /// Optional label for this session (shown in the status line).
    #[arg(short = 'l', long, value_name = "TEXT", help_heading = "Session")]
    pub label: Option<String>,

    /// Use the current git branch as the session label.
    #[arg(long = "git-label", help_heading = "Session")]
    pub git_label: bool,

    // --- Display options ----------------------------------------------------
    /// Colour theme — see `coffeebreak themes` for previews.
    #[arg(
        long,
        global = true,
        value_name = "NAME",
        ignore_case = true,
        value_parser = PossibleValuesParser::new(THEME_CHOICES),
        help_heading = "Display"
    )]
    pub theme: Option<String>,

    /// Animation frames per second (2–60; default 15).
    #[arg(long, value_name = "FPS", help_heading = "Display")]
    pub fps: Option<u32>,

    /// Big countdown style: digits (default) or ring.
    #[arg(
        long,
        value_enum,
        ignore_case = true,
        value_name = "STYLE",
        help_heading = "Display"
    )]
    pub indicator: Option<Indicator>,

    /// Play the brewing intro animation before the first focus block.
    #[arg(long, help_heading = "Display")]
    pub brew: bool,

    /// Interface language: en, de, es, fr, it, pt.
    #[arg(
        long,
        global = true,
        value_name = "CODE",
        ignore_case = true,
        value_parser = PossibleValuesParser::new(LANG_CODES),
        help_heading = "Display"
    )]
    pub lang: Option<String>,

    /// Plain, non-animated line output (also used automatically when piped).
    #[arg(long, help_heading = "Display")]
    pub plain: bool,

    /// Disable coloured output.
    #[arg(long = "no-color", global = true, help_heading = "Display")]
    pub no_color: bool,

    // --- Feedback -----------------------------------------------------------
    /// Silence the audible cue on phase change.
    #[arg(long = "no-sound", help_heading = "Feedback")]
    pub no_sound: bool,

    /// Do not send desktop notifications.
    #[arg(long = "no-notify", help_heading = "Feedback")]
    pub no_notify: bool,

    // --- Shortcuts ----------------------------------------------------------
    /// Show today's and all-time statistics, then exit.
    #[arg(long, help_heading = "Shortcuts")]
    pub stats: bool,

    /// Treat -w/-b/--long-break values as SECONDS instead of minutes (for demos).
    #[arg(long, hide = true)]
    pub seconds: bool,

    #[command(subcommand)]
    pub command: Option<Command>,
}

/// The big-countdown indicator style for the live timer.
#[derive(ValueEnum, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Indicator {
    /// Large block digits counting down `MM:SS` (default).
    #[default]
    Digits,
    /// A circular gauge that fills as the phase elapses, with a percentage.
    Ring,
}

impl Indicator {
    /// Parse from a config string, defaulting to [`Indicator::Digits`] for an
    /// empty or unrecognised value.
    pub fn parse(s: &str) -> Indicator {
        match s.trim().to_ascii_lowercase().as_str() {
            "ring" => Indicator::Ring,
            _ => Indicator::Digits,
        }
    }
}

/// Output format for `coffeebreak stats`.
#[derive(ValueEnum, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StatsFormat {
    /// Human-readable animated dashboard (default).
    #[default]
    Text,
    /// Machine-readable JSON (summary + per-day history).
    Json,
    /// Comma-separated values: date, completed_pomodoros, focus_minutes.
    Csv,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Show focus statistics (today, all-time, streak, best day).
    Stats {
        /// Output format: text (dashboard), json, or csv.
        #[arg(long, value_enum, default_value_t = StatsFormat::Text)]
        format: StatsFormat,
    },

    /// Show your earned badges and progress toward the next.
    Achievements,

    /// Inspect or create the configuration file.
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },

    /// List the available colour themes with a preview.
    Themes,

    /// List the available timer presets.
    Presets,

    /// List the available interface languages.
    Languages,

    /// Showcase every widget and animation, then exit.
    Demo,

    /// Run environment diagnostics (terminal, locale, config, …).
    Doctor,

    /// Generate a shell completion script (bash, zsh, fish, …).
    Completions {
        /// The shell to generate completions for.
        shell: Shell,
    },

    /// Print a roff man page to stdout.
    Man,

    /// Manage the installed coffeebreak binary (update / uninstall).
    #[command(name = "self")]
    Selfcmd {
        #[command(subcommand)]
        action: SelfAction,
    },
}

#[derive(Subcommand, Debug)]
pub enum ConfigAction {
    /// Write a default config file (does nothing if one already exists).
    Init,
    /// Print the path to the config file.
    Path,
    /// Print the effective configuration.
    Show,
}

#[derive(Subcommand, Debug)]
pub enum SelfAction {
    /// Update coffeebreak to the latest GitHub release.
    Update {
        /// Only check whether a newer version exists; do not install.
        #[arg(long)]
        check: bool,
    },
    /// Remove the coffeebreak binary and its config/data directories.
    Uninstall {
        /// Skip the confirmation prompt.
        #[arg(short = 'y', long)]
        yes: bool,
    },
}

impl Cli {
    /// Parse from the process arguments with help/usage text localised via `i18n`.
    ///
    /// `--help`/`--version`/parse errors are handled by clap (it prints and
    /// exits); on success the parsed [`Cli`] is returned.
    pub fn parse_localized(i18n: &I18n) -> Cli {
        let matches = localized_command(i18n).get_matches();
        match Cli::from_arg_matches(&matches) {
            Ok(cli) => cli,
            Err(e) => e.exit(),
        }
    }

    /// Build the clap `Command` (used for completions and the man page).
    ///
    /// The examples/key-bindings epilogue comes from the canonical English
    /// [`Msg::HelpAfter`] — the same text `--help` shows — so the man page and
    /// the runtime help can never drift apart.
    pub fn command() -> clap::Command {
        <Cli as CommandFactory>::command()
            .after_help(Msg::HelpAfter.en())
            .after_long_help(Msg::HelpAfter.en())
    }
}

/// Apply localised help text to the derived clap command.
///
/// English remains the canonical text baked in by the derive; this overrides the
/// user-visible strings for the active locale (a no-op effect for English).
fn localized_command(i18n: &I18n) -> clap::Command {
    let t = |m: Msg| i18n.t(m).to_string();

    let mut cmd = Cli::command()
        .about(t(Msg::HelpAbout))
        .long_about(t(Msg::HelpLongAbout))
        .after_help(t(Msg::HelpAfter))
        .after_long_help(t(Msg::HelpAfter));

    // Top-level argument help.
    let args: &[(&str, Msg)] = &[
        ("work", Msg::HelpWork),
        ("brk", Msg::HelpBreak),
        ("cycles", Msg::HelpCycles),
        ("goal", Msg::HelpGoal),
        ("wait", Msg::HelpWait),
        ("preset", Msg::HelpPreset),
        ("long", Msg::HelpLong),
        ("long_break", Msg::HelpLongBreak),
        ("long_every", Msg::HelpLongEvery),
        ("label", Msg::HelpLabel),
        ("git_label", Msg::HelpGitLabel),
        ("theme", Msg::HelpTheme),
        ("fps", Msg::HelpFps),
        ("indicator", Msg::HelpIndicator),
        ("brew", Msg::HelpBrew),
        ("lang", Msg::HelpLang),
        ("plain", Msg::HelpPlain),
        ("no_color", Msg::HelpNoColor),
        ("no_sound", Msg::HelpNoSound),
        ("no_notify", Msg::HelpNoNotify),
        ("stats", Msg::HelpStatsFlag),
    ];
    for (id, msg) in args {
        cmd = cmd.mut_arg(*id, |a| a.help(t(*msg)));
    }

    // Subcommand descriptions (and their own arguments/subcommands).
    let subs: &[(&str, Msg)] = &[
        ("achievements", Msg::HelpAchievements),
        ("themes", Msg::HelpThemes),
        ("presets", Msg::HelpPresets),
        ("languages", Msg::HelpLanguages),
        ("demo", Msg::HelpDemo),
        ("doctor", Msg::HelpDoctor),
        ("man", Msg::HelpMan),
    ];
    for (name, msg) in subs {
        cmd = cmd.mut_subcommand(*name, |c| c.about(t(*msg)));
    }
    cmd = cmd.mut_subcommand("stats", |c| {
        c.about(t(Msg::HelpStats))
            .mut_arg("format", |a| a.help(t(Msg::HelpFormat)))
    });
    cmd = cmd.mut_subcommand("completions", |c| {
        c.about(t(Msg::HelpCompletions))
            .mut_arg("shell", |a| a.help(t(Msg::HelpCompletionsShell)))
    });
    cmd = cmd.mut_subcommand("config", |c| {
        c.about(t(Msg::HelpConfig))
            .mut_subcommand("init", |s| s.about(t(Msg::HelpConfigInit)))
            .mut_subcommand("path", |s| s.about(t(Msg::HelpConfigPath)))
            .mut_subcommand("show", |s| s.about(t(Msg::HelpConfigShow)))
    });
    cmd = cmd.mut_subcommand("self", |c| {
        c.about(t(Msg::HelpSelf))
            .mut_subcommand("update", |s| {
                s.about(t(Msg::HelpSelfUpdate))
                    .mut_arg("check", |a| a.help(t(Msg::HelpUpdateCheck)))
            })
            .mut_subcommand("uninstall", |s| {
                s.about(t(Msg::HelpSelfUninstall))
                    .mut_arg("yes", |a| a.help(t(Msg::HelpUninstallYes)))
            })
    });

    cmd
}
