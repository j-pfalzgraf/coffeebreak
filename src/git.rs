//! Best-effort current git branch detection for session labels.
//!
//! This shells out to `git` and never fails the program: any error (not a repo,
//! git missing, detached HEAD) simply yields `None`.

use std::process::Command;

/// Return the current branch name, or `None` when it can't be determined.
pub fn current_branch() -> Option<String> {
    let output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let name = String::from_utf8(output.stdout).ok()?;
    let name = name.trim();

    // Detached HEAD reports "HEAD"; treat that as "no meaningful branch".
    if name.is_empty() || name == "HEAD" {
        None
    } else {
        Some(name.to_string())
    }
}
