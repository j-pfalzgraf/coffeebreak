//! A small, flicker-free line renderer.
//!
//! Animation needs smooth repaints without tearing. Rather than a full cell
//! grid, we render each frame as a list of pre-styled lines and diff against the
//! previous frame: only lines that actually changed are repainted (cursor moved,
//! line cleared, line rewritten). Static content — header, quote, hints — is
//! left untouched, so there is no flicker and very little terminal I/O.
//!
//! A [`Line`] couples styled text (which may contain zero-width ANSI escapes)
//! with its true display width, so the compositor can centre it correctly.

use std::io::{self, Write};

use crossterm::style::{Print, ResetColor};
use crossterm::{cursor, queue, terminal};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

/// A styled line plus its visible width (excluding ANSI escapes).
#[derive(Debug, Clone)]
pub struct Line {
    text: String,
    width: usize,
}

impl Line {
    /// A line whose styled `text` renders `width` visible columns.
    pub fn styled(text: impl Into<String>, width: usize) -> Line {
        Line { text: text.into(), width }
    }

    /// A plain (unstyled) line; width is measured from the text.
    pub fn plain(text: impl Into<String>) -> Line {
        let text = text.into();
        let width = text.width();
        Line { text, width }
    }

    /// An empty spacer line.
    pub fn blank() -> Line {
        Line { text: String::new(), width: 0 }
    }

    pub fn width(&self) -> usize {
        self.width
    }
}

/// A logical frame: an ordered stack of lines, not yet positioned on screen.
#[derive(Debug, Default)]
pub struct Frame {
    lines: Vec<Line>,
}

impl Frame {
    pub fn new() -> Frame {
        Frame { lines: Vec::new() }
    }

    pub fn push(&mut self, line: Line) {
        self.lines.push(line);
    }

    pub fn push_blank(&mut self) {
        self.lines.push(Line::blank());
    }

    pub fn extend<I: IntoIterator<Item = Line>>(&mut self, lines: I) {
        self.lines.extend(lines);
    }

    pub fn len(&self) -> usize {
        self.lines.len()
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    /// Position the frame within a `width`×`height` area: vertically centred,
    /// each line horizontally centred. Returns exactly `height` final strings
    /// (left-padded with spaces) so the diff has a stable line count.
    pub fn position(&self, width: usize, height: usize) -> Vec<String> {
        let content = self.lines.len().min(height);
        let top = (height.saturating_sub(content)) / 2;

        let mut out = Vec::with_capacity(height);
        for _ in 0..top {
            out.push(String::new());
        }
        for line in self.lines.iter().take(content) {
            let pad = width.saturating_sub(line.width) / 2;
            let composed = format!("{}{}", " ".repeat(pad), line.text);
            // Clip to the terminal width: a line wider than the screen would wrap
            // onto the next row and desync the line-based diff renderer.
            out.push(clip_to_width(&composed, width));
        }
        while out.len() < height {
            out.push(String::new());
        }
        out
    }
}

/// Truncate a (possibly ANSI-styled) string to at most `max` display columns,
/// passing escape sequences through without counting them. Appends a reset if
/// the string was actually cut, so a clipped colour can't bleed onto the rest
/// of the screen.
fn clip_to_width(s: &str, max: usize) -> String {
    let mut out = String::with_capacity(s.len());
    let mut width = 0usize;
    let mut chars = s.chars().peekable();
    let mut truncated = false;

    while let Some(c) = chars.next() {
        if c == '\x1b' {
            // Pass through a CSI escape sequence verbatim (ESC [ ... final).
            out.push(c);
            while let Some(&next) = chars.peek() {
                out.push(next);
                chars.next();
                // The final byte of a CSI sequence is in 0x40..=0x7E (e.g. 'm').
                if next != '[' && ('\u{40}'..='\u{7e}').contains(&next) {
                    break;
                }
            }
            continue;
        }
        let cw = UnicodeWidthChar::width(c).unwrap_or(0);
        if width + cw > max {
            truncated = true;
            break;
        }
        out.push(c);
        width += cw;
    }
    if truncated {
        out.push_str("\x1b[0m");
    }
    out
}

/// Owns the output stream and the previously drawn frame for diffing.
pub struct Renderer<W: Write> {
    out: W,
    prev: Vec<String>,
}

impl<W: Write> Renderer<W> {
    pub fn new(out: W) -> Renderer<W> {
        Renderer { out, prev: Vec::new() }
    }

    /// Forget the cached frame so the next `present` repaints everything
    /// (used after a resize or when returning from a sub-screen).
    pub fn invalidate(&mut self) {
        self.prev.clear();
    }

    /// Borrow the underlying writer (used in tests to inspect output).
    pub fn writer(&self) -> &W {
        &self.out
    }

    /// Clear the whole screen and drop the cache (e.g. after a resize, so stale
    /// rows from a larger previous frame don't linger).
    pub fn clear(&mut self) -> io::Result<()> {
        queue!(self.out, terminal::Clear(terminal::ClearType::All))?;
        self.out.flush()?;
        self.prev.clear();
        Ok(())
    }

    /// Draw `lines`, repainting only those that differ from the last frame.
    pub fn present(&mut self, lines: &[String]) -> io::Result<()> {
        for (y, line) in lines.iter().enumerate() {
            let changed = self.prev.get(y).map(|p| p != line).unwrap_or(true);
            if changed {
                queue!(
                    self.out,
                    cursor::MoveTo(0, y as u16),
                    terminal::Clear(terminal::ClearType::CurrentLine),
                    Print(line),
                    ResetColor
                )?;
            }
        }
        self.out.flush()?;
        self.prev = lines.to_vec();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_centres_vertically_and_horizontally() {
        let mut f = Frame::new();
        f.push(Line::plain("ab")); // width 2
        let out = f.position(6, 3);
        assert_eq!(out.len(), 3);
        // 3 rows, 1 content row -> top pad = 1
        assert_eq!(out[0], "");
        assert_eq!(out[1], "  ab"); // (6-2)/2 = 2 leading spaces
        assert_eq!(out[2], "");
    }

    #[test]
    fn clip_respects_width_and_passes_ansi() {
        // plain truncation
        assert_eq!(clip_to_width("abcdef", 3), "abc\x1b[0m");
        // within width: unchanged, no reset appended
        assert_eq!(clip_to_width("abc", 5), "abc");
        // ANSI escapes don't count toward width; the visible text is clipped
        let styled = "\x1b[31mabcdef\x1b[0m";
        let out = clip_to_width(styled, 3);
        assert!(out.starts_with("\x1b[31m"));
        assert!(out.contains("abc"));
        assert!(!out.contains("def"));
    }

    #[test]
    fn position_never_exceeds_width() {
        let mut f = Frame::new();
        f.push(Line::plain("this line is definitely too long for the width"));
        let out = f.position(10, 3);
        for line in &out {
            // visible width (no ANSI here) must be <= 10
            assert!(line.chars().count() <= 10 + 4, "line too wide: {line:?}");
        }
    }

    #[test]
    fn present_only_redraws_changed_lines() {
        let mut r = Renderer::new(Vec::<u8>::new());
        r.present(&["a".into(), "b".into()]).unwrap();
        let first = r.writer().len();
        assert!(first > 0);
        // same frame again -> no new bytes written
        r.present(&["a".into(), "b".into()]).unwrap();
        assert_eq!(r.writer().len(), first, "unchanged frame should not redraw");
        // a changed line -> more bytes written
        r.present(&["a".into(), "c".into()]).unwrap();
        assert!(r.writer().len() > first, "changed frame should redraw");
    }
}
