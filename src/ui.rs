//! Shared low-level building blocks for composing styled terminal frames.
//!
//! Everything that draws — the live timer [`crate::widgets`], the statistics
//! [`crate::charts`], and the screen composition in [`crate::app`] — ultimately
//! emits [`Line`]s (styled text plus a true display width). Two primitives are
//! shared here so that logic lives in exactly one place (DRY):
//!
//! * [`row_from_cells`] builds a [`Line`] from a row of individually coloured,
//!   single-column glyphs — the natural model for the cup, the charts, and the
//!   block digits.
//! * [`LineBuf`] incrementally appends differently-styled spans to one line while
//!   tracking the visible width, for status lines and headers that mix colours.
//!
//! Both take a [`Theme`] so colour degrades to plain text automatically when
//! colour is disabled, and neither knows anything about positioning — that is the
//! renderer's job.

use crossterm::{cursor, execute};
use unicode_width::UnicodeWidthStr;

use crate::render::Line;
use crate::theme::{Rgb, Theme};

/// Hides the terminal cursor for the lifetime of an animated reveal and restores
/// it on drop — on a normal return, an early `return`, or a panic-unwind. Shared
/// by every inline (non-alternate-screen) animation: the statistics dashboard
/// ([`crate::stats`]) and the achievements board ([`crate::achievements`]).
pub(crate) struct CursorGuard;

impl CursorGuard {
    /// Hide the cursor until the returned guard is dropped.
    pub(crate) fn hide() -> CursorGuard {
        let _ = execute!(std::io::stdout(), cursor::Hide);
        CursorGuard
    }
}

impl Drop for CursorGuard {
    fn drop(&mut self) {
        let _ = execute!(std::io::stdout(), cursor::Show);
    }
}

/// Build a line from individual cells, each an (optional-coloured) character.
///
/// Assumes every glyph is one display column. This holds for all glyphs used by
/// coffeebreak under `unicode-width`'s default (ambiguous-width treated as
/// narrow), which matches standard Western terminals. In a terminal configured to
/// render East-Asian *ambiguous* width as 2 (e.g. some CJK locales), a
/// box-drawing frame would misalign; such setups should use `--plain`.
pub fn row_from_cells(theme: &Theme, cells: &[(char, Option<Rgb>)]) -> Line {
    let mut s = String::with_capacity(cells.len() * 4);
    for (ch, color) in cells {
        match color {
            Some(rgb) => s.push_str(&theme.paint(*ch, *rgb)),
            None => s.push(*ch),
        }
    }
    Line::styled(s, cells.len())
}

/// Build a horizontal bar `width` cells wide: the first `filled` cells are
/// solid blocks coloured per cell by `fill`, the rest are the shared dim track.
///
/// This is the one place the bar look (`█`/`░` and the track shade) is defined;
/// the timer's gradient progress bar and the stats goal bar both delegate here
/// and differ only in their fill-colour function.
pub fn bar_line(theme: &Theme, filled: usize, width: usize, fill: impl Fn(usize) -> Rgb) -> Line {
    let track = theme.palette.muted.shade(0.6);
    let mut cells = Vec::with_capacity(width);
    for i in 0..width {
        if i < filled {
            cells.push(('█', Some(fill(i))));
        } else {
            cells.push(('░', Some(track)));
        }
    }
    row_from_cells(theme, &cells)
}

/// A small builder for a single styled line that tracks visible width as spans
/// are appended.
///
/// Each `push`-style method appends a differently-styled span and advances the
/// recorded display width (excluding the zero-width ANSI escapes), so the
/// finished [`Line`] can be centred correctly by the renderer. Methods return
/// `&mut Self` so calls can be chained.
#[derive(Debug, Default)]
pub struct LineBuf {
    s: String,
    w: usize,
}

impl LineBuf {
    /// An empty builder.
    pub fn new() -> LineBuf {
        LineBuf {
            s: String::new(),
            w: 0,
        }
    }

    /// Append unstyled text. The `theme` argument is accepted for symmetry with
    /// the other span methods (so a caller can swap styles freely) but unused.
    pub fn plain(&mut self, _theme: &Theme, text: &str) -> &mut Self {
        self.s.push_str(text);
        self.w += text.width();
        self
    }

    /// Append `text` painted in `rgb`.
    pub fn color(&mut self, theme: &Theme, text: &str, rgb: Rgb) -> &mut Self {
        self.s.push_str(&theme.paint(text, rgb));
        self.w += text.width();
        self
    }

    /// Append bold `text` in `rgb`.
    pub fn bold(&mut self, theme: &Theme, text: &str, rgb: Rgb) -> &mut Self {
        self.s.push_str(&theme.bold(text, rgb));
        self.w += text.width();
        self
    }

    /// Append dimmed/muted `text`.
    pub fn dim(&mut self, theme: &Theme, text: impl AsRef<str>) -> &mut Self {
        let t = text.as_ref();
        self.s.push_str(&theme.dim(t));
        self.w += t.width();
        self
    }

    /// The accumulated visible width so far.
    pub fn width(&self) -> usize {
        self.w
    }

    /// Finish, yielding a [`Line`] with the tracked width.
    pub fn into_line(self) -> Line {
        Line::styled(self.s, self.w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cells_become_a_line_of_matching_width() {
        let theme = Theme::resolve("coffee", true);
        let line = row_from_cells(
            &theme,
            &[('a', None), ('b', Some(Rgb(1, 2, 3))), ('c', None)],
        );
        assert_eq!(line.width(), 3);
    }

    #[test]
    fn linebuf_tracks_visible_width_excluding_ansi() {
        let theme = Theme::resolve("coffee", true);
        let mut buf = LineBuf::new();
        buf.bold(&theme, "ab", Rgb(1, 2, 3))
            .plain(&theme, "  ")
            .dim(&theme, "cd");
        // 2 + 2 + 2 visible columns, regardless of the embedded escapes.
        assert_eq!(buf.width(), 6);
        let line = buf.into_line();
        assert_eq!(line.width(), 6);
        // Colour is enabled, so the styled text carries escapes but the width
        // stays truthful.
        assert!(line.as_str().contains("\x1b["));
    }

    #[test]
    fn linebuf_is_plain_when_colour_disabled() {
        let theme = Theme::resolve("coffee", false);
        let mut buf = LineBuf::new();
        buf.bold(&theme, "hi", Rgb(9, 9, 9));
        let line = buf.into_line();
        assert_eq!(line.as_str(), "hi");
        assert_eq!(line.width(), 2);
    }
}
