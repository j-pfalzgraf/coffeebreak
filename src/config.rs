//! On-disk configuration (`config.toml`).
//!
//! Every field is optional in the file so a partial config still loads and
//! missing values fall back to [`Config::default`]. A first run with no config
//! file works immediately (one of the project's success criteria).

use std::collections::BTreeMap;
use std::fs;

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

use crate::fsutil::{self, FileKind};
use crate::paths;

/// Built-in defaults: the classic 25/5 Pomodoro, long break of 15 every 4
/// focus blocks, one cycle, with sound and notifications on.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    /// Focus block length in minutes.
    pub work_minutes: u64,
    /// Short break length in minutes.
    pub break_minutes: u64,
    /// Long break length in minutes.
    pub long_break_minutes: u64,
    /// Number of focus→break cycles to run.
    pub cycles: u64,
    /// Take a long break after this many focus blocks (when `--long`/`long`).
    pub long_break_every: u64,
    /// Whether the long break is enabled by default.
    pub long_break: bool,
    /// Audible cue on phase change (terminal bell, or chime with `sound`).
    pub sound: bool,
    /// Desktop notification on phase change.
    pub notifications: bool,
    /// Use the current git branch as the session label when none is given.
    pub git_label: bool,
    /// Colour theme name (see `coffeebreak themes`).
    pub theme: String,
    /// Animation frames per second for the live UI (2..=60).
    pub fps: u32,
    /// Interface language code (see `coffeebreak languages`); empty = auto-detect.
    pub language: String,
    /// Daily pomodoro goal shown in the stats dashboard (0 = disabled).
    pub daily_goal: u64,
    /// Log every completed focus block to `~/.coffeebreak/history.jsonl`
    /// (viewable with `coffeebreak history`). Off by default.
    pub history: bool,
    /// Automatically start the next phase. When false (or with `--wait`), the
    /// timer waits for a keypress between phases.
    pub auto_advance: bool,
    /// Big-countdown indicator style for the live timer: `digits` or `ring`.
    pub indicator: String,
    /// Play the brewing intro animation before the first focus block.
    pub brew: bool,
    /// Palette overrides for `--theme custom`: a map of palette-field name to a
    /// `#RRGGBB` hex colour (see `coffeebreak themes` / the docs for the keys).
    /// Unset fields fall back to the `coffee` palette.
    pub custom_theme: BTreeMap<String, String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            work_minutes: 25,
            break_minutes: 5,
            long_break_minutes: 15,
            cycles: 1,
            long_break_every: 4,
            long_break: false,
            sound: true,
            notifications: true,
            git_label: false,
            theme: crate::theme::DEFAULT_THEME.to_string(),
            fps: 15,
            // Empty means "auto-detect from the environment, default English".
            language: String::new(),
            daily_goal: 0,
            history: false,
            auto_advance: true,
            indicator: "digits".to_string(),
            brew: false,
            custom_theme: BTreeMap::new(),
        }
    }
}

impl Config {
    /// The path the config would be written to (for `config path`).
    pub fn path() -> anyhow::Result<std::path::PathBuf> {
        paths::config_file()
    }

    /// Write the built-in defaults to the config file if it does not already
    /// exist. Returns the path and whether a new file was created.
    pub fn init() -> anyhow::Result<(std::path::PathBuf, bool)> {
        let path = paths::config_file()?;
        if path.exists() {
            return Ok((path, false));
        }
        Config::default().save()?;
        Ok((path, true))
    }
}

/// All keys readable/writable via `config get`/`config set`, in file order.
/// (The `[custom_theme]` table is edited in the file directly.)
pub const KEYS: &[&str] = &[
    "work_minutes",
    "break_minutes",
    "long_break_minutes",
    "cycles",
    "long_break_every",
    "long_break",
    "sound",
    "notifications",
    "git_label",
    "theme",
    "fps",
    "language",
    "daily_goal",
    "history",
    "auto_advance",
    "indicator",
    "brew",
];

impl Config {
    /// The current value of a scalar config key, rendered as it would appear
    /// in the TOML file (bools as `true`/`false`, strings unquoted).
    pub fn get(&self, key: &str) -> Result<String> {
        Ok(match key {
            "work_minutes" => self.work_minutes.to_string(),
            "break_minutes" => self.break_minutes.to_string(),
            "long_break_minutes" => self.long_break_minutes.to_string(),
            "cycles" => self.cycles.to_string(),
            "long_break_every" => self.long_break_every.to_string(),
            "long_break" => self.long_break.to_string(),
            "sound" => self.sound.to_string(),
            "notifications" => self.notifications.to_string(),
            "git_label" => self.git_label.to_string(),
            "theme" => self.theme.clone(),
            "fps" => self.fps.to_string(),
            "language" => self.language.clone(),
            "daily_goal" => self.daily_goal.to_string(),
            "history" => self.history.to_string(),
            "auto_advance" => self.auto_advance.to_string(),
            "indicator" => self.indicator.clone(),
            "brew" => self.brew.to_string(),
            _ => bail!(
                "unknown config key `{key}` (valid keys: {})",
                KEYS.join(", ")
            ),
        })
    }

    /// Set a scalar config key from its string representation, validating both
    /// the type and the domain (so `config set` can never persist a value the
    /// loader or the UI would reject or silently clamp).
    pub fn set(&mut self, key: &str, value: &str) -> Result<()> {
        let value = value.trim();
        match key {
            "work_minutes" => self.work_minutes = parse_minutes(key, value)?,
            "break_minutes" => self.break_minutes = parse_minutes(key, value)?,
            "long_break_minutes" => self.long_break_minutes = parse_minutes(key, value)?,
            "cycles" => self.cycles = parse_int_in(key, value, 1, 10_000)?,
            "long_break_every" => self.long_break_every = parse_int_in(key, value, 1, 10_000)?,
            "long_break" => self.long_break = parse_bool(key, value)?,
            "sound" => self.sound = parse_bool(key, value)?,
            "notifications" => self.notifications = parse_bool(key, value)?,
            "git_label" => self.git_label = parse_bool(key, value)?,
            "theme" => {
                let normalized = value.to_ascii_lowercase();
                if !crate::theme::THEME_CHOICES.contains(&normalized.as_str()) {
                    bail!(
                        "invalid value `{value}` for `{key}` (valid themes: {})",
                        crate::theme::THEME_CHOICES.join(", ")
                    );
                }
                self.theme = normalized;
            }
            "fps" => self.fps = parse_int_in(key, value, 2, 60)? as u32,
            "language" => {
                let normalized = value.to_ascii_lowercase();
                if !normalized.is_empty() && !crate::i18n::LANG_CODES.contains(&normalized.as_str())
                {
                    bail!(
                        "invalid value `{value}` for `{key}` (valid languages: {})",
                        crate::i18n::LANG_CODES.join(", ")
                    );
                }
                self.language = normalized;
            }
            "daily_goal" => self.daily_goal = parse_int_in(key, value, 0, 1_000)?,
            "history" => self.history = parse_bool(key, value)?,
            "auto_advance" => self.auto_advance = parse_bool(key, value)?,
            "indicator" => {
                let normalized = value.to_ascii_lowercase();
                if !matches!(normalized.as_str(), "digits" | "ring") {
                    bail!("invalid value `{value}` for `{key}` (valid: digits, ring)");
                }
                self.indicator = normalized;
            }
            "brew" => self.brew = parse_bool(key, value)?,
            _ => bail!(
                "unknown config key `{key}` (valid keys: {})",
                KEYS.join(", ")
            ),
        }
        Ok(())
    }
}

/// Parse a boolean, accepting the common on/off spellings.
fn parse_bool(key: &str, value: &str) -> Result<bool> {
    match value.to_ascii_lowercase().as_str() {
        "true" | "yes" | "on" | "1" => Ok(true),
        "false" | "no" | "off" | "0" => Ok(false),
        _ => bail!("invalid value `{value}` for `{key}` (expected true or false)"),
    }
}

/// Parse an integer and require it to lie in `min..=max`.
fn parse_int_in(key: &str, value: &str, min: u64, max: u64) -> Result<u64> {
    let n: u64 = value
        .parse()
        .map_err(|_| anyhow::anyhow!("invalid value `{value}` for `{key}` (expected a number)"))?;
    if !(min..=max).contains(&n) {
        bail!("invalid value `{value}` for `{key}` (expected {min}..={max})");
    }
    Ok(n)
}

/// A duration key in minutes: at least 1, at most a full day.
fn parse_minutes(key: &str, value: &str) -> Result<u64> {
    parse_int_in(key, value, 1, 24 * 60)
}

impl Config {
    /// Load `config.toml`, falling back to defaults when it does not exist.
    ///
    /// A malformed config is a hard error (so typos are surfaced) rather than
    /// being silently ignored.
    pub fn load() -> Result<Config> {
        let path = paths::config_file()?;
        match fs::read_to_string(&path) {
            Ok(text) => toml::from_str(&text)
                .with_context(|| format!("failed to parse config at {}", path.display())),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Config::default()),
            Err(e) => {
                Err(e).with_context(|| format!("failed to read config at {}", path.display()))
            }
        }
    }

    /// Write the current config to disk (atomically), creating the directory if
    /// needed.
    pub fn save(&self) -> Result<()> {
        let dir = paths::config_dir()?;
        fs::create_dir_all(&dir)
            .with_context(|| format!("failed to create config dir {}", dir.display()))?;
        let path = paths::config_file()?;
        let text = toml::to_string_pretty(self).context("failed to serialize config")?;
        fsutil::write_atomic(&path, &text, FileKind::Shareable)
            .with_context(|| format!("failed to write config to {}", path.display()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `KEYS` and the struct can never drift apart: every scalar field in the
    /// serialized config must be gettable, settable (round-tripping its own
    /// current value), and listed — and vice versa.
    #[test]
    fn keys_cover_every_scalar_field_exactly() {
        let mut cfg = Config::default();
        let table: toml::Table = toml::from_str(&toml::to_string(&cfg).unwrap()).unwrap();

        for (field, value) in &table {
            if value.is_table() {
                continue; // [custom_theme] is deliberately not settable
            }
            assert!(
                KEYS.contains(&field.as_str()),
                "config field `{field}` is missing from KEYS"
            );
        }
        for key in KEYS {
            let current = cfg.get(key).unwrap_or_else(|e| panic!("get {key}: {e}"));
            cfg.set(key, &current)
                .unwrap_or_else(|e| panic!("set {key} = {current}: {e}"));
            assert_eq!(cfg.get(key).unwrap(), current, "round-trip of `{key}`");
        }
    }

    #[test]
    fn set_validates_types_and_domains() {
        let mut cfg = Config::default();

        cfg.set("work_minutes", "50").unwrap();
        assert_eq!(cfg.work_minutes, 50);
        cfg.set("history", "on").unwrap();
        assert!(cfg.history);
        cfg.set("theme", "NORD").unwrap(); // case-insensitive, normalised
        assert_eq!(cfg.theme, "nord");
        cfg.set("language", "").unwrap(); // "" = auto-detect stays allowed
        cfg.set("indicator", "ring").unwrap();

        for (key, bad) in [
            ("work_minutes", "0"),   // a zero-length focus block
            ("work_minutes", "abc"), // not a number
            ("fps", "1"),            // below the supported range
            ("fps", "61"),           // above the supported range
            ("theme", "sepia"),      // not a theme
            ("language", "xx"),      // not a shipped locale
            ("indicator", "spiral"), // not an indicator
            ("history", "maybe"),    // not a bool
            ("cycles", "0"),         // must run at least one
            ("no_such_key", "1"),    // unknown key
        ] {
            assert!(
                cfg.set(key, bad).is_err(),
                "set {key} = {bad} should be rejected"
            );
        }

        // Errors are actionable: the unknown-key message lists the valid keys.
        let err = cfg.set("no_such_key", "1").unwrap_err().to_string();
        assert!(err.contains("work_minutes"), "unhelpful error: {err}");
    }
}
