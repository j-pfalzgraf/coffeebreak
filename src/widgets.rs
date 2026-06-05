//! Drawable widgets that compose the animated timer screen.
//!
//! Each widget returns one or more [`Line`]s (styled text + display width) so the
//! renderer can centre them. The marquee pieces:
//!
//! * [`coffee_cup`] — a cup whose coffee level rises/falls with the phase, with a
//!   shimmering liquid surface and animated steam.
//! * [`big_time`] — the remaining time in large block digits.
//! * [`progress_bar`] — a gradient bar.
//! * [`confetti`] — celebratory sparkles for the finale.

use crate::render::Line;
use crate::theme::{Rgb, Theme};

/// Build a line from individual cells, each an (optional-coloured) character.
///
/// Assumes every glyph is one display column. This holds for all glyphs used
/// here under `unicode-width`'s default (ambiguous-width treated as narrow),
/// which matches standard Western terminals. In a terminal configured to render
/// East-Asian *ambiguous* width as 2 (e.g. some CJK locales), the box-drawing
/// frame would misalign; such setups should use `--plain`.
fn row_from_cells(theme: &Theme, cells: &[(char, Option<Rgb>)]) -> Line {
    let mut s = String::with_capacity(cells.len() * 4);
    for (ch, color) in cells {
        match color {
            Some(rgb) => s.push_str(&theme.paint(*ch, *rgb)),
            None => s.push(*ch),
        }
    }
    Line::styled(s, cells.len())
}

// ---------------------------------------------------------------------------
// Coffee cup
// ---------------------------------------------------------------------------

const CUP_W: usize = 22; // total widget width (margins + cup + handle)
const IW: usize = 16; // interior width
const IH: usize = 6; // interior height (rows of liquid space)
const STEAM_ROWS: usize = 3;
const INT_START: usize = 2; // first interior column

/// Render the coffee cup.
///
/// * `fill` — coffee level, `0.0` (empty) ..= `1.0` (full).
/// * `steam` — steam intensity, `0.0` ..= `1.0`.
/// * `frame` — animation frame counter.
pub fn coffee_cup(theme: &Theme, fill: f64, steam: f64, frame: usize) -> Vec<Line> {
    let p = &theme.palette;
    let fill = fill.clamp(0.0, 1.0);
    let steam = steam.clamp(0.0, 1.0);
    let mut lines = Vec::with_capacity(STEAM_ROWS + IH + 3);

    // --- steam: rises above the rim, more rows when hotter ---
    let active_steam = (steam * STEAM_ROWS as f64).round() as usize;
    let wisp_cols = [INT_START + 3, INT_START + 8, INT_START + 13];
    for sr in 0..STEAM_ROWS {
        let mut cells = vec![(' ', None); CUP_W];
        // The lowest `active_steam` rows are shown (steam dissipates upward).
        if sr >= STEAM_ROWS - active_steam {
            // Higher rows fade out.
            let fade = 0.45 + 0.18 * sr as f64;
            for (i, &col) in wisp_cols.iter().enumerate() {
                let ch = if (frame + sr + i) % 2 == 0 { '(' } else { ')' };
                cells[col] = (ch, Some(p.steam.shade(fade)));
            }
        }
        lines.push(row_from_cells(theme, &cells));
    }

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
    rows.iter().map(|cells| row_from_cells(theme, cells)).collect()
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
            let t = if width > 1 { i as f64 / (width - 1) as f64 } else { 0.0 };
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cup_lines_share_one_width() {
        let theme = Theme::resolve("coffee", true);
        let lines = coffee_cup(&theme, 0.5, 0.8, 0);
        let w = lines[0].width();
        assert_eq!(w, CUP_W);
        assert!(lines.iter().all(|l| l.width() == w), "all cup lines same width");
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
}
