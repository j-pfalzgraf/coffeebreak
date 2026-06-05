//! coffeebreak — a Pomodoro focus timer for your terminal.
//!
//! The binary is thin (see `main.rs`); all behaviour lives here so it can be
//! unit-tested. The codebase is organised into small, single-purpose modules:
//!
//! **Domain**
//! * [`cli`]     — argument parsing (clap derive), subcommands, styled help.
//! * [`config`]  — persisted defaults (`config.toml`).
//! * [`session`] — the resolved run plan: durations, presets, display prefs.
//! * [`stats`]   — daily statistics persisted to `~/.coffeebreak/stats.json`.
//! * [`quotes`]  — developer quotes shown when a break begins.
//! * [`paths`]   — XDG-aware locations for config and data.
//! * [`git`]     — best-effort current-branch detection for session labels.
//!
//! **Runtime / UI**
//! * [`app`]      — the orchestrator: animated TUI plus a plain fallback.
//! * [`clock`]    — [`clock::Clock`] and a pause-aware, testable phase timer.
//! * [`theme`]    — truecolour palettes and the one styling entry point.
//! * [`render`]   — a flicker-free, line-diffing frame renderer.
//! * [`widgets`]  — the coffee cup, big-digit clock, progress bar, confetti.
//! * [`term`]     — RAII alternate-screen / raw-mode terminal session.
//! * [`input`]    — keyboard controls (pause / skip / quit / adjust).
//! * [`feedback`] — [`feedback::Notifier`] / [`feedback::SoundPlayer`] backends.
//!
//! **Commands**
//! * [`commands`]    — `stats`, `config`, `themes`, `presets` handlers.
//! * [`completions`] — shell completions and the man page.
//! * [`selfcmd`]     — `self update` / `self uninstall` lifecycle commands.

pub mod app;
pub mod cli;
pub mod clock;
pub mod commands;
pub mod completions;
pub mod config;
pub mod feedback;
pub mod git;
pub mod input;
pub mod paths;
pub mod quotes;
pub mod render;
pub mod selfcmd;
pub mod session;
pub mod stats;
pub mod term;
pub mod theme;
pub mod widgets;

/// GitHub coordinates used by `self update` and the install scripts.
///
/// Swap these (and the `repository` field in `Cargo.toml` + the install
/// scripts) to point at your own fork.
pub const REPO_OWNER: &str = "j-pfalzgraf";
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
