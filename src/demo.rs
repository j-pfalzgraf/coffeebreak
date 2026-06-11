//! The `demo` showcase: a guided tour of every widget and animation.
//!
//! This is the single best way to *see* what coffeebreak draws — handy for the
//! README, an asciinema recording, or just choosing a theme. It reuses the exact
//! same building blocks as the live timer ([`crate::widgets`], [`crate::charts`])
//! composed into a [`Frame`] and diff-rendered through [`Renderer`], so what you
//! see here is what a real session looks like. Any key exits; a resize repaints.

use std::io;
use std::time::Duration;

use anyhow::Result;

use crate::charts;
use crate::i18n::{I18n, Msg};
use crate::input::{self, Control};
use crate::render::{Frame, Renderer};
use crate::term::{self, TerminalSession};
use crate::theme::Theme;
use crate::ui::LineBuf;
use crate::widgets::{self, Spinner};

/// The scenes shown, in order: `(title, number of frames)`.
const SCENES: &[(Msg, usize)] = &[
    (Msg::SceneBrewing, 30),
    (Msg::SceneCup, 48),
    (Msg::SceneClock, 44),
    (Msg::SceneRing, 42),
    (Msg::SceneSpinner, 36),
    (Msg::SceneCharts, 44),
    (Msg::SceneFinale, 40),
];

/// Run the animation showcase. Requires an interactive terminal; on a non-TTY it
/// prints a localised hint and returns cleanly.
pub fn run(theme: &Theme, i18n: &I18n) -> Result<()> {
    if !term::is_interactive() {
        println!("coffeebreak: {}", i18n.t(Msg::DemoNotTty));
        return Ok(());
    }

    let _term = TerminalSession::enter()?;
    let mut renderer = Renderer::new(io::BufWriter::new(io::stdout()));
    let frame_dt = Duration::from_secs_f64(1.0 / 15.0);

    let mut frame = 0usize;
    for &(title, len) in SCENES {
        for f in 0..len {
            let (w, h) = TerminalSession::size();
            let (width, height) = (w as usize, h as usize);
            let fr = build_frame(theme, i18n, title, f, frame, width);
            renderer.present(&fr.position(width, height))?;
            frame = frame.wrapping_add(1);

            match input::poll(frame_dt)? {
                Some(Control::Resize) => renderer.clear()?,
                // Any other key ends the demo immediately.
                Some(_) => return Ok(()),
                None => {}
            }
        }
    }
    Ok(())
}

/// Compose one scene frame: a title chip, the scene body, and a footer hint.
fn build_frame(
    theme: &Theme,
    i18n: &I18n,
    title: Msg,
    f: usize,
    frame: usize,
    width: usize,
) -> Frame {
    let p = &theme.palette;
    let mut fr = Frame::new();

    let mut head = LineBuf::new();
    head.bold(theme, &format!("▌ {} ▐", i18n.t(title)), p.accent);
    fr.push(head.into_line());
    fr.push_blank();

    match title {
        Msg::SceneBrewing => fr.extend(widgets::brew_splash(theme, f)),
        Msg::SceneCup => {
            // Drain then refill, like focus → break.
            let level = ((f as f64 * 0.12).cos() * 0.5 + 0.5).clamp(0.0, 1.0);
            fr.extend(widgets::coffee_cup(theme, level, level, frame / 3));
        }
        Msg::SceneClock => {
            let remaining = 1500u64.saturating_sub((f as u64) * 13);
            let frac = 1.0 - remaining as f64 / 1500.0;
            fr.extend(widgets::big_time(theme, &fmt_mmss(remaining), p.focus));
            fr.push_blank();
            let bw = width.saturating_sub(12).clamp(10, 46);
            fr.push(widgets::progress_bar(
                theme,
                frac,
                bw,
                p.bar_start,
                p.bar_end,
            ));
        }
        Msg::SceneRing => {
            let frac = (f as f64 / 41.0).clamp(0.0, 1.0);
            let pct = (frac * 100.0).round() as u64;
            fr.extend(widgets::ring_gauge(
                theme,
                frac,
                p.focus,
                &format!("{pct}%"),
            ));
        }
        Msg::SceneSpinner => {
            fr.push(widgets::spinner_label(
                theme,
                Spinner::Dots,
                frame,
                i18n.t(Msg::Checking),
                p.accent,
            ));
            fr.push_blank();
            fr.push(widgets::spinner_label(
                theme,
                Spinner::Bounce,
                frame,
                i18n.t(Msg::Checking),
                p.short_break,
            ));
        }
        Msg::SceneCharts => {
            let data = [1u64, 3, 2, 5, 4, 6, 2, 7, 3, 5, 4, 6];
            fr.push(charts::sparkline(theme, &data, p.focus));
            fr.push_blank();
            fr.extend(charts::bar_chart(theme, &data, 5, p.focus, p.accent));
            fr.push_blank();
            fr.push(charts::goal_bar(theme, 6, 8, 20, p.accent));
        }
        Msg::SceneFinale => {
            let cw = width.saturating_sub(6).clamp(20, 56);
            fr.push(widgets::confetti(theme, cw, frame));
            fr.push_blank();
            fr.extend(widgets::big_time(theme, "8", p.success));
            fr.push_blank();
            fr.push(widgets::confetti(theme, cw, frame + 4));
        }
        _ => {}
    }

    fr.push_blank();
    let mut foot = LineBuf::new();
    foot.dim(theme, i18n.t(Msg::DemoFooter));
    fr.push(foot.into_line());
    fr
}

/// Format whole seconds as `MM:SS`.
fn fmt_mmss(secs: u64) -> String {
    format!("{:02}:{:02}", secs / 60, secs % 60)
}
