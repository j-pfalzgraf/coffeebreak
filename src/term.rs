//! RAII terminal setup/teardown for the animated UI.
//!
//! Entering the timer switches to the alternate screen, enables raw mode (so we
//! can read single keypresses for pause/skip/quit), and hides the cursor. The
//! [`TerminalSession`] guard restores all of that on drop — on normal exit, on
//! `?` error propagation, and (because we also install a panic hook in `main`)
//! on panic. Leaving the alternate screen also wipes the animation, keeping the
//! user's scrollback clean.

use std::io::{self, IsTerminal, Write};

use anyhow::{Context, Result};
use crossterm::{cursor, execute, terminal};

/// Active alternate-screen + raw-mode session. Restores the terminal on drop.
pub struct TerminalSession;

impl TerminalSession {
    /// Enter the alternate screen, enable raw mode, and hide the cursor.
    pub fn enter() -> Result<TerminalSession> {
        terminal::enable_raw_mode().context("failed to enable raw mode")?;
        let mut out = io::stdout();
        // If entering the alternate screen fails, raw mode is already on and the
        // `TerminalSession` guard is never constructed — so undo raw mode here
        // rather than leave the user's shell wedged (no echo, no line editing).
        if let Err(e) = execute!(out, terminal::EnterAlternateScreen, cursor::Hide) {
            let _ = terminal::disable_raw_mode();
            return Err(e).context("failed to enter alternate screen");
        }
        Ok(TerminalSession)
    }

    /// Current terminal size as `(cols, rows)`, with a sane fallback.
    pub fn size() -> (u16, u16) {
        terminal::size().unwrap_or((80, 24))
    }

    /// Best-effort manual restore (also run on drop).
    pub fn restore() {
        let mut out = io::stdout();
        let _ = execute!(out, cursor::Show, terminal::LeaveAlternateScreen);
        let _ = terminal::disable_raw_mode();
        let _ = out.flush();
    }
}

impl Drop for TerminalSession {
    fn drop(&mut self) {
        TerminalSession::restore();
    }
}

/// Whether both stdout and stdin are real terminals (required for the animated,
/// interactive UI). When false, the caller falls back to plain line output.
pub fn is_interactive() -> bool {
    io::stdout().is_terminal() && io::stdin().is_terminal()
}

/// The single colour-output decision, shared by every command path.
///
/// Colour is on only when all of these hold: the user didn't pass `--no-color`,
/// the `NO_COLOR` environment variable is unset **or empty** (per the
/// <https://no-color.org> spec, an empty value does not disable colour), and
/// stdout is a real terminal.
pub fn color_enabled(no_color_flag: bool) -> bool {
    if no_color_flag {
        return false;
    }
    let no_color_env = std::env::var_os("NO_COLOR").is_some_and(|v| !v.is_empty());
    !no_color_env && io::stdout().is_terminal()
}
