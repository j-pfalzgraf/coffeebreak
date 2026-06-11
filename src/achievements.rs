//! Achievements — motivational badges derived purely from the statistics.
//!
//! This module adds a light, opt-in layer of gamification on top of
//! [`crate::stats`]. It introduces **no new persisted state**: every badge is a
//! pure predicate over a [`Snapshot`] of metrics the [`Stats`] API already
//! exposes (lifetime totals, streaks, the best day, recent activity, the daily
//! goal). The catalogue is a single `static` table — adding a badge is one row.
//!
//! Rendering mirrors the statistics dashboard ([`Stats::print_summary`]): a
//! localised board that, on a colour terminal, *reveals* with a short animation
//! (badges light up and a mastery bar fills) and otherwise prints once. All text
//! is localised via [`Msg`]; emoji glyphs appear only in free-form strings, never
//! in width-counted cells, so column alignment never depends on emoji width.

use std::io::{IsTerminal, Write};
use std::time::Duration;

use chrono::{Datelike, Local, NaiveDate, Weekday};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{cursor, execute};
use unicode_width::UnicodeWidthStr;

use crate::charts;
use crate::i18n::{I18n, Msg, Noun};
use crate::stats::Stats;
use crate::theme::{Palette, Rgb, Theme};
use crate::ui::CursorGuard;

/// A snapshot of every metric the badge predicates read, computed once so each
/// predicate is a cheap field access rather than a re-scan of the history.
#[derive(Debug, Clone, Copy)]
pub struct Snapshot {
    total_pomos: u64,
    total_minutes: u64,
    today_pomos: u64,
    streak: u64,
    longest_streak: u64,
    best_day_pomos: u64,
    /// Active days within the last 7 (inclusive of today).
    last7_active: u64,
    /// Whether any Saturday/Sunday in the last 12 weeks had a pomodoro.
    weekend_active: bool,
    goal: u64,
}

impl Snapshot {
    /// Derive a snapshot from the stats for `today` with the active `goal`.
    pub fn new(stats: &Stats, goal: u64, today: NaiveDate) -> Snapshot {
        let (total_pomos, total_minutes, _) = stats.totals();
        let best_day_pomos = stats
            .best_day()
            .map(|(_, d)| d.completed_pomodoros)
            .unwrap_or(0);
        let last7_active = stats
            .last_n_days(7, today)
            .iter()
            .filter(|(_, d)| d.completed_pomodoros > 0)
            .count() as u64;
        let weekend_active = stats.last_n_days(84, today).iter().any(|(date, d)| {
            d.completed_pomodoros > 0 && matches!(date.weekday(), Weekday::Sat | Weekday::Sun)
        });
        Snapshot {
            total_pomos,
            total_minutes,
            today_pomos: stats.day(today).completed_pomodoros,
            streak: stats.streak(today),
            longest_streak: stats.longest_streak(),
            best_day_pomos,
            last7_active,
            weekend_active,
            goal,
        }
    }
}

/// The tiers badges are grouped under, in display order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tier {
    First,
    Volume,
    Streak,
    SingleDay,
    Consistency,
}

/// Tiers in display order.
const TIERS: [Tier; 5] = [
    Tier::First,
    Tier::Volume,
    Tier::Streak,
    Tier::SingleDay,
    Tier::Consistency,
];

impl Tier {
    /// The localised heading for this tier.
    fn heading(self) -> Msg {
        match self {
            Tier::First => Msg::AchTierFirst,
            Tier::Volume => Msg::AchTierVolume,
            Tier::Streak => Msg::AchTierStreak,
            Tier::SingleDay => Msg::AchTierSingleDay,
            Tier::Consistency => Msg::AchTierConsistency,
        }
    }

    /// The accent colour unlocked badges in this tier are painted.
    fn color(self, p: &Palette) -> Rgb {
        match self {
            Tier::First => p.short_break,
            Tier::Volume => p.accent,
            Tier::Streak => p.focus,
            Tier::SingleDay => p.long_break,
            Tier::Consistency => p.success,
        }
    }
}

/// One badge: identity, tier, localised text, and pure predicates over a
/// [`Snapshot`].
struct Achievement {
    /// Stable, kebab-case id — the badge's identity for tests and a future
    /// machine-readable export. Not read by the renderer (which uses the
    /// localised title), hence the explicit allow.
    #[allow(dead_code)]
    id: &'static str,
    glyph: &'static str,
    tier: Tier,
    title: Msg,
    desc: Msg,
    /// Whether the badge is earned.
    unlocked: fn(&Snapshot) -> bool,
    /// Progress toward the badge as `(current, target)` for the "next" hint bar.
    progress: fn(&Snapshot) -> (u64, u64),
    /// Whether the badge is shown at all (e.g. the goal badge is hidden when no
    /// daily goal is set, so it never lingers as permanently locked).
    visible: fn(&Snapshot) -> bool,
}

/// Always-visible badge.
const SHOWN: fn(&Snapshot) -> bool = |_| true;

/// The full badge catalogue, in display order within each tier.
static CATALOGUE: &[Achievement] = &[
    // --- First steps ---
    Achievement {
        id: "first-sip",
        glyph: "☕",
        tier: Tier::First,
        title: Msg::AchFirstSipT,
        desc: Msg::AchFirstSipD,
        unlocked: |s| s.total_pomos >= 1,
        progress: |s| (s.total_pomos, 1),
        visible: SHOWN,
    },
    Achievement {
        id: "getting-started",
        glyph: "🌱",
        tier: Tier::First,
        title: Msg::AchGettingStartedT,
        desc: Msg::AchGettingStartedD,
        unlocked: |s| s.total_pomos >= 10,
        progress: |s| (s.total_pomos, 10),
        visible: SHOWN,
    },
    // --- Volume milestones ---
    Achievement {
        id: "half-century",
        glyph: "✨",
        tier: Tier::Volume,
        title: Msg::AchHalfCenturyT,
        desc: Msg::AchHalfCenturyD,
        unlocked: |s| s.total_pomos >= 50,
        progress: |s| (s.total_pomos, 50),
        visible: SHOWN,
    },
    Achievement {
        id: "centurion",
        glyph: "💯",
        tier: Tier::Volume,
        title: Msg::AchCenturionT,
        desc: Msg::AchCenturionD,
        unlocked: |s| s.total_pomos >= 100,
        progress: |s| (s.total_pomos, 100),
        visible: SHOWN,
    },
    Achievement {
        id: "deep-diver",
        glyph: "🌊",
        tier: Tier::Volume,
        title: Msg::AchDeepDiverT,
        desc: Msg::AchDeepDiverD,
        unlocked: |s| s.total_pomos >= 250,
        progress: |s| (s.total_pomos, 250),
        visible: SHOWN,
    },
    Achievement {
        id: "mountaineer",
        // Append U+FE0F so unicode-width counts this emoji as 2 columns, matching
        // how terminals render it (else a clipped board line would wrap and
        // desync the animated reveal's in-place repaint).
        glyph: "🏔\u{fe0f}",
        tier: Tier::Volume,
        title: Msg::AchMountaineerT,
        desc: Msg::AchMountaineerD,
        unlocked: |s| s.total_pomos >= 500,
        progress: |s| (s.total_pomos, 500),
        visible: SHOWN,
    },
    Achievement {
        id: "millennium",
        glyph: "👑",
        tier: Tier::Volume,
        title: Msg::AchMillenniumT,
        desc: Msg::AchMillenniumD,
        unlocked: |s| s.total_pomos >= 1000,
        progress: |s| (s.total_pomos, 1000),
        visible: SHOWN,
    },
    Achievement {
        id: "hour-master",
        glyph: "⏳",
        tier: Tier::Volume,
        title: Msg::AchHourMasterT,
        desc: Msg::AchHourMasterD,
        unlocked: |s| s.total_minutes >= 600,
        progress: |s| (s.total_minutes, 600),
        visible: SHOWN,
    },
    // --- Streak milestones ---
    Achievement {
        id: "on-a-roll",
        glyph: "🔥",
        tier: Tier::Streak,
        title: Msg::AchOnARollT,
        desc: Msg::AchOnARollD,
        unlocked: |s| s.longest_streak >= 3,
        progress: |s| (s.longest_streak, 3),
        visible: SHOWN,
    },
    Achievement {
        id: "week-warrior",
        glyph: "📅",
        tier: Tier::Streak,
        title: Msg::AchWeekWarriorT,
        desc: Msg::AchWeekWarriorD,
        unlocked: |s| s.longest_streak >= 7,
        progress: |s| (s.longest_streak, 7),
        visible: SHOWN,
    },
    Achievement {
        id: "fortnight-focus",
        glyph: "🗓\u{fe0f}",
        tier: Tier::Streak,
        title: Msg::AchFortnightT,
        desc: Msg::AchFortnightD,
        unlocked: |s| s.longest_streak >= 14,
        progress: |s| (s.longest_streak, 14),
        visible: SHOWN,
    },
    Achievement {
        id: "unbroken",
        glyph: "💎",
        tier: Tier::Streak,
        title: Msg::AchUnbrokenT,
        desc: Msg::AchUnbrokenD,
        unlocked: |s| s.longest_streak >= 30,
        progress: |s| (s.longest_streak, 30),
        visible: SHOWN,
    },
    // --- Single-day feats ---
    Achievement {
        id: "productive-day",
        glyph: "⭐",
        tier: Tier::SingleDay,
        title: Msg::AchProductiveDayT,
        desc: Msg::AchProductiveDayD,
        unlocked: |s| s.best_day_pomos >= 4,
        progress: |s| (s.best_day_pomos, 4),
        visible: SHOWN,
    },
    Achievement {
        id: "in-the-zone",
        glyph: "🚀",
        tier: Tier::SingleDay,
        title: Msg::AchInTheZoneT,
        desc: Msg::AchInTheZoneD,
        unlocked: |s| s.best_day_pomos >= 8,
        progress: |s| (s.best_day_pomos, 8),
        visible: SHOWN,
    },
    Achievement {
        id: "marathoner",
        glyph: "🏃",
        tier: Tier::SingleDay,
        title: Msg::AchMarathonT,
        desc: Msg::AchMarathonD,
        unlocked: |s| s.best_day_pomos >= 12,
        progress: |s| (s.best_day_pomos, 12),
        visible: SHOWN,
    },
    // --- Consistency ---
    Achievement {
        id: "weekend-focus",
        glyph: "🌤\u{fe0f}",
        tier: Tier::Consistency,
        title: Msg::AchWeekendFocusT,
        desc: Msg::AchWeekendFocusD,
        unlocked: |s| s.weekend_active,
        progress: |s| (u64::from(s.weekend_active), 1),
        visible: SHOWN,
    },
    Achievement {
        id: "regular",
        glyph: "🧭",
        tier: Tier::Consistency,
        title: Msg::AchRegularT,
        desc: Msg::AchRegularD,
        unlocked: |s| s.last7_active >= 5,
        progress: |s| (s.last7_active, 5),
        visible: SHOWN,
    },
    Achievement {
        id: "goal-getter",
        glyph: "🎯",
        tier: Tier::Consistency,
        title: Msg::AchGoalGetterT,
        desc: Msg::AchGoalGetterD,
        unlocked: |s| s.goal > 0 && s.today_pomos >= s.goal,
        progress: |s| (s.today_pomos, s.goal.max(1)),
        // Only meaningful with a daily goal set.
        visible: |s| s.goal > 0,
    },
];

/// `(unlocked, visible_total)` for the current snapshot.
pub fn tally(stats: &Stats, goal: u64, today: NaiveDate) -> (u64, u64) {
    let snap = Snapshot::new(stats, goal, today);
    let visible: Vec<&Achievement> = CATALOGUE.iter().filter(|a| (a.visible)(&snap)).collect();
    let unlocked = visible.iter().filter(|a| (a.unlocked)(&snap)).count() as u64;
    (unlocked, visible.len() as u64)
}

/// Render the achievements board to stdout, styled via `theme`, localised via
/// `i18n`, for the active daily `goal`.
pub fn print(stats: &Stats, theme: &Theme, i18n: &I18n, goal: u64) {
    let p = &theme.palette;
    let today = Local::now().date_naive();
    let snap = Snapshot::new(stats, goal, today);

    if snap.total_pomos == 0 {
        println!("\n{}\n", theme.bold(i18n.t(Msg::AchTitle), p.accent));
        println!("  {}\n", theme.dim(i18n.t(Msg::AchEmpty)));
        return;
    }

    let width = crossterm::terminal::size().map(|(w, _)| w).unwrap_or(80);
    let animate = theme.color() && std::io::stdout().is_terminal() && width >= 60;

    if !animate {
        for line in board_lines(&snap, theme, i18n, 1.0) {
            println!("{line}");
        }
        return;
    }

    let mut out = std::io::stdout();
    // RAII: restore the cursor on every exit path (normal, early, or panic).
    let _cursor = CursorGuard::hide();
    const FRAMES: u16 = 16;
    let cols = width as usize;
    for frame in 0..=FRAMES {
        let reveal = f64::from(frame) / f64::from(FRAMES);
        let rendered: Vec<String> = board_lines(&snap, theme, i18n, reveal)
            .into_iter()
            .map(|l| crate::render::clip_to_width(&l, cols))
            .collect();
        if frame > 0 {
            let _ = execute!(out, cursor::MoveToPreviousLine(rendered.len() as u16));
        }
        for line in &rendered {
            let _ = execute!(out, Clear(ClearType::CurrentLine));
            let _ = writeln!(out, "{line}");
        }
        let _ = out.flush();
        std::thread::sleep(Duration::from_millis(26));
    }
}

/// Build the board as styled lines. `reveal` (`0.0..=1.0`) animates the mastery
/// bar and lights unlocked badges in sequence; the line count is constant across
/// `reveal` so the animation can repaint in place.
fn board_lines(snap: &Snapshot, theme: &Theme, i18n: &I18n, reveal: f64) -> Vec<String> {
    let p = &theme.palette;
    let accent = p.accent;
    let reveal = reveal.clamp(0.0, 1.0);

    let visible: Vec<&Achievement> = CATALOGUE.iter().filter(|a| (a.visible)(snap)).collect();
    let unlocked_flags: Vec<bool> = visible.iter().map(|a| (a.unlocked)(snap)).collect();
    let total = visible.len() as u64;
    let unlocked_count = unlocked_flags.iter().filter(|b| **b).count() as u64;
    let shown_unlocked = (unlocked_count as f64 * reveal).round() as u64;

    let field = |label: &str, value: String| {
        format!("  {} {}", theme.bold(format!("{label:<16}"), accent), value)
    };

    let mut out = vec![
        String::new(),
        theme.bold(i18n.t(Msg::AchTitle), accent),
        String::new(),
        field(
            i18n.t(Msg::AchUnlocked),
            format!("{shown_unlocked} / {total}"),
        ),
        field(i18n.t(Msg::StatsStreak), i18n.count(snap.streak, Noun::Day)),
        format!(
            "  {}",
            charts::goal_bar(theme, shown_unlocked, total.max(1), 24, accent).as_str()
        ),
    ];

    // Align the description column to the widest (localised) title.
    let title_w = visible
        .iter()
        .map(|a| i18n.t(a.title).width())
        .max()
        .unwrap_or(12);

    // Unlocked badges light up one after another as `reveal` grows.
    let mut unlocked_seen = 0u64;
    for tier in TIERS {
        let in_tier: Vec<usize> = visible
            .iter()
            .enumerate()
            .filter(|(_, a)| a.tier == tier)
            .map(|(i, _)| i)
            .collect();
        if in_tier.is_empty() {
            continue;
        }
        out.push(String::new());
        out.push(format!("  {}", theme.dim(i18n.t(tier.heading()))));
        for i in in_tier {
            let a = visible[i];
            let title = i18n.t(a.title);
            let desc = i18n.t(a.desc);
            let pad = " ".repeat(title_w.saturating_sub(title.width()));
            let lit = if unlocked_flags[i] {
                unlocked_seen += 1;
                reveal + 1e-9 >= unlocked_seen as f64 / (unlocked_count as f64 + 1.0)
            } else {
                false
            };
            let line = if lit {
                format!(
                    "  {} {}{}  {}",
                    a.glyph,
                    theme.bold(title, tier.color(p)),
                    pad,
                    theme.paint(desc, p.text),
                )
            } else {
                format!(
                    "  {} {}{}  {}",
                    theme.dim(a.glyph),
                    theme.dim(title),
                    pad,
                    theme.dim(desc),
                )
            };
            out.push(line);
        }
    }

    // A hint toward the next badge (the first visible, still-locked one).
    out.push(String::new());
    match unlocked_flags.iter().position(|u| !u) {
        Some(idx) => {
            let a = visible[idx];
            let (cur, target) = (a.progress)(snap);
            let target = target.max(1);
            let bar = charts::goal_bar(theme, cur.min(target), target, 12, accent);
            out.push(format!(
                "  {} {} {}  {} {}/{}",
                theme.bold(i18n.t(Msg::AchNext), accent),
                a.glyph,
                i18n.t(a.title),
                bar.as_str(),
                cur.min(target),
                target,
            ));
        }
        None => out.push(format!(
            "  {}",
            theme.bold(i18n.t(Msg::AchAllUnlocked), p.success)
        )),
    }
    out.push(String::new());
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::LANGUAGES;

    fn date() -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 6, 11).unwrap()
    }

    fn stats_with(pomos_per_day: &[(&str, u64, u64)]) -> Stats {
        let mut s = Stats::default();
        for (day, pomos, minutes) in pomos_per_day {
            for _ in 0..*pomos {
                s.record_pomodoro(minutes / pomos.max(&1), day);
            }
        }
        s
    }

    #[test]
    fn every_badge_locked_on_empty_stats() {
        let snap = Snapshot::new(&Stats::default(), 0, date());
        assert!(
            CATALOGUE.iter().all(|a| !(a.unlocked)(&snap)),
            "no badge should unlock with no activity"
        );
    }

    #[test]
    fn first_sip_unlocks_at_one_pomodoro() {
        let s = stats_with(&[("2026-06-11", 1, 25)]);
        let (unlocked, total) = tally(&s, 0, date());
        assert!(unlocked >= 1);
        // goal badge hidden without a goal → 17 visible.
        assert_eq!(total, 17);
    }

    #[test]
    fn goal_badge_visibility_tracks_goal() {
        let s = stats_with(&[("2026-06-11", 1, 25)]);
        assert_eq!(tally(&s, 0, date()).1, 17, "goal badge hidden when goal=0");
        assert_eq!(tally(&s, 5, date()).1, 18, "goal badge visible when goal>0");
    }

    #[test]
    fn catalogue_ids_unique_and_kebab() {
        let mut seen = std::collections::HashSet::new();
        for a in CATALOGUE {
            assert!(seen.insert(a.id), "duplicate id {}", a.id);
            assert!(
                a.id.chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'),
                "id `{}` is not kebab-case",
                a.id
            );
        }
    }

    #[test]
    fn every_badge_glyph_is_two_columns_wide() {
        // A width-1 emoji in a board line would render wider than `unicode-width`
        // counts, wrap, and desync the animated repaint. Every glyph must measure
        // a consistent 2 columns (some emoji need an explicit U+FE0F selector).
        for a in CATALOGUE {
            assert_eq!(
                a.glyph.width(),
                2,
                "badge `{}` glyph {:?} is not 2 columns wide",
                a.id,
                a.glyph
            );
        }
    }

    #[test]
    fn titles_and_descriptions_resolve_in_every_locale() {
        for (code, _) in LANGUAGES {
            let i18n = I18n::new(code);
            for a in CATALOGUE {
                assert!(!i18n.t(a.title).trim().is_empty(), "{code}: {}", a.id);
                assert!(!i18n.t(a.desc).trim().is_empty(), "{code}: {}", a.id);
            }
        }
    }

    #[test]
    fn board_line_count_is_constant_across_reveal() {
        // A populated snapshot so several tiers render; the animated repaint
        // relies on a stable line count.
        let s = stats_with(&[("2026-06-09", 4, 100), ("2026-06-10", 6, 150)]);
        let snap = Snapshot::new(&s, 4, date());
        let theme = Theme::resolve("coffee", true);
        let i18n = I18n::new("en");
        let n0 = board_lines(&snap, &theme, &i18n, 0.0).len();
        let n1 = board_lines(&snap, &theme, &i18n, 0.5).len();
        let n2 = board_lines(&snap, &theme, &i18n, 1.0).len();
        assert_eq!(n0, n1);
        assert_eq!(n1, n2);
    }
}
