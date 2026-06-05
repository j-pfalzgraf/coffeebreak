//! Command-line interface (clap derive).
//!
//! Running `coffeebreak` with no subcommand starts a timer. Subcommands group
//! everything else: `stats`, `config`, `themes`, `presets`, `completions`,
//! `man`, and `self`. Help is colourised and carries examples plus the
//! interactive key bindings.

use clap::builder::PossibleValuesParser;
use clap::builder::styling::{AnsiColor, Effects, Styles};
use clap::{Parser, Subcommand};
use clap_complete::Shell;

use crate::session::PRESET_NAMES;
use crate::theme::THEME_NAMES;

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

const AFTER_HELP: &str = "\x1b[1;33mExamples:\x1b[0m\n  \
    coffeebreak                         Classic 25/5, one cycle\n  \
    coffeebreak --preset classic        Four 25/5 rounds with a long break\n  \
    coffeebreak -w 50 -b 10 --cycles 3  Deep work: three 50/10 rounds\n  \
    coffeebreak --theme ocean           Use the ocean colour theme\n  \
    coffeebreak --stats                 Show your focus statistics\n\n\
    \x1b[1;33mDuring a session:\x1b[0m\n  \
    space / p   pause or resume        s / n   skip the current phase\n  \
    + / =       add a minute           - / _   remove a minute\n  \
    q / Esc     quit (stats are saved)\n";

#[derive(Parser, Debug)]
#[command(
    name = "coffeebreak",
    version,
    about = "A Pomodoro focus timer for your terminal ☕",
    long_about = "coffeebreak runs Pomodoro focus/break cycles with a live, animated coffee \
                  cup whose steam and fill track the time, large countdown digits, a gradient \
                  progress bar, desktop notifications, and a developer quote at each break.",
    styles = help_styles(),
    after_help = AFTER_HELP,
    after_long_help = AFTER_HELP,
)]
pub struct Cli {
    // --- Timer options ------------------------------------------------------
    /// Focus block length in minutes (default 25).
    #[arg(short = 'w', long = "work", value_name = "MIN", help_heading = "Timer options")]
    pub work: Option<u64>,

    /// Break length in minutes (default 5).
    #[arg(short = 'b', long = "break", value_name = "MIN", help_heading = "Timer options")]
    pub brk: Option<u64>,

    /// Number of focus→break cycles to run (default 1).
    #[arg(long, value_name = "N", help_heading = "Timer options")]
    pub cycles: Option<u64>,

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
    #[arg(long = "long-break", value_name = "MIN", help_heading = "Timer options")]
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
    /// Colour theme: coffee, ocean, forest, grape, mono.
    #[arg(
        long,
        global = true,
        value_name = "NAME",
        ignore_case = true,
        value_parser = PossibleValuesParser::new(THEME_NAMES),
        help_heading = "Display"
    )]
    pub theme: Option<String>,

    /// Animation frames per second (2–60; default 15).
    #[arg(long, value_name = "FPS", help_heading = "Display")]
    pub fps: Option<u32>,

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

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Show focus statistics (today, all-time, streak, best day).
    Stats,

    /// Inspect or create the configuration file.
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },

    /// List the available colour themes with a preview.
    Themes,

    /// List the available timer presets.
    Presets,

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
    /// Parse from the process arguments.
    pub fn parse_args() -> Cli {
        Cli::parse()
    }

    /// Build the clap `Command` (used for completions and the man page).
    pub fn command() -> clap::Command {
        <Cli as clap::CommandFactory>::command()
    }
}
