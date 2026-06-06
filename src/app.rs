//! The application orchestrator.
//!
//! [`App`] owns the run: it drives an animated, interactive timer in a real
//! terminal, and falls back to plain line output when stdout/stdin aren't TTYs
//! (pipes, CI, `--plain`). The animated path composes [`crate::widgets`] into a
//! [`Frame`], diff-renders it via [`Renderer`], and reads keyboard [`Control`]s
//! each frame. All countdown arithmetic lives in [`PhaseTimer`].

use std::io::{self, IsTerminal, Write};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use anyhow::Result;
use unicode_width::UnicodeWidthStr;

use crate::clock::PhaseTimer;
use crate::feedback::Feedback;
use crate::i18n::{I18n, Msg, Noun};
use crate::input::{self, Control, WaitEvent};
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
    i18n: I18n,
    frame_dt: Duration,
}

impl App {
    /// Build the app from resolved session preferences.
    pub fn new(session: &Session) -> App {
        let theme = Theme::resolve(&session.theme, session.color);
        let i18n = I18n::new(&session.lang);
        let feedback = Feedback::new(session.notifications, session.sound, i18n);
        let fps = session.fps.clamp(2, 60);
        let frame_dt = Duration::from_secs_f64(1.0 / f64::from(fps));
        App {
            theme,
            feedback,
            i18n,
            frame_dt,
        }
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
                // Manual advance: alert (above), then wait for the user.
                if !session.auto_advance {
                    frame = frame.wrapping_add(1);
                    if !self.wait_screen(&mut renderer, phase, idx + 1, session, &mut frame)? {
                        quit = true;
                        break;
                    }
                }
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
                    .compose(
                        phase,
                        &timer,
                        now,
                        quote,
                        idx + 1,
                        session,
                        frame,
                        width,
                        height,
                    )
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

        Ok(Outcome {
            completed_focus,
            interrupted: quit,
        })
    }

    /// The animated "press any key to continue" screen shown between phases in
    /// manual-advance mode. Returns `true` to continue, `false` to quit.
    fn wait_screen<W: Write>(
        &self,
        renderer: &mut Renderer<W>,
        next: Phase,
        cycle: usize,
        session: &Session,
        frame: &mut usize,
    ) -> Result<bool> {
        let theme = &self.theme;
        let accent = theme.phase_color(next);
        loop {
            let (w, h) = TerminalSession::size();
            let (width, height) = (w as usize, h as usize);
            let f = *frame;

            let mut fr = Frame::new();
            let mut head = LineBuf::new();
            head.bold(
                theme,
                &format!("▌ {} ▐", self.i18n.phase_label(next)),
                accent,
            );
            head.dim(
                theme,
                format!(
                    "  {}",
                    self.i18n.tf(
                        Msg::CycleOf,
                        &[
                            ("n", &cycle.to_string()),
                            ("total", &session.cycles.to_string())
                        ],
                    )
                ),
            );
            fr.push(head.into_line());
            fr.push_blank();

            if height >= 16 && width >= 26 {
                // A steaming, ready cup awaiting the user.
                fr.extend(widgets::coffee_cup(theme, 1.0, 1.0, f / 3));
                fr.push_blank();
            }

            // Gently pulse the prompt so it reads as "waiting".
            let mut prompt = LineBuf::new();
            if (f / 4) % 2 == 0 {
                prompt.bold(theme, self.i18n.t(Msg::WaitContinue), accent);
            } else {
                prompt.dim(theme, self.i18n.t(Msg::WaitContinue));
            }
            fr.push(prompt.into_line());

            renderer.present(&fr.position(width, height))?;
            *frame = frame.wrapping_add(1);

            match input::poll_wait(self.frame_dt)? {
                WaitEvent::Continue => return Ok(true),
                WaitEvent::Quit => return Ok(false),
                WaitEvent::Resize => renderer.clear()?,
                WaitEvent::Timeout => {}
            }
        }
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
        head.bold(
            theme,
            &format!("▌ {} ▐", self.i18n.phase_label(phase)),
            accent,
        );
        head.dim(
            theme,
            format!(
                "  {}",
                self.i18n.tf(
                    Msg::CycleOf,
                    &[
                        ("n", &cycle.to_string()),
                        ("total", &session.cycles.to_string())
                    ],
                )
            ),
        );
        f.push(head.into_line());
        f.push_blank();

        if show_cup {
            f.extend(widgets::coffee_cup(theme, fill, steam, anim));
            f.push_blank();
        }

        let remaining = ceil_secs(timer.remaining(now));
        let time_str = fmt_mmss(remaining);
        if show_big {
            let color = if paused && (frame / 8) % 2 == 0 {
                p.muted
            } else {
                accent
            };
            f.extend(widgets::big_time(theme, &time_str, color));
        } else {
            let mut t = LineBuf::new();
            t.bold(theme, &time_str, accent);
            f.push(t.into_line());
        }
        f.push_blank();

        // Progress bar + meta line.
        f.push(widgets::progress_bar(
            theme,
            elapsed_frac,
            bar_w,
            accent,
            p.accent,
        ));
        let mut meta = LineBuf::new();
        meta.dim(theme, format!("{} {}", time_str, self.i18n.t(Msg::Left)));
        meta.dim(theme, "  ·  ");
        meta.dim(theme, format!("{}%", (elapsed_frac * 100.0).round() as u64));
        if let Some(label) = &session.label {
            meta.dim(theme, "  ·  ");
            meta.color(theme, label, p.muted);
        }
        if paused {
            // Blink the PAUSED marker; the off-frame is the same width of blanks
            // so the line doesn't jitter (the localised word can differ in width).
            let marker = format!("⏸ {}", self.i18n.t(Msg::Paused));
            let marker_w = marker.width();
            meta.plain(theme, "  ");
            if (frame / 8) % 2 == 0 {
                meta.bold(theme, &marker, p.warn);
            } else {
                meta.plain(theme, &" ".repeat(marker_w));
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
        hint.dim(theme, self.i18n.t(Msg::ControlsHint));
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
            let count = self.i18n.count(pomodoros, Noun::Pomodoro);
            msg.bold(
                theme,
                &self.i18n.tf(Msg::CelebrateMsg, &[("count", &count)]),
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

        let plan_summary = self.i18n.tf(
            Msg::PlanSummary,
            &[
                ("count", &self.i18n.count(session.cycles, Noun::Cycle)),
                ("work", &fmt_mmss(session.work)),
                ("brk", &fmt_mmss(session.short_break)),
            ],
        );
        println!(
            "{}  {}",
            theme.bold("coffeebreak", theme.palette.accent),
            theme.dim(plan_summary)
        );

        'outer: for (idx, &(phase, duration)) in plan.iter().enumerate() {
            if idx != 0 {
                self.feedback.announce(phase);
                // Manual advance: wait for Enter, but only on an interactive
                // stdin (piped/CI input auto-advances so nothing blocks).
                if !session.auto_advance && io::stdin().is_terminal() {
                    println!("{}", theme.dim(self.i18n.t(Msg::WaitContinuePlain)));
                    let _ = io::stdout().flush();
                    let mut buf = String::new();
                    let eof = io::stdin().read_line(&mut buf).unwrap_or(0) == 0;
                    if eof || shutdown.load(Ordering::SeqCst) {
                        interrupted = true;
                        break 'outer;
                    }
                }
            }
            let accent = theme.phase_color(phase);
            print!(
                "{}  {}",
                theme.bold(format!("▶ {}", self.i18n.phase_label(phase)), accent),
                fmt_mmss(duration)
            );
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

        Ok(Outcome {
            completed_focus,
            interrupted,
        })
    }
}

/// A small builder for a styled line that tracks visible width as it goes.
struct LineBuf {
    s: String,
    w: usize,
}

impl LineBuf {
    fn new() -> LineBuf {
        LineBuf {
            s: String::new(),
            w: 0,
        }
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
