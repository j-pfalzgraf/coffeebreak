//! Keyboard controls for the running timer.
//!
//! In raw mode the terminal delivers Ctrl+C as a key event (not SIGINT), so we
//! treat it as quit here. Polling with a timeout doubles as the frame clock:
//! [`poll`] blocks up to `timeout`, returning early when the user presses a key,
//! which keeps CPU near zero while staying instantly responsive.

use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

/// A user control action mapped from a keypress (or a terminal resize).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Control {
    /// Pause or resume the countdown (space / `p`).
    TogglePause,
    /// Skip the rest of the current phase (`s` / `n`).
    Skip,
    /// Quit the whole session (`q` / Esc / Ctrl+C).
    Quit,
    /// Add a minute to the current phase (`+` / `=` / Up).
    Extend,
    /// Remove a minute from the current phase (`-` / Down).
    Shrink,
    /// The terminal was resized; the renderer cache must be invalidated.
    Resize,
}

/// Wait up to `timeout` for a control action. Returns `None` on timeout with no
/// recognised input.
pub fn poll(timeout: Duration) -> Result<Option<Control>> {
    if !event::poll(timeout)? {
        return Ok(None);
    }
    match event::read()? {
        Event::Key(key) => Ok(map_key(key)),
        Event::Resize(_, _) => Ok(Some(Control::Resize)),
        _ => Ok(None),
    }
}

/// The outcome of waiting on the "press any key to continue" screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaitEvent {
    /// Nothing happened before the timeout elapsed.
    Timeout,
    /// A key was pressed to continue.
    Continue,
    /// The user asked to quit (`q` / Esc / Ctrl+C).
    Quit,
    /// The terminal was resized.
    Resize,
}

/// Wait up to `timeout` for the between-phases screen: any key continues, the
/// usual quit keys quit.
pub fn poll_wait(timeout: Duration) -> Result<WaitEvent> {
    if !event::poll(timeout)? {
        return Ok(WaitEvent::Timeout);
    }
    match event::read()? {
        Event::Key(key) => Ok(if is_quit(key) {
            WaitEvent::Quit
        } else {
            WaitEvent::Continue
        }),
        Event::Resize(_, _) => Ok(WaitEvent::Resize),
        _ => Ok(WaitEvent::Timeout),
    }
}

/// Whether a key event means "quit".
fn is_quit(key: KeyEvent) -> bool {
    if key.modifiers.contains(KeyModifiers::CONTROL)
        && matches!(key.code, KeyCode::Char('c') | KeyCode::Char('d'))
    {
        return true;
    }
    matches!(
        key.code,
        KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc
    )
}

fn map_key(key: KeyEvent) -> Option<Control> {
    if is_quit(key) {
        return Some(Control::Quit);
    }
    match key.code {
        KeyCode::Char(' ') | KeyCode::Char('p') | KeyCode::Char('P') => Some(Control::TogglePause),
        KeyCode::Char('s') | KeyCode::Char('S') | KeyCode::Char('n') | KeyCode::Char('N') => {
            Some(Control::Skip)
        }
        KeyCode::Char('+') | KeyCode::Char('=') | KeyCode::Up => Some(Control::Extend),
        KeyCode::Char('-') | KeyCode::Char('_') | KeyCode::Down => Some(Control::Shrink),
        _ => None,
    }
}
