//! coffeebreak — a Pomodoro focus timer for your terminal.
//!
//! The binary is thin (see `main.rs`); all behaviour lives here so it can be
//! unit-tested. Modules are deliberately small and loosely coupled:
//!
//! * [`cli`]     — argument parsing (clap derive) and subcommands.
//! * [`config`]  — persisted defaults (`config.toml`) merged with CLI flags.
//! * [`session`] — the resolved, ready-to-run plan (durations, cycles, flags).
//! * [`timer`]   — the core countdown loop, progress bar and live rendering.
//! * [`art`]     — the ASCII coffee cup whose steam tracks the remaining time.
//! * [`quotes`]  — developer quotes shown when a break begins.
//! * [`notify`]  — desktop notification + bell/sound on phase change.
//! * [`stats`]   — daily statistics persisted to `~/.coffeebreak/stats.json`.
//! * [`paths`]   — XDG-aware locations for config and data.
//! * [`git`]     — best-effort current-branch detection for session labels.
//! * [`selfcmd`] — `self update` / `self uninstall` lifecycle commands.

pub mod art;
pub mod cli;
pub mod config;
pub mod git;
pub mod notify;
pub mod paths;
pub mod quotes;
pub mod selfcmd;
pub mod session;
pub mod stats;
pub mod timer;

/// GitHub coordinates used by `self update` and the install scripts.
///
/// Swap these (and the `repository` field in `Cargo.toml` + the install
/// scripts) to point at your own fork.
pub const REPO_OWNER: &str = "leuchtturm";
pub const REPO_NAME: &str = "coffeebreak";
/// The installed command / release asset name (distinct from the crate name
/// `coffeebreak-cli`).
pub const BIN_NAME: &str = "coffeebreak";

/// One segment of a Pomodoro session.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    /// Heads-down work.
    Focus,
    /// The short breather after a focus block.
    ShortBreak,
    /// The longer breather after every Nth focus block.
    LongBreak,
}

impl Phase {
    /// Whether this phase counts as work (used for stats and notifications).
    pub fn is_focus(self) -> bool {
        matches!(self, Phase::Focus)
    }

    /// Short uppercase label for the status line, e.g. `FOCUS`.
    pub fn label(self) -> &'static str {
        match self {
            Phase::Focus => "FOCUS",
            Phase::ShortBreak => "BREAK",
            Phase::LongBreak => "LONG BREAK",
        }
    }

    /// Human sentence used in desktop notifications.
    pub fn announce(self) -> &'static str {
        match self {
            Phase::Focus => "Time to focus. ☕",
            Phase::ShortBreak => "Short break — step away from the keyboard.",
            Phase::LongBreak => "Long break — you earned it. 🎉",
        }
    }
}
