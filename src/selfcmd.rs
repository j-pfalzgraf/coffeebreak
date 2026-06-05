//! Lifecycle commands: `self update` and `self uninstall`.
//!
//! Updates pull signed GitHub releases over TLS via the `self_update` crate.
//! Per the project's security stance, updates only ever run on an explicit
//! command — never silently in the background.

use std::fs;
use std::io::{self, Write};

use anyhow::{Context, Result, bail};

use crate::{BIN_NAME, REPO_NAME, REPO_OWNER, paths};

const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// `coffeebreak self update [--check]`.
pub fn update(check_only: bool) -> Result<()> {
    let updater = self_update::backends::github::Update::configure()
        .repo_owner(REPO_OWNER)
        .repo_name(REPO_NAME)
        .bin_name(BIN_NAME)
        .current_version(CURRENT_VERSION)
        .show_download_progress(true)
        .build()
        .context("failed to configure the updater")?;

    if check_only {
        let latest = updater
            .get_latest_release()
            .context("failed to query the latest release")?;
        if self_update::version::bump_is_greater(CURRENT_VERSION, &latest.version)
            .unwrap_or(false)
        {
            println!(
                "A newer version is available: {CURRENT_VERSION} -> {}",
                latest.version
            );
            println!("Run `coffeebreak self update` to upgrade.");
        } else {
            println!("coffeebreak {CURRENT_VERSION} is up to date.");
        }
        return Ok(());
    }

    // Be transparent about what is about to happen (security stance).
    println!("Current version: {CURRENT_VERSION}");
    println!("Source: https://github.com/{REPO_OWNER}/{REPO_NAME}/releases");

    let status = updater.update().context("update failed")?;
    if status.updated() {
        println!("✓ Updated to {}.", status.version());
    } else {
        println!("Already up to date ({}).", status.version());
    }
    Ok(())
}

/// `coffeebreak self uninstall [--yes]`.
///
/// Removes the config directory, the data directory, and finally the binary
/// itself. Each removal is reported; a failure to remove the running binary
/// (common on Windows) is surfaced with manual instructions rather than
/// aborting.
pub fn uninstall(assume_yes: bool) -> Result<()> {
    let binary = std::env::current_exe().context("could not locate the running binary")?;
    let config_dir = paths::config_dir()?;
    let data_dir = paths::data_dir()?;

    println!("This will remove coffeebreak and its data:");
    println!("  • binary:  {}", binary.display());
    println!("  • config:  {}", config_dir.display());
    println!("  • data:    {}", data_dir.display());
    println!();

    if !assume_yes && !confirm("Remove all of the above?")? {
        println!("Aborted. Nothing was removed.");
        return Ok(());
    }

    remove_dir_if_present(&config_dir)?;
    remove_dir_if_present(&data_dir)?;

    match fs::remove_file(&binary) {
        Ok(()) => println!("✓ Removed binary {}", binary.display()),
        Err(e) => {
            // On Windows a running executable can't delete itself.
            eprintln!(
                "Could not remove the binary automatically ({e}).\n\
                 Please delete it manually:\n  {}",
                binary.display()
            );
        }
    }

    println!("coffeebreak uninstalled. ☕ Thanks for the focus sessions!");
    Ok(())
}

/// Remove a directory tree if it exists, treating "not found" as success.
fn remove_dir_if_present(dir: &std::path::Path) -> Result<()> {
    match fs::remove_dir_all(dir) {
        Ok(()) => {
            println!("✓ Removed {}", dir.display());
            Ok(())
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e).with_context(|| format!("failed to remove {}", dir.display())),
    }
}

/// Yes/no prompt on the controlling terminal. Refuses to assume "yes" when
/// stdin isn't a TTY (e.g. piped) — the caller must pass `--yes` for that.
fn confirm(prompt: &str) -> Result<bool> {
    use std::io::IsTerminal;
    if !io::stdin().is_terminal() {
        bail!("not a terminal; re-run with --yes to confirm non-interactively");
    }
    print!("{prompt} [y/N] ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let answer = input.trim().to_ascii_lowercase();
    Ok(answer == "y" || answer == "yes")
}
