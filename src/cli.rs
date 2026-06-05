//! Command-line interface (clap derive).
//!
//! Running `coffeebreak` with no subcommand starts a timer (or shows stats with
//! `--stats`). The `self` subcommand groups lifecycle operations.

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "coffeebreak",
    version,
    about = "A Pomodoro focus timer for your terminal ☕",
    long_about = "coffeebreak runs focus/break cycles with a live countdown, an \
                  ASCII coffee cup whose steam tracks the remaining time, desktop \
                  notifications on phase change, and a developer quote at each break."
)]
pub struct Cli {
    /// Focus block length in minutes (default 25).
    #[arg(short = 'w', long = "work", value_name = "MIN")]
    pub work: Option<u64>,

    /// Break length in minutes (default 5).
    #[arg(short = 'b', long = "break", value_name = "MIN")]
    pub brk: Option<u64>,

    /// Enable a long break after every N focus blocks.
    #[arg(long)]
    pub long: bool,

    /// Long break length in minutes (implies --long; default 15).
    #[arg(long = "long-break", value_name = "MIN")]
    pub long_break: Option<u64>,

    /// How many focus blocks before a long break (default 4).
    #[arg(long = "long-every", value_name = "N")]
    pub long_every: Option<u64>,

    /// Number of focus→break cycles to run (default 1).
    #[arg(long, value_name = "N")]
    pub cycles: Option<u64>,

    /// Optional label for this session (shown in the status line).
    #[arg(short = 'l', long, value_name = "TEXT")]
    pub label: Option<String>,

    /// Use the current git branch as the session label.
    #[arg(long = "git-label")]
    pub git_label: bool,

    /// Show today's and all-time statistics, then exit.
    #[arg(long)]
    pub stats: bool,

    /// Silence the audible cue on phase change.
    #[arg(long = "no-sound")]
    pub no_sound: bool,

    /// Do not send desktop notifications.
    #[arg(long = "no-notify")]
    pub no_notify: bool,

    /// Disable coloured output.
    #[arg(long = "no-color")]
    pub no_color: bool,

    /// Treat -w/-b/--long-break values as SECONDS instead of minutes.
    /// Intended for quick demos and tests.
    #[arg(long, hide = true)]
    pub seconds: bool,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Manage the installed coffeebreak binary (update / uninstall).
    #[command(name = "self", subcommand_help_heading = "Self commands")]
    Selfcmd {
        #[command(subcommand)]
        action: SelfAction,
    },
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
}
