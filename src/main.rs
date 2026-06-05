//! coffeebreak — a Pomodoro focus timer for the terminal.
//!
//! Thin entry point: parse args, dispatch lifecycle subcommands, otherwise
//! resolve a session and run the timer. Stats are always saved on the way out,
//! even after Ctrl+C.

use std::io::IsTerminal;
use std::process::ExitCode;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::Result;

use coffeebreak::cli::{Cli, Command, SelfAction};
use coffeebreak::config::Config;
use coffeebreak::session::Session;
use coffeebreak::stats::Stats;
use coffeebreak::{selfcmd, timer};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("coffeebreak: error: {e:#}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse_args();

    // Lifecycle subcommands need neither config nor stats.
    if let Some(Command::Selfcmd { action }) = &cli.command {
        return match action {
            SelfAction::Update { check } => selfcmd::update(*check),
            SelfAction::Uninstall { yes } => selfcmd::uninstall(*yes),
        };
    }

    // `--stats` works even if the config file is malformed.
    if cli.stats {
        let color = !cli.no_color && std::io::stdout().is_terminal();
        Stats::load_or_default().print_summary(color);
        return Ok(());
    }

    let config = Config::load()?;
    let session = Session::resolve(&cli, &config);
    let mut stats = Stats::load_or_default();

    // The timer hides the terminal cursor; restore it even on a panic. Because
    // the release profile uses panic="abort" (no unwinding), the CursorGuard's
    // Drop would not run on panic, so we re-show the cursor in a panic hook too.
    {
        let prev = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            let _ = crossterm::execute!(std::io::stderr(), crossterm::cursor::Show);
            prev(info);
        }));
    }

    // Ctrl+C flips a flag the timer polls, so the run unwinds cleanly and we
    // still persist whatever was completed.
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

    let result = timer::run(&session, &mut stats, shutdown);

    if let Err(e) = stats.save() {
        eprintln!("coffeebreak: warning: could not save stats ({e:#})");
    }

    result
}
