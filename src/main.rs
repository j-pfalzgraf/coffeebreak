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
use coffeebreak::cli::{Cli, Command, SelfAction, StatsFormat};
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
    // Install the panic hook first so the terminal cursor/screen is restored on a
    // panic from ANY path (the animated stats reveal hides the cursor too, not
    // just the timer).
    install_panic_hook();

    // Resolve the locale *before* parsing, so even `--help`/errors are localised.
    // Config is loaded leniently here (a bad config must not block --help).
    // Load config leniently for pre-parse locale/theme/goal (a bad config must
    // not block --help or meta commands; the timer path re-loads it strictly).
    let cfg = Config::load().ok();
    let cfg_lang = cfg
        .as_ref()
        .map(|c| c.language.clone())
        .filter(|s| !s.is_empty());
    let help_i18n = I18n::detect(scan_lang_arg().as_deref(), cfg_lang.as_deref());

    let cli = Cli::parse_localized(&help_i18n);

    // The authoritative locale for runtime output.
    let i18n = I18n::detect(cli.lang.as_deref(), cfg_lang.as_deref());

    // Colour for non-timer output: honour --no-color, NO_COLOR, and tty-ness.
    let color =
        !cli.no_color && std::env::var_os("NO_COLOR").is_none() && std::io::stdout().is_terminal();
    let theme_name = cli
        .theme
        .clone()
        .or_else(|| cfg.as_ref().map(|c| c.theme.clone()))
        .filter(|t| !t.is_empty())
        .unwrap_or_else(|| DEFAULT_THEME.to_string());
    // Build the optional `custom` palette from config once; reused for both the
    // meta-command theme and the timer theme.
    let custom_palette = cfg
        .as_ref()
        .filter(|c| !c.custom_theme.is_empty())
        .map(|c| {
            coffeebreak::theme::custom_palette(
                c.custom_theme.iter().map(|(k, v)| (k.as_str(), v.as_str())),
            )
        });
    let meta_theme = Theme::build(&theme_name, color, custom_palette);
    let goal = cli
        .goal
        .or_else(|| cfg.as_ref().map(|c| c.daily_goal))
        .unwrap_or(0);

    // Subcommands (none of these run the timer).
    if let Some(cmd) = &cli.command {
        return match cmd {
            Command::Stats { format } => {
                commands::stats(&meta_theme, &i18n, goal, *format);
                Ok(())
            }
            Command::Doctor => {
                commands::doctor(&meta_theme, &i18n);
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
        commands::stats(&meta_theme, &i18n, goal, StatsFormat::Text);
        return Ok(());
    }

    // Default action: run the timer.
    let config = Config::load()?;
    let session = Session::resolve(&cli, &config);
    let mut stats = Stats::load_or_default(&i18n);

    // Ctrl+C flag for the plain (non-raw) fallback; the animated UI reads Ctrl+C
    // as a keypress instead.
    let shutdown = Arc::new(AtomicBool::new(false));
    {
        let flag = shutdown.clone();
        if let Err(e) = ctrlc::set_handler(move || flag.store(true, Ordering::SeqCst)) {
            eprintln!(
                "coffeebreak: {}",
                i18n.tf(Msg::WarnCtrlc, &[("error", &e.to_string())])
            );
        }
    }

    let run_theme = Theme::build(&session.theme, session.color, custom_palette);
    let mut app = App::new(&session, run_theme);
    let outcome = app.run(&session, &mut stats, shutdown);

    if let Err(e) = stats.save() {
        eprintln!(
            "coffeebreak: {}",
            i18n.tf(Msg::WarnStatsSave, &[("error", &format!("{e:#}"))])
        );
    }

    let outcome = outcome?;
    // `run_theme` is `Copy`, so it's still valid after `App::new` took a copy.
    print_summary(&run_theme, &outcome, &i18n);
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
fn print_summary(theme: &Theme, outcome: &Outcome, i18n: &I18n) {
    let p = &theme.palette;
    let count = i18n.count(outcome.completed_focus, Noun::Pomodoro);
    let (msg, color) = if outcome.interrupted {
        (i18n.tf(Msg::StoppedFooter, &[("count", &count)]), p.warn)
    } else {
        (i18n.tf(Msg::DoneFooter, &[("count", &count)]), p.success)
    };
    println!("{}", theme.bold(msg, color));
}
