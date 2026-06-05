//! The application orchestrator.
//!
//! [`App`] owns the run: it drives an animated, interactive timer in a real
//! terminal, and falls back to plain line output when stdout/stdin aren't TTYs
//! (pipes, CI, `--plain`). The animated path composes [`crate::widgets`] into a
//! [`Frame`], diff-renders it via [`Renderer`], and reads keyboard [`Control`]s
//! each frame. All countdown arithmetic lives in [`PhaseTimer`].

use std::io::{self, Write};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use anyhow::Result;
use unicode_width::UnicodeWidthStr;

use crate::clock::PhaseTimer;
use crate::feedback::Feedback;
use crate::input::{self, Control};
use crate::render::{Frame, Line, Renderer};
use crate::session::Session;
use crate::stats::{self, Stats};
use crate::term::{self, TerminalSession};
use crate::theme::{Rgb, Theme};
use crate::widgets;
use crate::{Phase, quotes};

/// What a finished run produced, for the caller's summary.
#[derive(Debug, Clone, Copy)]
pub struct Outcome {
    pub completed_focus: u64,
    pub interrupted: bool,
}

/// Orchestrates a session from start to finish.
pub struct App {
    theme: Theme,
    feedback: Feedback,
    frame_dt: Duration,
}

impl App {
    /// Build the app from resolved session preferences.
    pub fn new(session: &Session) -> App {
        let theme = Theme::resolve(&session.theme, session.color);
        let feedback = Feedback::new(session.notifications, session.sound);
        let fps = session.fps.clamp(2, 60);
        let frame_dt = Duration::from_secs_f64(1.0 / f64::from(fps));
        App { theme, feedback, frame_dt }
    }

    /// Run the whole session. `shutdown` is the Ctrl+C flag used by the plain
    /// fallback (the interactive path reads Ctrl+C as a keypress instead).
    pub fn run(
        &mut self,
        session: &Session,
        stats: &mut Stats,
        shutdown: Arc<AtomicBool>,
    ) -> Result<Outcome> {
        if session.plain || !term::is_interactive() {
            self.run_plain(session, stats, &shutdown)
        } else {
            self.run_tui(session, stats, &shutdown)
        }
    }

    // ----------------------------------------------------------------- TUI ---

    fn run_tui(
        &mut self,
        session: &Session,
        stats: &mut Stats,
        shutdown: &Arc<AtomicBool>,
    ) -> Result<Outcome> {
        let _term = TerminalSession::enter()?;
        let mut renderer = Renderer::new(io::BufWriter::new(io::stdout()));

        let plan = session.phases();
        let work_minutes = session.work.as_secs() / 60;
        let mut completed_focus = 0u64;
        let mut last_quote: Option<&str> = None;
        let mut frame: usize = 0;
        let mut quit = false;

        'outer: for (idx, &(phase, duration)) in plan.iter().enumerate() {
            if idx != 0 {
                self.feedback.announce(phase);
            }
            let quote = if phase.is_focus() {
                None
            } else {
                let q = quotes::random_quote(last_quote);
                last_quote = Some(q);
                Some(q)
            };

            let mut timer = PhaseTimer::start(duration, Instant::now());
            let mut completed = true;

            loop {
                if shutdown.load(Ordering::SeqCst) {
                    quit = true;
                    completed = false;
                    break;
                }
                let now = Instant::now();
                if timer.is_done(now) {
                    break;
                }

                let (w, h) = TerminalSession::size();
                let (width, height) = (w as usize, h as usize);
                let lines = self
                    .compose(phase, &timer, now, quote, idx + 1, session, frame, width, height)
                    .position(width, height);
                renderer.present(&lines)?;
                frame = frame.wrapping_add(1);

                match input::poll(self.frame_dt)? {
                    Some(Control::Quit) => {
                        quit = true;
                        completed = false;
                        break;
                    }
                    Some(Control::Skip) => {
                        completed = false;
                        break;
                    }
                    Some(Control::TogglePause) => timer.toggle(Instant::now()),
                    Some(Control::Extend) => timer.extend(session.step),
                    Some(Control::Shrink) => timer.shrink(session.step, Instant::now()),
                    Some(Control::Resize) => renderer.clear()?,
                    None => {}
                }
            }

            if phase.is_focus() && completed {
                completed_focus += 1;
                stats.record_pomodoro(work_minutes, &stats::today());
            }
            if quit {
                break 'outer;
            }
        }

        if !quit {
            self.celebrate(&mut renderer, completed_focus)?;
        }

        Ok(Outcome { completed_focus, interrupted: quit })
    }

    /// Compose one animation frame.
    #[allow(clippy::too_many_arguments)]
    fn compose(
        &self,
        phase: Phase,
        timer: &PhaseTimer,
        now: Instant,
        quote: Option<&str>,
        cycle: usize,
        session: &Session,
        frame: usize,
        width: usize,
        height: usize,
    ) -> Frame {
        let theme = &self.theme;
        let p = &theme.palette;
        let accent = theme.phase_color(phase);
        let frac_remaining = timer.fraction_remaining(now);
        let elapsed_frac = 1.0 - frac_remaining;
        let paused = timer.is_paused();

        // Coffee level and steam: focus drains the cup, a break refills it.
        let (fill, steam) = if phase.is_focus() {
            (frac_remaining, frac_remaining)
        } else {
            (elapsed_frac, elapsed_frac)
        };
        // The steam animation freezes while paused.
        let anim = if paused { frame / 6 } else { frame } / 3;

        let show_cup = height >= 19 && width >= 26;
        let show_big = height >= (if show_cup { 26 } else { 12 });
        let bar_w = (width.saturating_sub(12)).clamp(10, 46);

        let mut f = Frame::new();

        // Header chip: phase + cycle counter.
        let mut head = LineBuf::new();
        head.bold(theme, &format!("▌ {} ▐", phase.label()), accent);
        head.dim(theme, format!("  cycle {} of {}", cycle, session.cycles));
        f.push(head.into_line());
        f.push_blank();

        if show_cup {
            f.extend(widgets::coffee_cup(theme, fill, steam, anim));
            f.push_blank();
        }

        let remaining = ceil_secs(timer.remaining(now));
        let time_str = fmt_mmss(remaining);
        if show_big {
            let color = if paused && (frame / 8) % 2 == 0 { p.muted } else { accent };
            f.extend(widgets::big_time(theme, &time_str, color));
        } else {
            let mut t = LineBuf::new();
            t.bold(theme, &time_str, accent);
            f.push(t.into_line());
        }
        f.push_blank();

        // Progress bar + meta line.
        f.push(widgets::progress_bar(theme, elapsed_frac, bar_w, accent, p.accent));
        let mut meta = LineBuf::new();
        meta.dim(theme, format!("{} left", time_str));
        meta.dim(theme, "  ·  ");
        meta.dim(theme, format!("{}%", (elapsed_frac * 100.0).round() as u64));
        if let Some(label) = &session.label {
            meta.dim(theme, "  ·  ");
            meta.color(theme, label, p.muted);
        }
        if paused {
            meta.plain(theme, "  ");
            // Blink the PAUSED marker.
            if (frame / 8) % 2 == 0 {
                meta.bold(theme, "⏸ PAUSED", p.warn);
            } else {
                meta.plain(theme, "        ");
            }
        }
        f.push(meta.into_line());

        // Quote during breaks, wrapped.
        if let Some(q) = quote {
            f.push_blank();
            let wrap = width.saturating_sub(8).clamp(20, 64);
            for line in textwrap::wrap(q, wrap) {
                let mut l = LineBuf::new();
                l.dim(theme, &line);
                f.push(l.into_line());
            }
        }

        f.push_blank();
        let mut hint = LineBuf::new();
        hint.dim(theme, "space pause · s skip · +/- adjust · q quit");
        f.push(hint.into_line());

        f
    }

    /// A short celebratory finale animation in the alternate screen.
    fn celebrate<W: Write>(&self, renderer: &mut Renderer<W>, pomodoros: u64) -> Result<()> {
        let theme = &self.theme;
        let p = &theme.palette;
        let mut last_size = (0u16, 0u16);

        for frame in 0..36 {
            let (w, h) = TerminalSession::size();
            if (w, h) != last_size {
                renderer.clear()?; // repaint cleanly after a resize
                last_size = (w, h);
            }
            let width = w as usize;
            let cw = width.saturating_sub(6).clamp(20, 56);

            let mut f = Frame::new();
            f.push(widgets::confetti(theme, cw, frame));
            f.push_blank();
            // Big digits showing how many pomodoros were completed.
            f.extend(widgets::big_time(theme, &pomodoros.to_string(), p.success));
            f.push_blank();
            let mut msg = LineBuf::new();
            msg.bold(
                theme,
                &format!("Session complete — {} pomodoro{} done!", pomodoros, plural(pomodoros)),
                p.success,
            );
            f.push(msg.into_line());
            f.push_blank();
            f.push(widgets::confetti(theme, cw, frame + 4));
            renderer.present(&f.position(width, h as usize))?;

            // The poll is both the frame delay and an escape hatch: any key ends
            // the finale immediately (so quit/Ctrl+C aren't dead during it).
            if input::poll(Duration::from_millis(55))?.is_some() {
                break;
            }
        }
        Ok(())
    }

    // --------------------------------------------------------------- plain ---

    fn run_plain(
        &mut self,
        session: &Session,
        stats: &mut Stats,
        shutdown: &Arc<AtomicBool>,
    ) -> Result<Outcome> {
        let theme = &self.theme;
        let plan = session.phases();
        let work_minutes = session.work.as_secs() / 60;
        let mut completed_focus = 0u64;
        let mut last_quote: Option<&str> = None;
        let mut interrupted = false;

        println!(
            "{}  {}",
            theme.bold("coffeebreak", theme.palette.accent),
            theme.dim(format!(
                "{} cycle{} · focus {} / break {}",
                session.cycles,
                plural(session.cycles),
                fmt_mmss(session.work),
                fmt_mmss(session.short_break)
            ))
        );

        'outer: for (idx, &(phase, duration)) in plan.iter().enumerate() {
            if idx != 0 {
                self.feedback.announce(phase);
            }
            let accent = theme.phase_color(phase);
            print!("{}  {}", theme.bold(format!("▶ {}", phase.label()), accent), fmt_mmss(duration));
            if !phase.is_focus() {
                let q = quotes::random_quote(last_quote);
                last_quote = Some(q);
                print!("  —  {}", theme.dim(q));
            }
            println!();
            let _ = io::stdout().flush();

            // Sleep the phase in small chunks so Ctrl+C is responsive.
            let mut completed = true;
            let end = Instant::now() + duration;
            while Instant::now() < end {
                if shutdown.load(Ordering::SeqCst) {
                    interrupted = true;
                    completed = false;
                    break;
                }
                std::thread::sleep(Duration::from_millis(150));
            }

            if phase.is_focus() && completed {
                completed_focus += 1;
                stats.record_pomodoro(work_minutes, &stats::today());
            }
            if interrupted {
                break 'outer;
            }
        }

        Ok(Outcome { completed_focus, interrupted })
    }
}

/// A small builder for a styled line that tracks visible width as it goes.
struct LineBuf {
    s: String,
    w: usize,
}

impl LineBuf {
    fn new() -> LineBuf {
        LineBuf { s: String::new(), w: 0 }
    }
    fn plain(&mut self, _theme: &Theme, text: &str) {
        self.s.push_str(text);
        self.w += text.width();
    }
    fn color(&mut self, theme: &Theme, text: &str, rgb: Rgb) {
        self.s.push_str(&theme.paint(text, rgb));
        self.w += text.width();
    }
    fn bold(&mut self, theme: &Theme, text: &str, rgb: Rgb) {
        self.s.push_str(&theme.bold(text, rgb));
        self.w += text.width();
    }
    fn dim(&mut self, theme: &Theme, text: impl AsRef<str>) {
        let t = text.as_ref();
        self.s.push_str(&theme.dim(t));
        self.w += t.width();
    }
    fn into_line(self) -> Line {
        Line::styled(self.s, self.w)
    }
}

fn plural(n: u64) -> &'static str {
    if n == 1 { "" } else { "s" }
}

/// Round a duration up to whole seconds (so a countdown shows 00:01 before 00:00).
fn ceil_secs(d: Duration) -> Duration {
    Duration::from_secs(d.as_secs() + u64::from(d.subsec_nanos() > 0))
}

/// Format a duration as `MM:SS`.
fn fmt_mmss(d: Duration) -> String {
    let s = d.as_secs();
    format!("{:02}:{:02}", s / 60, s % 60)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mmss_formats() {
        assert_eq!(fmt_mmss(Duration::from_secs(0)), "00:00");
        assert_eq!(fmt_mmss(Duration::from_secs(1500)), "25:00");
        assert_eq!(fmt_mmss(Duration::from_secs(61)), "01:01");
    }

    #[test]
    fn ceil_rounds_partial_seconds_up() {
        assert_eq!(ceil_secs(Duration::from_millis(1)).as_secs(), 1);
        assert_eq!(ceil_secs(Duration::from_secs(5)).as_secs(), 5);
    }
}
