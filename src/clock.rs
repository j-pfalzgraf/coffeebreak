//! Time abstraction and a pause-aware phase timer.
//!
//! [`PhaseTimer`] owns all the countdown arithmetic — elapsed, remaining,
//! fraction, pause/resume, extend/shrink — and takes the current [`Instant`]
//! explicitly on every query. That keeps it pure and unit-testable (no sleeping,
//! no wall clock) while the real app feeds it `Instant::now()`.

use std::time::{Duration, Instant};

/// Source of "now" and of sleeping. Real code uses [`SystemClock`]; the
/// indirection exists so timing-dependent code can be exercised deterministically.
pub trait Clock {
    fn now(&self) -> Instant;
    fn sleep(&self, dur: Duration);
}

/// The production clock backed by the OS.
#[derive(Debug, Clone, Copy, Default)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> Instant {
        Instant::now()
    }
    fn sleep(&self, dur: Duration) {
        std::thread::sleep(dur);
    }
}

/// A single phase's countdown, aware of pausing.
///
/// Internally it tracks time already accumulated while running plus, when not
/// paused, the [`Instant`] the current running stretch began. All public queries
/// are computed against a caller-supplied `now`.
#[derive(Debug, Clone)]
pub struct PhaseTimer {
    total: Duration,
    accumulated: Duration,
    running_since: Option<Instant>,
}

impl PhaseTimer {
    /// Start a timer of length `total`, running, as of `now`.
    pub fn start(total: Duration, now: Instant) -> PhaseTimer {
        PhaseTimer {
            total,
            accumulated: Duration::ZERO,
            running_since: Some(now),
        }
    }

    /// Whether the timer is currently paused.
    pub fn is_paused(&self) -> bool {
        self.running_since.is_none()
    }

    /// Total configured length (after any extend/shrink).
    pub fn total(&self) -> Duration {
        self.total
    }

    /// Elapsed time so far.
    pub fn elapsed(&self, now: Instant) -> Duration {
        let live = match self.running_since {
            Some(since) => now.saturating_duration_since(since),
            None => Duration::ZERO,
        };
        (self.accumulated + live).min(self.total)
    }

    /// Time left.
    pub fn remaining(&self, now: Instant) -> Duration {
        self.total.saturating_sub(self.elapsed(now))
    }

    /// Fraction of time remaining in `0.0..=1.0` (1.0 = just started).
    pub fn fraction_remaining(&self, now: Instant) -> f64 {
        if self.total.is_zero() {
            return 0.0;
        }
        self.remaining(now).as_secs_f64() / self.total.as_secs_f64()
    }

    /// Whether the phase has run out.
    pub fn is_done(&self, now: Instant) -> bool {
        self.elapsed(now) >= self.total
    }

    /// Pause the countdown, banking elapsed time.
    pub fn pause(&mut self, now: Instant) {
        if let Some(since) = self.running_since.take() {
            self.accumulated += now.saturating_duration_since(since);
        }
    }

    /// Resume after a pause.
    pub fn resume(&mut self, now: Instant) {
        if self.running_since.is_none() {
            self.running_since = Some(now);
        }
    }

    /// Toggle pause/resume.
    pub fn toggle(&mut self, now: Instant) {
        if self.is_paused() {
            self.resume(now);
        } else {
            self.pause(now);
        }
    }

    /// Lengthen the phase by `delta`.
    pub fn extend(&mut self, delta: Duration) {
        self.total = self.total.saturating_add(delta);
    }

    /// Shorten the phase by `delta`, never below the time already elapsed (so it
    /// won't instantly complete unless it already had).
    pub fn shrink(&mut self, delta: Duration, now: Instant) {
        let floor = self.elapsed(now);
        self.total = self.total.saturating_sub(delta).max(floor);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(base: Instant, secs: u64) -> Instant {
        base + Duration::from_secs(secs)
    }

    #[test]
    fn elapsed_and_remaining_track_running_time() {
        let t0 = Instant::now();
        let timer = PhaseTimer::start(Duration::from_secs(100), t0);
        assert_eq!(timer.elapsed(at(t0, 30)), Duration::from_secs(30));
        assert_eq!(timer.remaining(at(t0, 30)), Duration::from_secs(70));
        assert!((timer.fraction_remaining(at(t0, 50)) - 0.5).abs() < 1e-9);
        assert!(!timer.is_done(at(t0, 99)));
        assert!(timer.is_done(at(t0, 100)));
        // never overshoots
        assert_eq!(timer.elapsed(at(t0, 250)), Duration::from_secs(100));
    }

    #[test]
    fn pause_freezes_elapsed() {
        let t0 = Instant::now();
        let mut timer = PhaseTimer::start(Duration::from_secs(100), t0);
        timer.pause(at(t0, 20));
        // time passes while paused; elapsed stays at 20
        assert_eq!(timer.elapsed(at(t0, 80)), Duration::from_secs(20));
        timer.resume(at(t0, 80));
        // running again: 20 banked + 10 live = 30 at t=90
        assert_eq!(timer.elapsed(at(t0, 90)), Duration::from_secs(30));
    }

    #[test]
    fn extend_and_shrink_respect_floor() {
        let t0 = Instant::now();
        let mut timer = PhaseTimer::start(Duration::from_secs(60), t0);
        timer.extend(Duration::from_secs(30));
        assert_eq!(timer.total(), Duration::from_secs(90));
        // at t=40, shrinking by 60 cannot drop total below elapsed (40)
        timer.shrink(Duration::from_secs(60), at(t0, 40));
        assert_eq!(timer.total(), Duration::from_secs(40));
    }
}
