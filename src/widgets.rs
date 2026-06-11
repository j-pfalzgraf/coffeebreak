//! Drawable widgets that compose the animated timer screen.
//!
//! Each widget returns one or more [`Line`]s (styled text + display width) so the
//! renderer can centre them. The marquee pieces:
//!
//! * [`coffee_cup`] — a cup whose coffee level rises/falls with the phase, with a
//!   shimmering liquid surface and animated steam ([`steam_rows`] + [`cup_body`]).
//! * [`brew_splash`] — a pour-and-steam intro that fills the cup.
//! * [`big_time`] — the remaining time in large block digits.
//! * [`ring_gauge`] — a circular progress gauge, an alternative to `big_time`.
//! * [`progress_bar`] — a gradient bar.
//! * [`spinner`] / [`spinner_label`] — a compact activity spinner for waits.
//! * [`confetti`] — celebratory sparkles for the finale.

use unicode_width::UnicodeWidthStr;

use crate::render::Line;
use crate::theme::{Rgb, Theme};
use crate::ui::row_from_cells;

// ---------------------------------------------------------------------------
// Coffee cup
// ---------------------------------------------------------------------------

const CUP_W: usize = 22; // total widget width (margins + cup + handle)
const IW: usize = 16; // interior width
const IH: usize = 6; // interior height (rows of liquid space)
const STEAM_ROWS: usize = 3;
const INT_START: usize = 2; // first interior column

/// Render the coffee cup: an animated steam plume above the cup body.
///
/// * `fill` — coffee level, `0.0` (empty) ..= `1.0` (full).
/// * `steam` — steam intensity, `0.0` ..= `1.0`.
/// * `frame` — animation frame counter.
///
/// Composed from the two reusable halves [`steam_rows`] and [`cup_body`], so the
/// brewing splash can drive them independently. The result is always
/// `STEAM_ROWS + IH + 3` lines, each `CUP_W` columns wide.
pub fn coffee_cup(theme: &Theme, fill: f64, steam: f64, frame: usize) -> Vec<Line> {
    let mut lines = steam_rows(theme, steam, frame);
    lines.extend(cup_body(theme, fill, frame));
    lines
}

/// The steam plume rising off the cup: exactly `STEAM_ROWS` lines, each
/// `CUP_W` columns wide.
///
/// `steam` (`0.0..=1.0`) sets how many rows are active — steam dissipates
/// upward, so a cooler cup shows fewer, lower rows. `frame` drives an organic
/// per-wisp sway and out-of-phase flicker. Deterministic (no RNG) so it is safe
/// in replayable contexts. Every painted cell stays within the cup interior
/// (`INT_START..INT_START + IW`).
pub fn steam_rows(theme: &Theme, steam: f64, frame: usize) -> Vec<Line> {
    let p = &theme.palette;
    let steam = steam.clamp(0.0, 1.0);
    let active = (steam * STEAM_ROWS as f64).round() as usize;
    // Five wisp sources across the interior, each with its own phase so the
    // plume drifts rather than pulsing in lockstep.
    const BASES: [usize; 5] = [2, 5, 8, 11, 14];

    let mut lines = Vec::with_capacity(STEAM_ROWS);
    for sr in 0..STEAM_ROWS {
        let mut cells = vec![(' ', None); CUP_W];
        // Show only the lowest `active` rows (steam rises and thins out).
        if sr + active >= STEAM_ROWS {
            let height = (STEAM_ROWS - sr) as f64; // 1 lowest .. STEAM_ROWS top
            let fade = ((0.30 + 0.20 * sr as f64) * (0.5 + 0.5 * steam)).clamp(0.0, 1.0);
            for (i, &base) in BASES.iter().enumerate() {
                // Out-of-phase dissipation: each wisp blinks on its own cadence.
                if (frame / 2 + i + sr).is_multiple_of(3) {
                    continue;
                }
                let phase = i as f64 * 1.7;
                let sway =
                    (frame as f64 * 0.18 + phase + height * 0.6).sin() * (0.6 + 0.5 * height);
                let col =
                    (base as isize + sway.round() as isize).clamp(0, IW as isize - 1) as usize;
                // Thicker puffs near the surface; thin dots as it dissipates.
                let ch = if sr == 0 {
                    '·'
                } else if sr == STEAM_ROWS - 1 {
                    if (frame + i).is_multiple_of(2) {
                        '('
                    } else {
                        ')'
                    }
                } else {
                    '~'
                };
                cells[INT_START + col] = (ch, Some(p.steam.shade(fade)));
            }
        }
        lines.push(row_from_cells(theme, &cells));
    }
    lines
}

/// The cup itself — top rim, the liquid interior, bottom rim, and the saucer —
/// without any steam. `fill` (`0.0..=1.0`) sets the coffee level and `frame`
/// animates the shimmering surface. Always `IH + 3` lines of `CUP_W` columns.
pub fn cup_body(theme: &Theme, fill: f64, frame: usize) -> Vec<Line> {
    let p = &theme.palette;
    let fill = fill.clamp(0.0, 1.0);
    let mut lines = Vec::with_capacity(IH + 3);

    // --- top rim ---
    lines.push(rim(theme, '╭', '╮'));

    // --- interior rows ---
    let filled_rows = (fill * IH as f64).round() as usize;
    let first_liquid = IH - filled_rows; // topmost row index that holds liquid
    for r in 0..IH {
        let mut cells = vec![(' ', None); CUP_W];
        cells[0] = (' ', None);
        cells[1] = ('│', Some(p.cup));
        cells[INT_START + IW] = ('│', Some(p.cup));

        // Handle on the right for the middle rows.
        cells[INT_START + IW + 2] = match r {
            1 => ('⎞', Some(p.cup)),
            2 => ('⎟', Some(p.cup)),
            3 => ('⎠', Some(p.cup)),
            _ => (' ', None),
        };

        if filled_rows > 0 && r >= first_liquid {
            if r == first_liquid {
                // Shimmering surface.
                for x in 0..IW {
                    let wave = ((x as f64 * 0.7 + frame as f64 * 0.5).sin() * 0.5 + 0.5) * 0.55;
                    let color = p.coffee_top.lerp(p.cup, wave);
                    cells[INT_START + x] = ('~', Some(color));
                }
            } else {
                // Body: gradient deepening toward the bottom.
                let depth = (r - first_liquid) as f64 / filled_rows.max(1) as f64;
                let color = p.coffee_top.lerp(p.coffee_bottom, depth);
                for x in 0..IW {
                    cells[INT_START + x] = ('█', Some(color));
                }
            }
        }
        lines.push(row_from_cells(theme, &cells));
    }

    // --- bottom rim + saucer ---
    lines.push(rim(theme, '╰', '╯'));
    let mut saucer = vec![(' ', None); CUP_W];
    saucer[1] = ('╲', Some(p.cup.shade(0.8)));
    for x in 0..IW {
        saucer[INT_START + x] = ('_', Some(p.cup.shade(0.8)));
    }
    saucer[INT_START + IW] = ('╱', Some(p.cup.shade(0.8)));
    lines.push(row_from_cells(theme, &saucer));

    lines
}

/// A top/bottom rim row using the given corner glyphs.
fn rim(theme: &Theme, left: char, right: char) -> Line {
    let p = &theme.palette;
    let mut cells = vec![(' ', None); CUP_W];
    cells[1] = (left, Some(p.cup));
    for x in 0..IW {
        cells[INT_START + x] = ('─', Some(p.cup));
    }
    cells[INT_START + IW] = (right, Some(p.cup));
    row_from_cells(theme, &cells)
}

// ---------------------------------------------------------------------------
// Big block-digit clock
// ---------------------------------------------------------------------------

const FONT_H: usize = 5;

/// 5-row block font for digits and the separator.
fn glyph(c: char) -> [&'static str; FONT_H] {
    match c {
        '0' => ["████", "█  █", "█  █", "█  █", "████"],
        '1' => ["  █ ", " ██ ", "  █ ", "  █ ", " ███"],
        '2' => ["████", "   █", "████", "█   ", "████"],
        '3' => ["████", "   █", " ███", "   █", "████"],
        '4' => ["█  █", "█  █", "████", "   █", "   █"],
        '5' => ["████", "█   ", "████", "   █", "████"],
        '6' => ["████", "█   ", "████", "█  █", "████"],
        '7' => ["████", "   █", "  █ ", " █  ", " █  "],
        '8' => ["████", "█  █", "████", "█  █", "████"],
        '9' => ["████", "█  █", "████", "   █", "████"],
        ':' => [" ", "█", " ", "█", " "],
        _ => ["    ", "    ", "    ", "    ", "    "],
    }
}

/// Render `text` (e.g. `"24:59"`) as large block digits, 5 lines tall.
pub fn big_time(theme: &Theme, text: &str, color: Rgb) -> Vec<Line> {
    let mut rows: Vec<Vec<(char, Option<Rgb>)>> = vec![Vec::new(); FONT_H];
    for (i, ch) in text.chars().enumerate() {
        let g = glyph(ch);
        for (r, row) in rows.iter_mut().enumerate() {
            if i > 0 {
                row.push((' ', None));
            }
            // Subtle top-to-bottom shading for depth.
            let shade = 0.7 + 0.3 * (r as f64 / (FONT_H - 1) as f64);
            let c = color.shade(shade);
            for cell in g[r].chars() {
                if cell == '█' {
                    row.push(('█', Some(c)));
                } else {
                    row.push((' ', None));
                }
            }
        }
    }
    rows.iter()
        .map(|cells| row_from_cells(theme, cells))
        .collect()
}

// ---------------------------------------------------------------------------
// Progress bar
// ---------------------------------------------------------------------------

/// A gradient progress bar `width` cells wide; `elapsed` is `0.0..=1.0`.
pub fn progress_bar(theme: &Theme, elapsed: f64, width: usize, from: Rgb, to: Rgb) -> Line {
    let elapsed = elapsed.clamp(0.0, 1.0);
    let filled = (elapsed * width as f64).round() as usize;
    let mut cells = Vec::with_capacity(width);
    for i in 0..width {
        if i < filled {
            let t = if width > 1 {
                i as f64 / (width - 1) as f64
            } else {
                0.0
            };
            cells.push(('█', Some(from.lerp(to, t))));
        } else {
            cells.push(('░', Some(theme.palette.muted.shade(0.6))));
        }
    }
    row_from_cells(theme, &cells)
}

// ---------------------------------------------------------------------------
// Celebration
// ---------------------------------------------------------------------------

const SPARKLES: [char; 6] = ['✦', '✧', '·', '*', '✶', '°'];

/// A row of twinkling sparkles `width` wide, animated by `frame`. Deterministic
/// (no RNG) so it can't break workflow replay; the pattern shifts each frame.
pub fn confetti(theme: &Theme, width: usize, frame: usize) -> Line {
    let p = &theme.palette;
    let colors = [p.focus, p.short_break, p.long_break, p.accent, p.success];
    let mut cells = Vec::with_capacity(width);
    for x in 0..width {
        // A sparse, drifting pattern: a cell twinkles when this hash is 0.
        let h = (x.wrapping_mul(7) + frame.wrapping_mul(3)) % 9;
        if h < 2 {
            let ch = SPARKLES[(x + frame) % SPARKLES.len()];
            let color = colors[(x / 2 + frame) % colors.len()];
            cells.push((ch, Some(color)));
        } else {
            cells.push((' ', None));
        }
    }
    row_from_cells(theme, &cells)
}

// ---------------------------------------------------------------------------
// Brewing intro
// ---------------------------------------------------------------------------

/// Number of frames in the [`brew_splash`] intro animation (~1.4 s at 15 fps).
pub const BREW_FRAMES: usize = 21;

/// A short "brewing" intro: coffee pours into the cup, settles, then steams.
///
/// Returns the same line layout as [`coffee_cup`] (`STEAM_ROWS + IH + 3` lines of
/// `CUP_W` columns) so it can be positioned identically. A `frame` at or past
/// `BREW_FRAMES - 1` holds the finished, gently steaming cup. The three phases:
///
/// * **pour** (`0..=8`) — a stream falls down the steam region and the level rises;
/// * **settle** (`9..=14`) — the level tops off to full, the stream stops;
/// * **steam** (`15..`) — steam rises off the full cup.
pub fn brew_splash(theme: &Theme, frame: usize) -> Vec<Line> {
    let p = &theme.palette;
    let stream_col = INT_START + IW / 2;
    let pouring = frame <= 8;

    let (fill, steam) = if frame <= 8 {
        ((frame as f64 / 8.0) * 0.85, 0.0)
    } else if frame <= 14 {
        (0.85 + 0.15 * ((frame - 9) as f64 / 5.0), 0.0)
    } else {
        (1.0, ((frame - 15) as f64 / 5.0).min(1.0))
    };

    let mut lines: Vec<Line> = if pouring {
        // A falling stream down the steam region into the cup. The head descends
        // one row per frame; a spout caps the top of the stream.
        let head = (frame + 1).min(STEAM_ROWS);
        (0..STEAM_ROWS)
            .map(|sr| {
                let mut cells = vec![(' ', None); CUP_W];
                if sr + head >= STEAM_ROWS {
                    let glyph = if (frame + sr).is_multiple_of(2) {
                        '│'
                    } else {
                        '╎'
                    };
                    cells[stream_col] = (glyph, Some(p.coffee_top.shade(0.9)));
                }
                if sr == 0 {
                    cells[stream_col] = ('╧', Some(p.coffee_top));
                }
                row_from_cells(theme, &cells)
            })
            .collect()
    } else {
        steam_rows(theme, steam, frame)
    };

    lines.extend(cup_body(theme, fill, frame));
    lines
}

// ---------------------------------------------------------------------------
// Ring gauge
// ---------------------------------------------------------------------------

const RING_W: usize = 13; // odd, so there is a true centre column
const RING_H: usize = 7;

/// A circular progress gauge: a ring that fills clockwise from 12 o'clock as
/// `frac` (`0.0..=1.0`) grows, with `label` (e.g. `"57%"`) centred inside.
///
/// Always `RING_H` (7) lines of `RING_W` (13) columns — an alternative big
/// indicator to [`big_time`]. The fraction itself is the animation, so no frame
/// counter is needed.
pub fn ring_gauge(theme: &Theme, frac: f64, color: Rgb, label: &str) -> Vec<Line> {
    let p = &theme.palette;
    let frac = frac.clamp(0.0, 1.0);
    let cx = (RING_W as f64 - 1.0) / 2.0;
    let cy = (RING_H as f64 - 1.0) / 2.0;
    let empty = p.muted.shade(0.5);
    let label_row = RING_H / 2;

    // The angle of a grid cell on the ring, as a fraction `0.0..1.0` measured
    // clockwise from 12 o'clock — or `None` if the cell is off the ring. Cells
    // are roughly 2:1 (tall), so `x` is scaled to keep the ring round.
    let ring_t = |x: usize, y: usize| -> Option<f64> {
        let dx = (x as f64 - cx) * 0.5;
        let dy = y as f64 - cy;
        let dist = (dx * dx + dy * dy).sqrt();
        if !(2.0..=3.0).contains(&dist) {
            return None;
        }
        let theta = dx.atan2(-dy);
        let theta = if theta < 0.0 {
            theta + std::f64::consts::TAU
        } else {
            theta
        };
        Some(theta / std::f64::consts::TAU)
    };

    // The discrete grid's largest cell angle is below a full turn, so scale the
    // sweep to it: then `frac == 0.0` lights nothing and `frac == 1.0` lights the
    // whole ring (no permanently-lit top seam, no ~4% dead zone near the end).
    let mut t_max = 0.0_f64;
    for y in 0..RING_H {
        for x in 0..RING_W {
            if let Some(t) = ring_t(x, y) {
                t_max = t_max.max(t);
            }
        }
    }
    let threshold = frac * t_max + 1e-9;

    let mut lines = Vec::with_capacity(RING_H);
    for y in 0..RING_H {
        let mut cells = vec![(' ', None); RING_W];
        for (x, cell) in cells.iter_mut().enumerate() {
            if let Some(t) = ring_t(x, y) {
                *cell = if frac > 0.0 && t <= threshold {
                    ('●', Some(color))
                } else {
                    ('·', Some(empty))
                };
            }
        }
        // Drop the centred label over the (empty) middle row.
        if y == label_row {
            let lw = label.chars().count();
            let start = RING_W.saturating_sub(lw) / 2;
            for (i, ch) in label.chars().enumerate() {
                if let Some(cell) = cells.get_mut(start + i) {
                    *cell = (ch, Some(color));
                }
            }
        }
        lines.push(row_from_cells(theme, &cells));
    }
    lines
}

// ---------------------------------------------------------------------------
// Spinner
// ---------------------------------------------------------------------------

/// A compact activity spinner style. Both use width-1 glyphs only.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Spinner {
    /// Braille dots — smooth and the default.
    Dots,
    /// A simple bouncing pip (fallback for terminals without Braille).
    Bounce,
}

impl Spinner {
    /// The animation frames for this style.
    fn frames(self) -> &'static [char] {
        match self {
            Spinner::Dots => &['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'],
            Spinner::Bounce => &['.', 'o', 'O', 'o'],
        }
    }
}

/// One spinner glyph for `frame`, painted `color`. Always width 1.
pub fn spinner(theme: &Theme, style: Spinner, frame: usize, color: Rgb) -> Line {
    let frames = style.frames();
    let ch = frames[frame % frames.len()];
    row_from_cells(theme, &[(ch, Some(color))])
}

/// A spinner, a space, then a dimmed `label` (e.g. `"⠙ checking for updates…"`).
/// Total visible width is `2 + label`'s display width.
pub fn spinner_label(theme: &Theme, style: Spinner, frame: usize, label: &str, color: Rgb) -> Line {
    let frames = style.frames();
    let ch = frames[frame % frames.len()];
    let mut s = theme.paint(ch, color);
    s.push(' ');
    s.push_str(&theme.dim(label));
    Line::styled(s, 2 + label.width())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cup_lines_share_one_width() {
        let theme = Theme::resolve("coffee", true);
        let lines = coffee_cup(&theme, 0.5, 0.8, 0);
        let w = lines[0].width();
        assert_eq!(w, CUP_W);
        assert!(
            lines.iter().all(|l| l.width() == w),
            "all cup lines same width"
        );
    }

    #[test]
    fn big_time_is_five_rows_of_equal_width() {
        let theme = Theme::resolve("coffee", true);
        let lines = big_time(&theme, "24:59", Rgb(255, 255, 255));
        assert_eq!(lines.len(), FONT_H);
        let w = lines[0].width();
        assert!(lines.iter().all(|l| l.width() == w));
    }

    #[test]
    fn progress_bar_has_exact_width() {
        let theme = Theme::resolve("coffee", true);
        let bar = progress_bar(&theme, 0.5, 30, Rgb(0, 0, 0), Rgb(255, 255, 255));
        assert_eq!(bar.width(), 30);
    }

    #[test]
    fn steam_rows_are_constant_count_and_width() {
        let theme = Theme::resolve("coffee", true);
        for steam in [0.0, 0.5, 1.0] {
            for frame in 0..12 {
                let rows = steam_rows(&theme, steam, frame);
                assert_eq!(rows.len(), STEAM_ROWS);
                assert!(rows.iter().all(|l| l.width() == CUP_W));
            }
        }
    }

    #[test]
    fn brew_splash_matches_cup_layout_every_frame() {
        let theme = Theme::resolve("coffee", true);
        let cup_len = coffee_cup(&theme, 1.0, 1.0, 0).len();
        // Cover all phases plus a couple past the end (which should hold).
        for frame in 0..(BREW_FRAMES + 4) {
            let lines = brew_splash(&theme, frame);
            assert_eq!(lines.len(), cup_len, "frame {frame} line count");
            assert!(
                lines.iter().all(|l| l.width() == CUP_W),
                "frame {frame} width"
            );
        }
    }

    #[test]
    fn ring_gauge_has_fixed_dimensions() {
        let theme = Theme::resolve("coffee", true);
        for frac in [0.0, 0.25, 0.5, 0.99, 1.0, 2.0] {
            let lines = ring_gauge(&theme, frac, Rgb(1, 2, 3), "57%");
            assert_eq!(lines.len(), RING_H);
            assert!(lines.iter().all(|l| l.width() == RING_W));
        }
    }

    #[test]
    fn ring_gauge_fills_from_empty_to_full() {
        let theme = Theme::resolve("coffee", true);
        // Count the lit ('●') cells; pass no label so it can't be confused.
        let lit = |frac| {
            ring_gauge(&theme, frac, Rgb(1, 2, 3), "")
                .iter()
                .map(|l| l.as_str().matches('●').count())
                .sum::<usize>()
        };
        assert_eq!(lit(0.0), 0, "the ring must be empty at 0%");
        let full = lit(1.0);
        assert!(full > 0, "the ring must light at 100%");
        // The fill grows monotonically and still has headroom past 95% (i.e. no
        // large dead zone where the gauge looks full before it is).
        assert!(lit(0.5) < full);
        assert!(lit(0.95) < full, "95% should not already be visually full");
    }

    #[test]
    fn spinner_is_one_column_for_every_frame() {
        let theme = Theme::resolve("coffee", true);
        for style in [Spinner::Dots, Spinner::Bounce] {
            for frame in 0..20 {
                assert_eq!(spinner(&theme, style, frame, Rgb(9, 9, 9)).width(), 1);
            }
        }
    }

    #[test]
    fn spinner_label_width_is_two_plus_label() {
        let theme = Theme::resolve("coffee", true);
        let line = spinner_label(&theme, Spinner::Dots, 0, "checking", Rgb(9, 9, 9));
        assert_eq!(line.width(), 2 + "checking".len());
    }
}
