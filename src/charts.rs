//! Chart widgets for the statistics dashboard.
//!
//! Like [`crate::widgets`], every function is pure and returns [`Line`]s (styled
//! text + display width) so the renderer and the inline dashboard can place them.
//! All colour comes from the [`Theme`]; all glyphs are display-width 1.
//!
//! Provided charts:
//! * [`sparkline`]   — a one-line trend.
//! * [`bar_chart`]   — vertical bars with sub-cell (eighth-block) resolution.
//! * [`heatmap`]     — a GitHub-style contribution grid.
//! * [`goal_bar`]    — a labelled progress bar toward a daily goal.

use crate::render::Line;
use crate::theme::{Rgb, Theme};
use crate::ui::row_from_cells as row;

/// Lower-eighth block glyphs, index `0..=8` (empty → full).
const EIGHTHS: [char; 9] = [' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

/// Map a value to a colour from `low` → `high` by its ratio to `max`.
fn heat(low: Rgb, high: Rgb, value: u64, max: u64) -> Rgb {
    if max == 0 || value == 0 {
        return low;
    }
    low.lerp(high, value as f64 / max as f64)
}

/// A single-line sparkline of `values`, scaled to its own maximum.
pub fn sparkline(theme: &Theme, values: &[u64], color: Rgb) -> Line {
    let max = values.iter().copied().max().unwrap_or(0);
    let cells: Vec<(char, Option<Rgb>)> = values
        .iter()
        .map(|&v| {
            let idx = if v == 0 || max == 0 {
                // A zero stays blank so an idle day is distinguishable from the
                // smallest non-zero one.
                0
            } else {
                // 1..=8 for any non-zero value so even small bars are visible.
                1 + ((v as f64 / max as f64) * 7.0).round() as usize
            };
            (EIGHTHS[idx.min(8)], Some(color))
        })
        .collect();
    row(theme, &cells)
}

/// A vertical bar chart `rows` tall. Each bar is one column; bars are separated
/// by a space. Sub-cell precision uses eighth blocks. Colour grades from `from`
/// (short bars) to `to` (tall bars).
pub fn bar_chart(theme: &Theme, values: &[u64], rows: usize, from: Rgb, to: Rgb) -> Vec<Line> {
    let rows = rows.max(1);
    let max = values.iter().copied().max().unwrap_or(0);
    let mut lines = Vec::with_capacity(rows);

    for r in 0..rows {
        // r counts from the top; the bottom row is rows-1.
        let from_bottom = rows - 1 - r;
        let mut cells: Vec<(char, Option<Rgb>)> = Vec::with_capacity(values.len() * 2);
        for (i, &v) in values.iter().enumerate() {
            if i > 0 {
                cells.push((' ', None));
            }
            let total_eighths = if max == 0 {
                0
            } else {
                ((v as f64 / max as f64) * (rows * 8) as f64).round() as usize
            };
            let cell_eighths = total_eighths.saturating_sub(from_bottom * 8).min(8);
            let color = (cell_eighths > 0).then(|| heat(from, to, v, max));
            cells.push((EIGHTHS[cell_eighths], color));
        }
        lines.push(row(theme, &cells));
    }
    lines
}

/// A GitHub-style contribution heatmap.
///
/// `cells` is a chronological run of daily counts; they are laid out into 7 rows
/// (one per weekday) and as many week-columns as needed, oldest on the left. The
/// first element should align to `first_weekday` (0 = the top row).
pub fn heatmap(
    theme: &Theme,
    counts: &[u64],
    first_weekday: usize,
    color: Rgb,
    empty: Rgb,
) -> Vec<Line> {
    let max = counts.iter().copied().max().unwrap_or(0);
    let total = first_weekday + counts.len();
    let weeks = total.div_ceil(7);

    let mut lines = Vec::with_capacity(7);
    for wd in 0..7 {
        let mut cells: Vec<(char, Option<Rgb>)> = Vec::with_capacity(weeks * 2);
        for week in 0..weeks {
            if week > 0 {
                cells.push((' ', None));
            }
            let slot = week * 7 + wd;
            if slot < first_weekday || slot >= total {
                cells.push((' ', None)); // padding outside the data range
            } else {
                let v = counts[slot - first_weekday];
                let c = if v == 0 {
                    empty
                } else {
                    heat(empty, color, v, max)
                };
                cells.push(('■', Some(c)));
            }
        }
        lines.push(row(theme, &cells));
    }
    lines
}

/// A labelled progress bar toward a daily goal: `done`/`goal`.
pub fn goal_bar(theme: &Theme, done: u64, goal: u64, width: usize, color: Rgb) -> Line {
    let width = width.max(1);
    let frac = if goal == 0 {
        0.0
    } else {
        (done as f64 / goal as f64).clamp(0.0, 1.0)
    };
    let filled = (frac * width as f64).round() as usize;
    let reached = goal > 0 && done >= goal;
    let bar_color = if reached {
        theme.palette.success
    } else {
        color
    };

    let mut cells = Vec::with_capacity(width);
    for i in 0..width {
        if i < filled {
            cells.push(('█', Some(bar_color)));
        } else {
            cells.push(('░', Some(theme.palette.muted.shade(0.6))));
        }
    }
    row(theme, &cells)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn theme() -> Theme {
        Theme::resolve("coffee", true)
    }

    #[test]
    fn sparkline_width_matches_values() {
        let s = sparkline(&theme(), &[0, 1, 5, 3, 8], Rgb(1, 2, 3));
        assert_eq!(s.width(), 5);
    }

    #[test]
    fn sparkline_keeps_zero_days_blank() {
        // A zero must render as the empty glyph, not the same ▁ as the
        // smallest non-zero value — otherwise idle days look active.
        let t = Theme::resolve("coffee", false); // no colour → raw glyphs
        let s = sparkline(&t, &[0, 1, 8], Rgb(1, 2, 3));
        let text: Vec<char> = s.as_str().chars().collect();
        assert_eq!(text[0], ' ', "zero should be blank, got {text:?}");
        assert_ne!(text[1], ' ', "non-zero must be visible, got {text:?}");
        assert_eq!(text[2], '█', "max should be full, got {text:?}");
    }

    #[test]
    fn bar_chart_dimensions() {
        let bars = bar_chart(&theme(), &[1, 2, 3, 4], 5, Rgb(0, 0, 0), Rgb(9, 9, 9));
        assert_eq!(bars.len(), 5);
        // 4 bars + 3 separators = 7 columns
        assert!(bars.iter().all(|l| l.width() == 7));
    }

    #[test]
    fn heatmap_is_seven_rows_of_equal_width() {
        let counts: Vec<u64> = (0..30).collect();
        let lines = heatmap(&theme(), &counts, 3, Rgb(9, 9, 9), Rgb(1, 1, 1));
        assert_eq!(lines.len(), 7);
        let w = lines[0].width();
        assert!(lines.iter().all(|l| l.width() == w));
    }

    #[test]
    fn goal_bar_exact_width_and_done_handling() {
        assert_eq!(goal_bar(&theme(), 0, 0, 20, Rgb(1, 1, 1)).width(), 20);
        assert_eq!(goal_bar(&theme(), 8, 4, 10, Rgb(1, 1, 1)).width(), 10); // over-goal clamps
    }

    #[test]
    fn empty_inputs_do_not_panic() {
        let _ = sparkline(&theme(), &[], Rgb(0, 0, 0));
        let _ = bar_chart(&theme(), &[], 4, Rgb(0, 0, 0), Rgb(1, 1, 1));
        let _ = heatmap(&theme(), &[], 0, Rgb(0, 0, 0), Rgb(1, 1, 1));
    }
}
