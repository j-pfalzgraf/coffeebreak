//! Phase-change feedback: a desktop notification plus an audible cue.
//!
//! Everything here is best-effort — a headless box with no notification daemon
//! or no audio device must not crash the timer, so failures are swallowed.

use crate::Phase;

/// Fire the configured feedback for entering `next`.
pub fn announce_phase(next: Phase, sound: bool, notifications: bool) {
    if notifications {
        desktop_notify(next);
    }
    if sound {
        play_sound();
    }
}

/// Send a desktop notification (no-op on failure).
pub fn desktop_notify(next: Phase) {
    use notify_rust::{Notification, Timeout};
    let _ = Notification::new()
        .appname("coffeebreak")
        .summary(&format!("coffeebreak — {}", next.label()))
        .body(next.announce())
        .timeout(Timeout::Milliseconds(6000))
        .show();
}

/// Play the audible cue: a soft chime when built with the `sound` feature,
/// otherwise (and as a fallback) the terminal bell.
pub fn play_sound() {
    #[cfg(feature = "sound")]
    {
        if chime().is_err() {
            bell();
        }
    }
    #[cfg(not(feature = "sound"))]
    {
        bell();
    }
}

/// The terminal bell (`\a`) — always available, zero dependencies. Written to
/// stderr so it never pollutes piped stdout.
pub fn bell() {
    use std::io::Write;
    let mut err = std::io::stderr();
    let _ = err.write_all(b"\x07");
    let _ = err.flush();
}

/// A two-note chime via rodio. Compiled only with `--features sound`.
#[cfg(feature = "sound")]
fn chime() -> anyhow::Result<()> {
    use std::time::Duration;

    use rodio::source::{SineWave, Source};

    // rodio 0.22 API: open the default device sink, connect a player to its
    // mixer, queue the notes, then block until they finish.
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
