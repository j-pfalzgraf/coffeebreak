//! On-disk configuration (`config.toml`).
//!
//! Every field is optional in the file so a partial config still loads and
//! missing values fall back to [`Config::default`]. A first run with no config
//! file works immediately (one of the project's success criteria).

use std::fs;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

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
    /// Automatically start the next phase. When false (or with `--wait`), the
    /// timer waits for a keypress between phases.
    pub auto_advance: bool,
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
            auto_advance: true,
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

    /// Write the current config to disk, creating the directory if needed.
    pub fn save(&self) -> Result<()> {
        let dir = paths::config_dir()?;
        fs::create_dir_all(&dir)
            .with_context(|| format!("failed to create config dir {}", dir.display()))?;
        let path = paths::config_file()?;
        let text = toml::to_string_pretty(self).context("failed to serialize config")?;
        fs::write(&path, text)
            .with_context(|| format!("failed to write config to {}", path.display()))
    }
}
