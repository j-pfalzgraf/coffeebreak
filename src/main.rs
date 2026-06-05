//! coffeebreak — a Pomodoro focus timer for the terminal.
//!
//! Thin entry point: parse args, dispatch subcommands, otherwise resolve a
//! session and run the timer. Stats are always saved on the way out, even after
//! Ctrl+C or an error.

use std::io::IsTerminal;
use std::process::ExitCode;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::Result;

use coffeebreak::app::{App, Outcome};
use coffeebreak::cli::{Cli, Command, SelfAction};
use coffeebreak::config::Config;
use coffeebreak::session::Session;
use coffeebreak::stats::Stats;
use coffeebreak::theme::{DEFAULT_THEME, Theme};
use coffeebreak::{commands, completions, selfcmd};

fn main() -> ExitCode {
    reset_sigpipe();
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("coffeebreak: error: {e:#}");
            ExitCode::FAILURE
        }
    }
}

/// Restore the default SIGPIPE disposition on Unix. By default Rust ignores
/// SIGPIPE and surfaces a closed pipe as a write error, which `println!` turns
/// into a panic — so `coffeebreak man | head` would crash with exit 101.
/// Resetting to the default makes the process terminate quietly instead.
#[cfg(unix)]
fn reset_sigpipe() {
    // SAFETY: setting a signal handler to the default disposition is sound.
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }
}

#[cfg(not(unix))]
fn reset_sigpipe() {}

fn run() -> Result<()> {
    let cli = Cli::parse_args();

    // Colour for non-timer output: honour --no-color, NO_COLOR, and tty-ness.
    let color = !cli.no_color
        && std::env::var_os("NO_COLOR").is_none()
        && std::io::stdout().is_terminal();
    // Theme for non-timer output: --theme flag, else the configured theme (loaded
    // leniently so a bad config never blocks meta commands), else the default.
    let theme_name = cli
        .theme
        .clone()
        .or_else(|| Config::load().ok().map(|c| c.theme))
        .filter(|t| !t.is_empty())
        .unwrap_or_else(|| DEFAULT_THEME.to_string());
    let meta_theme = Theme::resolve(&theme_name, color);

    // Subcommands (none of these run the timer).
    if let Some(cmd) = &cli.command {
        return match cmd {
            Command::Stats => {
                commands::stats(&meta_theme);
                Ok(())
            }
            Command::Config { action } => commands::config(action, &meta_theme),
            Command::Themes => {
                commands::themes(color);
                Ok(())
            }
            Command::Presets => {
                commands::presets(&meta_theme);
                Ok(())
            }
            Command::Completions { shell } => {
                completions::print_completions(*shell);
                Ok(())
            }
            Command::Man => completions::print_man(),
            Command::Selfcmd { action } => match action {
                SelfAction::Update { check } => selfcmd::update(*check),
                SelfAction::Uninstall { yes } => selfcmd::uninstall(*yes),
            },
        };
    }

    // `--stats` shortcut works even with a malformed config file.
    if cli.stats {
        commands::stats(&meta_theme);
        return Ok(());
    }

    // Default action: run the timer.
    let config = Config::load()?;
    let session = Session::resolve(&cli, &config);
    let mut stats = Stats::load_or_default();

    install_panic_hook();

    // Ctrl+C flag for the plain (non-raw) fallback; the animated UI reads Ctrl+C
    // as a keypress instead.
    let shutdown = Arc::new(AtomicBool::new(false));
    {
        let flag = shutdown.clone();
        if let Err(e) = ctrlc::set_handler(move || flag.store(true, Ordering::SeqCst)) {
            eprintln!(
                "coffeebreak: warning: could not install Ctrl+C handler ({e}); \
                 stats may not be saved if you interrupt the session"
            );
        }
    }

    let mut app = App::new(&session);
    let outcome = app.run(&session, &mut stats, shutdown);

    // Persist stats however the run ended.
    if let Err(e) = stats.save() {
        eprintln!("coffeebreak: warning: could not save stats ({e:#})");
    }

    let outcome = outcome?;
    print_summary(&session, &outcome);
    Ok(())
}

/// Restore the terminal (cursor, alternate screen, raw mode) on panic — needed
/// because the release profile uses `panic = "abort"`, which skips unwinding and
/// therefore the `TerminalSession` drop guard.
fn install_panic_hook() {
    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        coffeebreak::term::TerminalSession::restore();
        prev(info);
    }));
}

/// Concise post-run summary on the normal screen.
fn print_summary(session: &Session, outcome: &Outcome) {
    let theme = Theme::resolve(&session.theme, session.color);
    let p = &theme.palette;
    let n = outcome.completed_focus;
    let s = if n == 1 { "" } else { "s" };
    if outcome.interrupted {
        println!("{}", theme.bold(format!("Stopped — {n} pomodoro{s} completed this session."), p.warn));
    } else {
        println!("{}", theme.bold(format!("Done! {n} pomodoro{s} completed. ☕"), p.success));
    }
}
