//! Daily statistics persisted to `~/.coffeebreak/stats.json`.
//!
//! The file is a small JSON object keyed by date so it stays human-readable and
//! trivially mergeable. Stats are best-effort: a corrupt file is reported and
//! treated as empty rather than blocking the timer.

use std::collections::BTreeMap;
use std::fs;
use std::io::{IsTerminal, Write};
use std::time::Duration;

use anyhow::{Context, Result};
use chrono::{Datelike, Local, NaiveDate};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{cursor, execute};
use serde::{Deserialize, Serialize};

use crate::charts;
use crate::fsutil::{self, FileKind};
use crate::i18n::{I18n, Msg, Noun};
use crate::paths;
use crate::theme::Theme;
use crate::ui::CursorGuard;

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

/// Move an unparsable stats file aside so a later save can't destroy it.
/// Returns the backup path on success; best-effort (`None` on any failure —
/// e.g. the file was unreadable rather than corrupt, or the rename failed).
fn quarantine_corrupt_stats() -> Option<String> {
    let path = paths::stats_file().ok()?;
    if !path.exists() {
        return None;
    }
    let backup = path.with_extension("json.corrupt");
    fs::rename(&path, &backup).ok()?;
    Some(backup.display().to_string())
}

impl Stats {
    /// Load stats, returning defaults if the file is missing.
    pub fn load() -> Result<Stats> {
        let path = paths::stats_file()?;
        match fs::read_to_string(&path) {
            Ok(text) => serde_json::from_str(&text)
                .with_context(|| format!("failed to parse stats at {}", path.display())),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Stats::default()),
            Err(e) => Err(e).with_context(|| format!("failed to read stats at {}", path.display())),
        }
    }

    /// Like [`Stats::load`] but never fails: a corrupt or unreadable file is
    /// reported to stderr (localised) and treated as empty.
    ///
    /// A file that exists but does not parse is first moved aside to
    /// `stats.json.corrupt` — otherwise the next [`Stats::save`] would silently
    /// overwrite the user's entire history with the empty fallback.
    pub fn load_or_default(i18n: &I18n) -> Stats {
        match Stats::load() {
            Ok(s) => s,
            Err(e) => {
                eprintln!(
                    "coffeebreak: {}",
                    i18n.tf(Msg::WarnStatsRead, &[("error", &format!("{e:#}"))])
                );
                if let Some(backup) = quarantine_corrupt_stats() {
                    eprintln!(
                        "coffeebreak: {}",
                        i18n.tf(Msg::WarnStatsQuarantined, &[("path", &backup)])
                    );
                }
                Stats::default()
            }
        }
    }

    /// Persist to disk (atomically), creating the data directory if needed.
    pub fn save(&self) -> Result<()> {
        let dir = paths::data_dir()?;
        fs::create_dir_all(&dir)
            .with_context(|| format!("failed to create data dir {}", dir.display()))?;
        let path = paths::stats_file()?;
        let text = serde_json::to_string_pretty(self).context("failed to serialize stats")?;
        fsutil::write_atomic(&path, &text, FileKind::Private)
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

    /// The stats for a specific date (zeroed if absent).
    pub fn day(&self, date: NaiveDate) -> DayStat {
        self.days
            .get(&date.format("%Y-%m-%d").to_string())
            .copied()
            .unwrap_or_default()
    }

    /// The last `n` days up to and including `today`, oldest first, with absent
    /// days filled as zero — ready to feed the charts.
    pub fn last_n_days(&self, n: usize, today: NaiveDate) -> Vec<(NaiveDate, DayStat)> {
        let mut out = Vec::with_capacity(n);
        for i in (0..n).rev() {
            if let Some(d) = today.checked_sub_days(chrono::Days::new(i as u64)) {
                out.push((d, self.day(d)));
            }
        }
        out
    }

    /// The longest run of consecutive days with at least one pomodoro, anywhere
    /// in the history.
    pub fn longest_streak(&self) -> u64 {
        let mut dates: Vec<NaiveDate> = self
            .days
            .iter()
            .filter(|(_, s)| s.completed_pomodoros > 0)
            .filter_map(|(k, _)| NaiveDate::parse_from_str(k, "%Y-%m-%d").ok())
            .collect();
        dates.sort_unstable();

        let mut best = 0u64;
        let mut current = 0u64;
        let mut prev: Option<NaiveDate> = None;
        for d in dates {
            current = match prev {
                Some(p) if p.succ_opt() == Some(d) => current + 1,
                Some(p) if p == d => current, // de-dup guard
                _ => 1,
            };
            best = best.max(current);
            prev = Some(d);
        }
        best
    }

    /// A machine-readable JSON report: a computed summary plus the full per-day
    /// history. Stable shape for scripts and dashboards.
    pub fn to_json(&self) -> String {
        #[derive(Serialize)]
        struct BestDay {
            date: String,
            pomodoros: u64,
            focus_minutes: u64,
        }
        #[derive(Serialize)]
        struct Summary {
            today_pomodoros: u64,
            today_focus_minutes: u64,
            total_pomodoros: u64,
            total_focus_minutes: u64,
            active_days: usize,
            current_streak: u64,
            longest_streak: u64,
            best_day: Option<BestDay>,
        }
        #[derive(Serialize)]
        struct Report<'a> {
            generated: String,
            summary: Summary,
            days: &'a BTreeMap<String, DayStat>,
        }

        let today = Local::now().date_naive();
        let (total_pomodoros, total_focus_minutes, active_days) = self.totals();
        let today_stat = self.day(today);
        let report = Report {
            generated: today.format("%Y-%m-%d").to_string(),
            summary: Summary {
                today_pomodoros: today_stat.completed_pomodoros,
                today_focus_minutes: today_stat.focus_minutes,
                total_pomodoros,
                total_focus_minutes,
                active_days,
                current_streak: self.streak(today),
                longest_streak: self.longest_streak(),
                best_day: self.best_day().map(|(date, s)| BestDay {
                    date: date.clone(),
                    pomodoros: s.completed_pomodoros,
                    focus_minutes: s.focus_minutes,
                }),
            },
            days: &self.days,
        };
        serde_json::to_string_pretty(&report).unwrap_or_else(|_| "{}".to_string())
    }

    /// A CSV report, one row per recorded day (chronological), with a header.
    pub fn to_csv(&self) -> String {
        let mut out = String::from("date,completed_pomodoros,focus_minutes\n");
        for (date, s) in &self.days {
            out.push_str(&format!(
                "{date},{},{}\n",
                s.completed_pomodoros, s.focus_minutes
            ));
        }
        out
    }

    /// Render the statistics dashboard to stdout, styled via `theme`, localised
    /// via `i18n`, with an optional daily `goal`.
    ///
    /// On a colour-capable, wide-enough terminal the charts "grow in" with a
    /// short reveal animation; otherwise the final dashboard is printed once.
    pub fn print_summary(&self, theme: &Theme, i18n: &I18n, goal: u64) {
        let p = &theme.palette;
        let (pomos, _, _) = self.totals();
        if pomos == 0 {
            println!("\n{}\n", theme.bold(i18n.t(Msg::StatsTitle), p.accent));
            println!("  {}\n", theme.dim(i18n.t(Msg::StatsEmpty)));
            return;
        }

        let today_date = Local::now().date_naive();
        let (width, height) = crossterm::terminal::size().unwrap_or((80, 24));
        let final_dashboard = self.dashboard_lines(theme, i18n, goal, today_date, 1.0);
        // Animate only when the whole dashboard fits on screen: the in-place
        // repaint walks back up with MoveToPreviousLine, which clamps at the top
        // row — if the output scrolled, later frames would repaint wrong lines.
        let animate = theme.color()
            && std::io::stdout().is_terminal()
            && width >= 60
            && (height as usize) > final_dashboard.len();

        if !animate {
            for line in final_dashboard {
                println!("{line}");
            }
            return;
        }

        let mut out = std::io::stdout();
        // RAII: the cursor is restored on every exit path (normal end, an early
        // return, or a panic-unwind), not just at the end of the loop.
        let _cursor = CursorGuard::hide();
        const FRAMES: u16 = 16;
        let cols = width as usize;
        for frame in 0..=FRAMES {
            let reveal = f64::from(frame) / f64::from(FRAMES);
            // Clip every line to the terminal width: a wrapped line would print as
            // two physical rows and desync the MoveToPreviousLine repaint.
            let lines: Vec<String> = self
                .dashboard_lines(theme, i18n, goal, today_date, reveal)
                .into_iter()
                .map(|l| crate::render::clip_to_width(&l, cols))
                .collect();
            if frame > 0 {
                let _ = execute!(out, cursor::MoveToPreviousLine(lines.len() as u16));
            }
            for line in &lines {
                let _ = execute!(out, Clear(ClearType::CurrentLine));
                let _ = writeln!(out, "{line}");
            }
            let _ = out.flush();
            std::thread::sleep(Duration::from_millis(28));
        }
    }

    /// Build the dashboard as styled lines. `reveal` in `0.0..=1.0` scales the
    /// chart data so the charts can animate from empty to full; the textual
    /// summary is always shown in full. The line count is constant across
    /// `reveal` values so the animation can repaint in place.
    fn dashboard_lines(
        &self,
        theme: &Theme,
        i18n: &I18n,
        goal: u64,
        today_date: NaiveDate,
        reveal: f64,
    ) -> Vec<String> {
        let p = &theme.palette;
        let accent = p.accent;
        let scale = |v: u64| (v as f64 * reveal.clamp(0.0, 1.0)).round() as u64;
        let field = |label: &str, value: String| {
            format!("  {} {}", theme.bold(format!("{label:<16}"), accent), value)
        };

        let mut lines = vec![
            String::new(),
            theme.bold(i18n.t(Msg::StatsTitle), accent),
            String::new(),
        ];

        // --- textual summary (shown in full from the first frame) ---
        let (pomos, minutes, days) = self.totals();
        let min_focus = i18n.t(Msg::MinFocus);
        let today_stat = self.day(today_date);
        lines.push(field(
            i18n.t(Msg::StatsToday),
            format!(
                "{} · {} {min_focus}",
                i18n.count(today_stat.completed_pomodoros, Noun::Pomodoro),
                today_stat.focus_minutes
            ),
        ));
        lines.push(field(
            i18n.t(Msg::StatsAllTime),
            format!(
                "{} · {} {min_focus} {} {}",
                i18n.count(pomos, Noun::Pomodoro),
                minutes,
                i18n.t(Msg::Over),
                i18n.count(days as u64, Noun::Day),
            ),
        ));
        lines.push(field(
            i18n.t(Msg::StatsStreak),
            i18n.count(self.streak(today_date), Noun::Day),
        ));
        lines.push(field(
            i18n.t(Msg::StatsLongestStreak),
            i18n.count(self.longest_streak(), Noun::Day),
        ));
        if let Some((date, stat)) = self.best_day() {
            lines.push(field(
                i18n.t(Msg::StatsBestDay),
                format!(
                    "{date} ({})",
                    i18n.count(stat.completed_pomodoros, Noun::Pomodoro)
                ),
            ));
        }

        // --- daily goal (optional) ---
        if goal > 0 {
            let done = scale(today_stat.completed_pomodoros);
            let bar = charts::goal_bar(theme, done, goal, 20, accent);
            let mut value = format!("{} {}/{}", bar.as_str(), done, goal);
            if reveal >= 1.0 && today_stat.completed_pomodoros >= goal {
                value.push_str(&format!(
                    "  {}",
                    theme.bold(i18n.t(Msg::GoalReached), p.success)
                ));
            }
            lines.push(String::new());
            lines.push(field(i18n.t(Msg::StatsGoal), value));
        }

        // --- last 14 days bar chart ---
        let last14: Vec<u64> = self
            .last_n_days(14, today_date)
            .iter()
            .map(|(_, d)| scale(d.completed_pomodoros))
            .collect();
        lines.push(String::new());
        lines.push(format!("  {}", theme.dim(i18n.t(Msg::StatsLast14))));
        for bar in charts::bar_chart(theme, &last14, 5, p.focus, accent) {
            lines.push(format!("  {}", bar.as_str()));
        }

        // --- last 12 weeks contribution heatmap ---
        let series = self.last_n_days(84, today_date);
        let counts: Vec<u64> = series
            .iter()
            .map(|(_, d)| scale(d.completed_pomodoros))
            .collect();
        let first_wd = series
            .first()
            .map(|(d, _)| d.weekday().num_days_from_monday() as usize)
            .unwrap_or(0);
        let empty_cell = p.muted.shade(0.3);
        lines.push(String::new());
        lines.push(format!("  {}", theme.dim(i18n.t(Msg::StatsHeatmap))));
        for row in charts::heatmap(theme, &counts, first_wd, accent, empty_cell) {
            lines.push(format!("  {}", row.as_str()));
        }

        // legend: less ▢▢▢▢ more
        let mut legend = format!("  {} ", theme.dim(i18n.t(Msg::HeatLess)));
        for i in 0..5 {
            let c = empty_cell.lerp(accent, f64::from(i) / 4.0);
            legend.push_str(&theme.paint("■", c));
        }
        legend.push_str(&format!(" {}", theme.dim(i18n.t(Msg::HeatMore))));
        lines.push(legend);
        lines.push(String::new());

        lines
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

    #[test]
    fn last_n_days_fills_gaps_and_is_chronological() {
        let mut s = Stats::default();
        s.record_pomodoro(25, "2026-06-05"); // today
        s.record_pomodoro(25, "2026-06-03"); // two days ago
        let today = NaiveDate::from_ymd_opt(2026, 6, 5).unwrap();
        let series = s.last_n_days(3, today);
        let counts: Vec<u64> = series.iter().map(|(_, d)| d.completed_pomodoros).collect();
        // oldest first: 06-03=1, 06-04=0 (gap filled), 06-05=1
        assert_eq!(counts, vec![1, 0, 1]);
        assert_eq!(
            series.first().unwrap().0,
            NaiveDate::from_ymd_opt(2026, 6, 3).unwrap()
        );
        assert_eq!(series.last().unwrap().0, today);
    }

    #[test]
    fn longest_streak_finds_the_longest_run() {
        let mut s = Stats::default();
        // A run of 3, a gap, then a run of 2.
        for d in [
            "2026-05-01",
            "2026-05-02",
            "2026-05-03",
            "2026-05-10",
            "2026-05-11",
        ] {
            s.record_pomodoro(25, d);
        }
        assert_eq!(s.longest_streak(), 3);
        assert_eq!(Stats::default().longest_streak(), 0);
    }
}
