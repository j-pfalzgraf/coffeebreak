//! Phase-change feedback backends: desktop notifications and sound.
//!
//! Both are modelled as small traits with swappable implementations, so the app
//! depends on the behaviour, not the concrete library, and silencing a channel
//! is just a different implementation (`NullNotifier` / `SilentPlayer`) rather
//! than scattered `if enabled` checks. Everything is best-effort: a missing
//! notification daemon or audio device must never crash the timer.

use crate::Phase;
use crate::i18n::I18n;

/// Sends a desktop notification with already-localised text.
pub trait Notifier: Send {
    fn notify(&self, summary: &str, body: &str);
}

/// Plays an audible cue on phase change.
pub trait SoundPlayer: Send {
    fn play(&self);
}

/// Bundles the two channels and fans a phase change out to both, localising the
/// notification text via the active [`I18n`].
pub struct Feedback {
    notifier: Box<dyn Notifier>,
    sound: Box<dyn SoundPlayer>,
    i18n: I18n,
}

impl Feedback {
    /// Build feedback honoring the session's `notifications`/`sound` switches.
    pub fn new(notifications: bool, sound: bool, i18n: I18n) -> Feedback {
        let notifier: Box<dyn Notifier> = if notifications {
            Box::new(DesktopNotifier)
        } else {
            Box::new(NullNotifier)
        };
        let sound: Box<dyn SoundPlayer> = if sound {
            Box::new(default_player())
        } else {
            Box::new(SilentPlayer)
        };
        Feedback { notifier, sound, i18n }
    }

    /// Announce entry into `phase` on every enabled channel.
    pub fn announce(&self, phase: Phase) {
        let summary = format!("coffeebreak — {}", self.i18n.phase_label(phase));
        self.notifier.notify(&summary, self.i18n.phase_announce(phase));
        self.sound.play();
    }
}

// --- Notifier implementations ----------------------------------------------

/// Real desktop notifications via `notify-rust`.
pub struct DesktopNotifier;

impl Notifier for DesktopNotifier {
    fn notify(&self, summary: &str, body: &str) {
        use notify_rust::{Notification, Timeout};
        let _ = Notification::new()
            .appname("coffeebreak")
            .summary(summary)
            .body(body)
            .timeout(Timeout::Milliseconds(6000))
            .show();
    }
}

/// Drops notifications on the floor.
pub struct NullNotifier;

impl Notifier for NullNotifier {
    fn notify(&self, _summary: &str, _body: &str) {}
}

// --- SoundPlayer implementations -------------------------------------------

/// The terminal bell (`\a`) — universal, dependency-free.
pub struct BellPlayer;

impl SoundPlayer for BellPlayer {
    fn play(&self) {
        use std::io::Write;
        let mut err = std::io::stderr();
        let _ = err.write_all(b"\x07");
        let _ = err.flush();
    }
}

/// Plays nothing.
pub struct SilentPlayer;

impl SoundPlayer for SilentPlayer {
    fn play(&self) {}
}

/// The default audible player: a rich chime when built with `--features sound`,
/// otherwise the terminal bell.
#[cfg(feature = "sound")]
fn default_player() -> ChimePlayer {
    ChimePlayer
}

#[cfg(not(feature = "sound"))]
fn default_player() -> BellPlayer {
    BellPlayer
}

/// A two-note chime via `rodio`. Falls back to the bell if audio is unavailable.
#[cfg(feature = "sound")]
pub struct ChimePlayer;

#[cfg(feature = "sound")]
impl SoundPlayer for ChimePlayer {
    fn play(&self) {
        if chime().is_err() {
            BellPlayer.play();
        }
    }
}

#[cfg(feature = "sound")]
fn chime() -> anyhow::Result<()> {
    use std::time::Duration;

    use rodio::source::{SineWave, Source};

    let handle = rodio::DeviceSinkBuilder::open_default_sink()?;
    let player = rodio::Player::connect_new(handle.mixer());
    for freq in [660.0_f32, 880.0_f32] {
        let note = SineWave::new(freq)
            .take_duration(Duration::from_millis(160))
            .amplify(0.20);
        player.append(note);
    }
    player.sleep_until_end();
    Ok(())
}
