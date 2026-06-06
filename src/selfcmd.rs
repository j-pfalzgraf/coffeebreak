//! Lifecycle commands: `self update` and `self uninstall`.
//!
//! Updates pull signed GitHub releases over TLS via the `self_update` crate.
//! Per the project's security stance, updates only ever run on an explicit
//! command — never silently in the background.

use std::fs;
use std::io::{self, Write};

use anyhow::{Context, Result, bail};

use crate::i18n::{I18n, Msg};
use crate::{BIN_NAME, REPO_NAME, REPO_OWNER, paths};

const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// `coffeebreak self update [--check]`.
pub fn update(check_only: bool, i18n: &I18n) -> Result<()> {
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
        if self_update::version::bump_is_greater(CURRENT_VERSION, &latest.version).unwrap_or(false)
        {
            println!(
                "{}",
                i18n.tf(
                    Msg::UpdateNewer,
                    &[("current", CURRENT_VERSION), ("latest", &latest.version)],
                )
            );
            println!("{}", i18n.t(Msg::UpdateRunHint));
        } else {
            println!(
                "{}",
                i18n.tf(Msg::UpdateUpToDate, &[("version", CURRENT_VERSION)])
            );
        }
        return Ok(());
    }

    // Be transparent about what is about to happen (security stance).
    println!(
        "{}",
        i18n.tf(Msg::UpdateCurrent, &[("version", CURRENT_VERSION)])
    );
    let url = format!("https://github.com/{REPO_OWNER}/{REPO_NAME}/releases");
    println!("{}", i18n.tf(Msg::UpdateSource, &[("url", &url)]));

    let status = updater.update().context("update failed")?;
    if status.updated() {
        println!(
            "{}",
            i18n.tf(Msg::UpdateDone, &[("version", status.version())])
        );
    } else {
        println!(
            "{}",
            i18n.tf(Msg::UpdateAlready, &[("version", status.version())])
        );
    }
    Ok(())
}

/// `coffeebreak self uninstall [--yes]`.
///
/// Removes the config directory, the data directory, and finally the binary
/// itself. Each removal is reported; a failure to remove the running binary
/// (common on Windows) is surfaced with manual instructions rather than
/// aborting.
pub fn uninstall(assume_yes: bool, i18n: &I18n) -> Result<()> {
    let binary = std::env::current_exe().context("could not locate the running binary")?;
    let config_dir = paths::config_dir()?;
    let data_dir = paths::data_dir()?;

    println!("{}", i18n.t(Msg::UninstallIntro));
    println!(
        "  • {:<7} {}",
        i18n.t(Msg::UninstallItemBinary),
        binary.display()
    );
    println!(
        "  • {:<7} {}",
        i18n.t(Msg::UninstallItemConfig),
        config_dir.display()
    );
    println!(
        "  • {:<7} {}",
        i18n.t(Msg::UninstallItemData),
        data_dir.display()
    );
    println!();

    if !assume_yes && !confirm(i18n.t(Msg::UninstallConfirm), i18n)? {
        println!("{}", i18n.t(Msg::UninstallAborted));
        return Ok(());
    }

    remove_dir_if_present(&config_dir, i18n)?;
    remove_dir_if_present(&data_dir, i18n)?;

    match fs::remove_file(&binary) {
        Ok(()) => {
            println!(
                "{}",
                i18n.tf(
                    Msg::UninstallRemoved,
                    &[("path", &binary.display().to_string())]
                )
            )
        }
        Err(e) => {
            // On Windows a running executable can't delete itself.
            eprintln!(
                "{}\n  {}",
                i18n.tf(Msg::UninstallBinFail, &[("error", &e.to_string())]),
                binary.display()
            );
        }
    }

    println!("{}", i18n.t(Msg::UninstallDone));
    Ok(())
}

/// Remove a directory tree if it exists, treating "not found" as success.
fn remove_dir_if_present(dir: &std::path::Path, i18n: &I18n) -> Result<()> {
    match fs::remove_dir_all(dir) {
        Ok(()) => {
            println!(
                "{}",
                i18n.tf(
                    Msg::UninstallRemoved,
                    &[("path", &dir.display().to_string())]
                )
            );
            Ok(())
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e).with_context(|| format!("failed to remove {}", dir.display())),
    }
}

/// Yes/no prompt on the controlling terminal. Refuses to assume "yes" when
/// stdin isn't a TTY (e.g. piped) — the caller must pass `--yes` for that.
fn confirm(prompt: &str, i18n: &I18n) -> Result<bool> {
    use std::io::IsTerminal;
    if !io::stdin().is_terminal() {
        bail!("{}", i18n.t(Msg::NotATerminal));
    }
    print!("{prompt} {} ", i18n.t(Msg::ConfirmYesNo));
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let answer = input.trim().to_ascii_lowercase();
    // Accept the locale's affirmative key (matching the displayed prompt) plus a
    // universal English "y"/"yes" for muscle memory.
    let affirmative = i18n.t(Msg::ConfirmAffirmative);
    Ok(answer == affirmative || answer == "y" || answer == "yes")
}
