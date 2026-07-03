//! Filesystem locations for configuration and statistics.
//!
//! The project spec pins these:
//!   * config: `~/.config/coffeebreak/config.toml`
//!   * stats:  `~/.coffeebreak/stats.json`
//!
//! We honor those exact, predictable home-relative paths on **every** platform
//! (Linux/macOS/Windows), respecting `$XDG_CONFIG_HOME` for the config dir when
//! it is set to an absolute path. Keeping the layout identical across the Rust
//! app and the standalone `uninstall.sh` script avoids the two ever disagreeing
//! about what to clean up.

use std::path::PathBuf;

use anyhow::{Context, Result};
use directories::BaseDirs;

/// The user's home directory.
///
/// We honor the conventional per-platform home variable directly — `$HOME` on
/// Unix, `%USERPROFILE%` on Windows — and only fall back to the OS lookup when
/// it is unset (or not absolute). This keeps the home-relative config/data
/// paths predictable and redirectable on **every** platform: the `directories`
/// crate resolves the Windows home via `SHGetKnownFolderPath(FOLDERID_Profile)`,
/// which ignores `%USERPROFILE%`, so without this the location could not be
/// pinned — e.g. for the hermetic integration tests or a relocated profile.
pub fn home_dir() -> Result<PathBuf> {
    let var = if cfg!(windows) { "USERPROFILE" } else { "HOME" };
    if let Some(home) = std::env::var_os(var) {
        let p = PathBuf::from(&home);
        if p.is_absolute() {
            return Ok(p);
        }
    }
    let base = BaseDirs::new().context("could not determine the home directory")?;
    Ok(base.home_dir().to_path_buf())
}

/// Directory holding `config.toml`.
///
/// `$XDG_CONFIG_HOME/coffeebreak` when that variable is an absolute path,
/// otherwise `~/.config/coffeebreak` (per the spec).
pub fn config_dir() -> Result<PathBuf> {
    if let Some(xdg) = std::env::var_os("XDG_CONFIG_HOME") {
        let p = PathBuf::from(&xdg);
        if p.is_absolute() {
            return Ok(p.join("coffeebreak"));
        }
    }
    Ok(home_dir()?.join(".config").join("coffeebreak"))
}

/// Full path to the config file.
pub fn config_file() -> Result<PathBuf> {
    Ok(config_dir()?.join("config.toml"))
}

/// Directory holding `stats.json` (`~/.coffeebreak`), per the spec.
pub fn data_dir() -> Result<PathBuf> {
    Ok(home_dir()?.join(".coffeebreak"))
}

/// Full path to the statistics file.
pub fn stats_file() -> Result<PathBuf> {
    Ok(data_dir()?.join("stats.json"))
}

/// Full path to the per-session history log (`history.jsonl`).
pub fn history_file() -> Result<PathBuf> {
    Ok(data_dir()?.join("history.jsonl"))
}
