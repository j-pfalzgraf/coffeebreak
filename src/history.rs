//! Per-session history log (`~/.coffeebreak/history.jsonl`).
//!
//! Opt-in (`history = true` in the config; off by default so the data
//! directory layout is unchanged for existing users). When enabled, every
//! completed focus block appends **one JSON object per line**:
//!
//! ```json
//! {"ts":"2026-07-03T14:25:00+02:00","work_min":25,"label":"api-refactor","completed":true}
//! ```
//!
//! Append-only JSONL was chosen over rewriting a single document so the write
//! on the timer's hot path is a single `O_APPEND` syscall, an interrupted write
//! can at worst damage the final line (which the reader then skips), and the
//! file can be processed with standard line tools (`jq`, `grep`, `tail`).
//!
//! Reading mirrors [`crate::stats::Stats::load_or_default`]'s best-effort
//! stance: a missing file is an empty history and an unparsable line is
//! skipped, never a hard error.

use std::fs;
use std::io::Write;

use anyhow::{Context, Result};
use chrono::{DateTime, Local, SecondsFormat};
use serde::{Deserialize, Serialize};

use crate::i18n::{I18n, Msg, Noun};
use crate::paths;
use crate::theme::Theme;

/// One completed (or, in future shapes, abandoned) focus block.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HistoryEntry {
    /// RFC 3339 local timestamp of when the block finished.
    pub ts: String,
    /// Focus minutes actually credited for the block.
    pub work_min: u64,
    /// The session label, if one was set (`--label` / `--git-label`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Whether the block ran to completion. Always `true` for entries written
    /// by the current version; the field exists so later versions can also log
    /// abandoned blocks without changing the shape.
    pub completed: bool,
}

impl HistoryEntry {
    /// A completed entry stamped with the current local time.
    pub fn completed_now(work_min: u64, label: Option<String>) -> HistoryEntry {
        HistoryEntry {
            ts: Local::now().to_rfc3339_opts(SecondsFormat::Secs, false),
            work_min,
            label,
            completed: true,
        }
    }
}

/// Append one entry to the history log, creating the data directory and the
/// file (owner-only on Unix) as needed.
pub fn append(entry: &HistoryEntry) -> Result<()> {
    let dir = paths::data_dir()?;
    fs::create_dir_all(&dir)
        .with_context(|| format!("failed to create data dir {}", dir.display()))?;
    let path = paths::history_file()?;

    let mut options = fs::OpenOptions::new();
    options.create(true).append(true);
    #[cfg(unix)]
    {
        // Like the stats file: personal activity data is created owner-only.
        // (Only applied at creation; an existing file keeps its permissions.)
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options
        .open(&path)
        .with_context(|| format!("failed to open history log {}", path.display()))?;

    let line = serde_json::to_string(entry).context("failed to serialize history entry")?;
    writeln!(file, "{line}")
        .with_context(|| format!("failed to append to history log {}", path.display()))
}

/// Load the whole history, oldest first. Best-effort by design: a missing file
/// is an empty history, and any line that does not parse (a torn final line
/// after a crash, or a future format extension) is skipped.
pub fn load_or_empty() -> Vec<HistoryEntry> {
    let Ok(path) = paths::history_file() else {
        return Vec::new();
    };
    match fs::read_to_string(&path) {
        Ok(text) => parse_lines(&text),
        Err(_) => Vec::new(),
    }
}

/// Parse JSONL text into entries, skipping blank and malformed lines.
fn parse_lines(text: &str) -> Vec<HistoryEntry> {
    text.lines()
        .filter(|l| !l.trim().is_empty())
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect()
}

/// Render the `coffeebreak history` table: the last `limit` sessions (0 = all),
/// oldest first, localised via `i18n` and styled via `theme`.
pub fn print(theme: &Theme, i18n: &I18n, limit: usize) {
    let p = &theme.palette;
    println!("\n{}\n", theme.bold(i18n.t(Msg::HistoryTitle), p.accent));

    let entries = load_or_empty();
    if entries.is_empty() {
        println!("  {}\n", theme.dim(i18n.t(Msg::HistoryEmpty)));
        return;
    }

    let shown = if limit == 0 {
        &entries[..]
    } else {
        &entries[entries.len().saturating_sub(limit)..]
    };

    // Size the columns to their localised headers ("2026-07-03 14:25" is 16).
    let when_h = i18n.t(Msg::HistoryColWhen);
    let min_h = i18n.t(Msg::HistoryColMinutes);
    let label_h = i18n.t(Msg::HistoryColLabel);
    let when_w = when_h.chars().count().max(16);
    let min_w = min_h.chars().count().max(3);

    println!(
        "    {} {} {}",
        theme.bold(pad(when_h, when_w), p.accent),
        theme.bold(pad_left(min_h, min_w), p.accent),
        theme.bold(label_h, p.accent),
    );
    for e in shown {
        let (glyph, color) = if e.completed {
            ("✓", p.success)
        } else {
            ("·", p.muted)
        };
        println!(
            "  {} {} {} {}",
            theme.paint(glyph, color),
            pad(&display_ts(&e.ts), when_w),
            pad_left(&e.work_min.to_string(), min_w),
            theme.dim(e.label.as_deref().unwrap_or("")),
        );
    }
    println!(
        "\n  {}\n",
        theme.dim(i18n.count(shown.len() as u64, Noun::Pomodoro))
    );
}

/// A compact `YYYY-MM-DD HH:MM` rendering of a stored RFC 3339 timestamp,
/// falling back to the raw string if it does not parse.
fn display_ts(ts: &str) -> String {
    DateTime::parse_from_rfc3339(ts)
        .map(|t| t.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_else(|_| ts.to_string())
}

/// Pad `s` on the right to `width` characters.
fn pad(s: &str, width: usize) -> String {
    format!("{s:<width$}")
}

/// Pad `s` on the left to `width` characters.
fn pad_left(s: &str, width: usize) -> String {
    format!("{s:>width$}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entry_round_trips_through_jsonl() {
        let entry = HistoryEntry {
            ts: "2026-07-03T14:25:00+02:00".to_string(),
            work_min: 25,
            label: Some("api-refactor".to_string()),
            completed: true,
        };
        let line = serde_json::to_string(&entry).unwrap();
        let parsed = parse_lines(&line);
        assert_eq!(parsed, vec![entry]);
    }

    #[test]
    fn label_is_omitted_when_absent() {
        let entry = HistoryEntry {
            ts: "2026-07-03T14:25:00+02:00".to_string(),
            work_min: 25,
            label: None,
            completed: true,
        };
        let line = serde_json::to_string(&entry).unwrap();
        assert!(!line.contains("label"), "unexpected label field: {line}");
        assert_eq!(parse_lines(&line)[0].label, None);
    }

    #[test]
    fn malformed_and_blank_lines_are_skipped() {
        let text = concat!(
            "{\"ts\":\"2026-07-01T09:00:00+02:00\",\"work_min\":25,\"completed\":true}\n",
            "\n",
            "{ torn line after a cra",
            "\n",
            "{\"ts\":\"2026-07-02T10:00:00+02:00\",\"work_min\":50,\"completed\":true}\n",
        );
        let parsed = parse_lines(text);
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].work_min, 25);
        assert_eq!(parsed[1].work_min, 50);
    }

    #[test]
    fn completed_now_stamps_a_parseable_rfc3339_time() {
        let e = HistoryEntry::completed_now(25, None);
        assert!(DateTime::parse_from_rfc3339(&e.ts).is_ok(), "ts: {}", e.ts);
        assert!(e.completed);
    }

    #[test]
    fn display_ts_formats_and_falls_back() {
        assert_eq!(display_ts("2026-07-03T14:25:00+02:00"), "2026-07-03 14:25");
        assert_eq!(display_ts("not a timestamp"), "not a timestamp");
    }
}
