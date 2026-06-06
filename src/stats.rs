//! Daily statistics persisted to `~/.coffeebreak/stats.json`.
//!
//! The file is a small JSON object keyed by date so it stays human-readable and
//! trivially mergeable. Stats are best-effort: a corrupt file is reported and
//! treated as empty rather than blocking the timer.

use std::collections::BTreeMap;
use std::fs;

use anyhow::{Context, Result};
use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};

use crate::i18n::{I18n, Msg, Noun};
use crate::paths;

/// One day's tally.
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct DayStat {
    #[serde(default)]
    pub completed_pomodoros: u64,
    #[serde(default)]
    pub focus_minutes: u64,
}

/// The whole history, keyed by `YYYY-MM-DD`.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Stats {
    #[serde(default)]
    pub days: BTreeMap<String, DayStat>,
}

/// Today's date as a `YYYY-MM-DD` string (local time).
pub fn today() -> String {
    Local::now().format("%Y-%m-%d").to_string()
}

impl Stats {
    /// Load stats, returning defaults if the file is missing.
    pub fn load() -> Result<Stats> {
        let path = paths::stats_file()?;
        match fs::read_to_string(&path) {
            Ok(text) => serde_json::from_str(&text)
                .with_context(|| format!("failed to parse stats at {}", path.display())),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Stats::default()),
            Err(e) => {
                Err(e).with_context(|| format!("failed to read stats at {}", path.display()))
            }
        }
    }

    /// Like [`Stats::load`] but never fails: a corrupt or unreadable file is
    /// reported to stderr (localised) and treated as empty.
    pub fn load_or_default(i18n: &I18n) -> Stats {
        match Stats::load() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("coffeebreak: {}", i18n.tf(Msg::WarnStatsRead, &[("error", &format!("{e:#}"))]));
                Stats::default()
            }
        }
    }

    /// Persist to disk, creating the data directory if needed.
    pub fn save(&self) -> Result<()> {
        let dir = paths::data_dir()?;
        fs::create_dir_all(&dir)
            .with_context(|| format!("failed to create data dir {}", dir.display()))?;
        let path = paths::stats_file()?;
        let text = serde_json::to_string_pretty(self).context("failed to serialize stats")?;
        fs::write(&path, text)
            .with_context(|| format!("failed to write stats to {}", path.display()))
    }

    /// Credit one completed focus block of `minutes` to `date`.
    pub fn record_pomodoro(&mut self, minutes: u64, date: &str) {
        let entry = self.days.entry(date.to_string()).or_default();
        entry.completed_pomodoros += 1;
        entry.focus_minutes += minutes;
    }

    /// All-time totals: (pomodoros, focus minutes, active days).
    pub fn totals(&self) -> (u64, u64, usize) {
        let pomos = self.days.values().map(|d| d.completed_pomodoros).sum();
        let minutes = self.days.values().map(|d| d.focus_minutes).sum();
        (pomos, minutes, self.days.len())
    }

    /// Consecutive days with at least one pomodoro, counting back from `today`
    /// (inclusive). Returns 0 if `today` itself has no activity.
    pub fn streak(&self, today: NaiveDate) -> u64 {
        let mut streak = 0;
        let mut day = today;
        loop {
            let key = day.format("%Y-%m-%d").to_string();
            match self.days.get(&key) {
                Some(d) if d.completed_pomodoros > 0 => streak += 1,
                _ => break,
            }
            match day.pred_opt() {
                Some(prev) => day = prev,
                None => break,
            }
        }
        streak
    }

    /// The day with the most pomodoros, if any.
    pub fn best_day(&self) -> Option<(&String, &DayStat)> {
        self.days
            .iter()
            .filter(|(_, d)| d.completed_pomodoros > 0)
            .max_by_key(|(_, d)| d.completed_pomodoros)
    }

    /// Render the human-facing report to stdout, styled via `theme` and
    /// localised via `i18n`.
    pub fn print_summary(&self, theme: &crate::theme::Theme, i18n: &I18n) {
        let p = &theme.palette;

        let field = |label: &str, value: String| {
            println!("  {} {}", theme.bold(format!("{label:<16}"), p.accent), value);
        };

        println!("\n{}\n", theme.bold(i18n.t(Msg::StatsTitle), p.accent));

        let (pomos, minutes, days) = self.totals();
        if pomos == 0 {
            println!("  {}\n", theme.dim(i18n.t(Msg::StatsEmpty)));
            return;
        }

        let min_focus = i18n.t(Msg::MinFocus);
        let today_key = today();
        let today_stat = self.days.get(&today_key).copied().unwrap_or_default();

        field(
            i18n.t(Msg::StatsToday),
            format!(
                "{} · {} {min_focus}",
                i18n.count(today_stat.completed_pomodoros, Noun::Pomodoro),
                today_stat.focus_minutes
            ),
        );
        field(
            i18n.t(Msg::StatsAllTime),
            format!(
                "{} · {} {min_focus} {} {}",
                i18n.count(pomos, Noun::Pomodoro),
                minutes,
                i18n.t(Msg::Over),
                i18n.count(days as u64, Noun::Day),
            ),
        );

        if let Ok(today_date) = NaiveDate::parse_from_str(&today_key, "%Y-%m-%d") {
            field(i18n.t(Msg::StatsStreak), i18n.count(self.streak(today_date), Noun::Day));
        }
        if let Some((date, stat)) = self.best_day() {
            field(
                i18n.t(Msg::StatsBestDay),
                format!("{date} ({})", i18n.count(stat.completed_pomodoros, Noun::Pomodoro)),
            );
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_accumulates_per_day() {
        let mut s = Stats::default();
        s.record_pomodoro(25, "2026-06-05");
        s.record_pomodoro(25, "2026-06-05");
        s.record_pomodoro(50, "2026-06-06");
        assert_eq!(s.days["2026-06-05"].completed_pomodoros, 2);
        assert_eq!(s.days["2026-06-05"].focus_minutes, 50);
        assert_eq!(s.totals(), (3, 100, 2));
    }

    #[test]
    fn streak_counts_back_from_today() {
        let mut s = Stats::default();
        for d in ["2026-06-03", "2026-06-04", "2026-06-05"] {
            s.record_pomodoro(25, d);
        }
        let today = NaiveDate::from_ymd_opt(2026, 6, 5).unwrap();
        assert_eq!(s.streak(today), 3);

        // A gap breaks the streak.
        let today_gap = NaiveDate::from_ymd_opt(2026, 6, 7).unwrap();
        assert_eq!(s.streak(today_gap), 0);
    }
}
