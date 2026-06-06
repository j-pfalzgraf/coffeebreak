//! coffeebreak — a Pomodoro focus timer for the terminal.
//!
//! Thin entry point: detect the locale, parse args (with localised help),
//! dispatch subcommands, otherwise resolve a session and run the timer. Stats are
//! always saved on the way out, even after Ctrl+C or an error.

use std::io::IsTerminal;
use std::process::ExitCode;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::Result;

use coffeebreak::app::{App, Outcome};
use coffeebreak::cli::{Cli, Command, SelfAction};
use coffeebreak::config::Config;
use coffeebreak::i18n::{I18n, Msg, Noun};
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

fn run() -> Result<()> {
    // Resolve the locale *before* parsing, so even `--help`/errors are localised.
    // Config is loaded leniently here (a bad config must not block --help).
    let cfg_lang = Config::load().ok().map(|c| c.language).filter(|s| !s.is_empty());
    let help_i18n = I18n::detect(scan_lang_arg().as_deref(), cfg_lang.as_deref());

    let cli = Cli::parse_localized(&help_i18n);

    // The authoritative locale for runtime output.
    let i18n = I18n::detect(cli.lang.as_deref(), cfg_lang.as_deref());

    // Colour for non-timer output: honour --no-color, NO_COLOR, and tty-ness.
    let color = !cli.no_color
        && std::env::var_os("NO_COLOR").is_none()
        && std::io::stdout().is_terminal();
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
                commands::stats(&meta_theme, &i18n);
                Ok(())
            }
            Command::Config { action } => commands::config(action, &meta_theme, &i18n),
            Command::Themes => {
                commands::themes(&meta_theme, &i18n);
                Ok(())
            }
            Command::Presets => {
                commands::presets(&meta_theme, &i18n);
                Ok(())
            }
            Command::Languages => {
                commands::languages(&meta_theme, &i18n);
                Ok(())
            }
            Command::Completions { shell } => {
                completions::print_completions(*shell);
                Ok(())
            }
            Command::Man => completions::print_man(),
            Command::Selfcmd { action } => match action {
                SelfAction::Update { check } => selfcmd::update(*check, &i18n),
                SelfAction::Uninstall { yes } => selfcmd::uninstall(*yes, &i18n),
            },
        };
    }

    // `--stats` shortcut works even with a malformed config file.
    if cli.stats {
        commands::stats(&meta_theme, &i18n);
        return Ok(());
    }

    // Default action: run the timer.
    let config = Config::load()?;
    let session = Session::resolve(&cli, &config);
    let mut stats = Stats::load_or_default(&i18n);

    install_panic_hook();

    // Ctrl+C flag for the plain (non-raw) fallback; the animated UI reads Ctrl+C
    // as a keypress instead.
    let shutdown = Arc::new(AtomicBool::new(false));
    {
        let flag = shutdown.clone();
        if let Err(e) = ctrlc::set_handler(move || flag.store(true, Ordering::SeqCst)) {
            eprintln!("coffeebreak: {}", i18n.tf(Msg::WarnCtrlc, &[("error", &e.to_string())]));
        }
    }

    let mut app = App::new(&session);
    let outcome = app.run(&session, &mut stats, shutdown);

    if let Err(e) = stats.save() {
        eprintln!("coffeebreak: {}", i18n.tf(Msg::WarnStatsSave, &[("error", &format!("{e:#}"))]));
    }

    let outcome = outcome?;
    print_summary(&session, &outcome, &i18n);
    Ok(())
}

/// Pre-scan raw args for `--lang <code>` / `--lang=<code>` so the locale is known
/// before clap parsing (which is what produces `--help`).
fn scan_lang_arg() -> Option<String> {
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        if a == "--lang" {
            return args.next();
        }
        if let Some(v) = a.strip_prefix("--lang=") {
            return Some(v.to_string());
        }
    }
    None
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

/// Concise, localised post-run summary on the normal screen.
fn print_summary(session: &Session, outcome: &Outcome, i18n: &I18n) {
    let theme = Theme::resolve(&session.theme, session.color);
    let p = &theme.palette;
    let count = i18n.count(outcome.completed_focus, Noun::Pomodoro);
    let (msg, color) = if outcome.interrupted {
        (i18n.tf(Msg::StoppedFooter, &[("count", &count)]), p.warn)
    } else {
        (i18n.tf(Msg::DoneFooter, &[("count", &count)]), p.success)
    };
    println!("{}", theme.bold(msg, color));
}
