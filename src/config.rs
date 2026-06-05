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
        }
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
