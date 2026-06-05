//! The core countdown loop.
//!
//! For each phase in the [`Session`] plan it renders a live block — the steaming
//! coffee cup, a status line, and a progress bar — ticking a few times a second
//! (so CPU stays near zero) and remaining responsive to Ctrl+C. Completed focus
//! blocks are credited to [`Stats`]; an interrupted block is not.

use std::io::{self, IsTerminal};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};

use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use owo_colors::OwoColorize;

use crate::session::Session;
use crate::stats::{self, Stats};
use crate::{Phase, art, notify, quotes};

/// How often the live block is repainted. 200 ms keeps Ctrl+C responsive and
/// gives the steam a gentle animation, while idling near 0% CPU.
const TICK: Duration = Duration::from_millis(200);

/// Restores the terminal cursor when the timer ends, however it ends.
struct CursorGuard {
    tty: bool,
}

impl CursorGuard {
    fn hide() -> Self {
        let tty = io::stderr().is_terminal();
        if tty {
            let _ = crossterm::execute!(io::stderr(), crossterm::cursor::Hide);
        }
        CursorGuard { tty }
    }
}

impl Drop for CursorGuard {
    fn drop(&mut self) {
        if self.tty {
            let _ = crossterm::execute!(io::stderr(), crossterm::cursor::Show);
        }
    }
}

/// Run the whole session. `shutdown` is set asynchronously by the Ctrl+C
/// handler; the loop checks it and returns cleanly so the caller can still
/// persist stats.
pub fn run(session: &Session, stats: &mut Stats, shutdown: Arc<AtomicBool>) -> Result<()> {
    let color = session.color && io::stderr().is_terminal();
    let _cursor = CursorGuard::hide();

    print_header(session, color);

    let plan = session.phases();
    let work_minutes = session.work.as_secs() / 60;
    let mut last_quote: Option<&str> = None;
    let mut completed_focus = 0u64;

    for (idx, &(phase, duration)) in plan.iter().enumerate() {
        if shutdown.load(Ordering::SeqCst) {
            break;
        }

        // Notify/sound on every transition except the very first focus block
        // (the user just launched it and is watching the terminal).
        if idx != 0 {
            notify::announce_phase(phase, session.sound, session.notifications);
        }

        // A developer quote greets each break.
        let quote = if phase.is_focus() {
            None
        } else {
            let q = quotes::random_quote(last_quote);
            last_quote = Some(q);
            Some(q)
        };

        let completed = run_phase(phase, duration, session, quote, color, &shutdown)?;

        if phase.is_focus() && completed {
            completed_focus += 1;
            stats.record_pomodoro(work_minutes, &stats::today());
        }
    }

    print_footer(completed_focus, shutdown.load(Ordering::SeqCst), color);
    Ok(())
}

/// Run a single phase. Returns `true` if it ran to completion, `false` if it was
/// interrupted by Ctrl+C.
fn run_phase(
    phase: Phase,
    total: Duration,
    session: &Session,
    quote: Option<&str>,
    color: bool,
    shutdown: &Arc<AtomicBool>,
) -> Result<bool> {
    let total_secs = total.as_secs().max(1);
    let pb = ProgressBar::new(total_secs);
    pb.set_style(phase_style(phase, color)?);

    let start = Instant::now();
    let mut frame: usize = 0;

    loop {
        if shutdown.load(Ordering::SeqCst) {
            pb.finish_and_clear();
            return Ok(false);
        }

        let elapsed = start.elapsed();
        if elapsed >= total {
            break;
        }

        let remaining = total - elapsed;
        let frac = remaining.as_secs_f64() / total.as_secs_f64();
        pb.set_position(elapsed.as_secs().min(total_secs));
        // Steam animates roughly every 800 ms (frame / 4 at a 200 ms tick).
        pb.set_message(render_block(phase, remaining, frac, frame / 4, session, quote, color));

        frame = frame.wrapping_add(1);
        thread::sleep(TICK);
    }

    pb.set_position(total_secs);
    pb.finish_and_clear();
    Ok(true)
}

/// Build the multi-line live block fed to the progress message: the cup, then a
/// status line, then (for breaks) the quote.
fn render_block(
    phase: Phase,
    remaining: Duration,
    frac: f64,
    frame: usize,
    session: &Session,
    quote: Option<&str>,
    color: bool,
) -> String {
    let cup = art::coffee_cup(frac, frame);
    let time = fmt_mmss(remaining);

    let mut status = format!("  {}  ·  {} left", phase.label(), time);
    if let Some(label) = &session.label {
        status.push_str(&format!("  ·  {label}"));
    }

    let mut block = String::new();
    if color {
        block.push_str(&cup.yellow().to_string());
        block.push('\n');
        block.push_str(&status.bold().to_string());
        if let Some(q) = quote {
            block.push('\n');
            block.push_str(&format!("  {}", q).italic().dimmed().to_string());
        }
    } else {
        block.push_str(&cup);
        block.push('\n');
        block.push_str(&status);
        if let Some(q) = quote {
            block.push_str(&format!("\n  {q}"));
        }
    }
    block
}

/// The progress-bar style for a phase. Focus is warm/red, breaks are cool.
fn phase_style(phase: Phase, color: bool) -> Result<ProgressStyle> {
    let template = if color {
        let bar_color = match phase {
            Phase::Focus => "red",
            Phase::ShortBreak => "cyan",
            Phase::LongBreak => "magenta",
        };
        format!("{{msg}}\n  [{{bar:32.{bar_color}}}] {{percent:>3}}%")
    } else {
        "{msg}\n  [{bar:32}] {percent:>3}%".to_string()
    };
    Ok(ProgressStyle::with_template(&template)?.progress_chars("█▉ "))
}

/// Header printed once above the live block.
fn print_header(session: &Session, color: bool) {
    let cycles = session.cycles;
    let plan = format!(
        "{} cycle{} · focus {} / break {}",
        cycles,
        if cycles == 1 { "" } else { "s" },
        fmt_mmss(session.work),
        fmt_mmss(session.short_break),
    );
    let title = "☕ coffeebreak";
    if color {
        println!("\n{}  {}\n", title.bold().yellow(), plan.dimmed());
    } else {
        println!("\n{title}  {plan}\n");
    }
}

/// Footer printed once when the session ends or is interrupted.
fn print_footer(completed_focus: u64, interrupted: bool, color: bool) {
    let msg = if interrupted {
        format!(
            "Stopped. {completed_focus} pomodoro{} completed this session.",
            plural(completed_focus)
        )
    } else {
        format!(
            "Session complete! {completed_focus} pomodoro{} done. 🎉",
            plural(completed_focus)
        )
    };
    if color {
        println!("{}", msg.bold().green());
    } else {
        println!("{msg}");
    }
}

fn plural(n: u64) -> &'static str {
    if n == 1 { "" } else { "s" }
}

/// Format a duration as `MM:SS`, rounding partial seconds up so a countdown
/// reaches `00:01` before completing rather than dwelling on `00:00`.
fn fmt_mmss(d: Duration) -> String {
    let secs = d.as_secs() + if d.subsec_nanos() > 0 { 1 } else { 0 };
    format!("{:02}:{:02}", secs / 60, secs % 60)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mmss_rounds_up_partials() {
        assert_eq!(fmt_mmss(Duration::from_secs(0)), "00:00");
        assert_eq!(fmt_mmss(Duration::from_millis(1)), "00:01");
        assert_eq!(fmt_mmss(Duration::from_secs(59)), "00:59");
        assert_eq!(fmt_mmss(Duration::from_secs(60)), "01:00");
        assert_eq!(fmt_mmss(Duration::from_secs(1500)), "25:00");
    }
}
