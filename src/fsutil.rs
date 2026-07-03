//! Small filesystem helpers shared by every module that persists state.
//!
//! The one rule: **user data is never half-written**. All saves go through
//! [`write_atomic`], which writes to a temporary file in the target directory
//! and then renames it into place — so a crash, a full disk, or Ctrl+C at the
//! wrong moment leaves either the old file or the new one, never a torn mix.

use std::fs;
use std::io;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

/// How the written file should be protected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileKind {
    /// World-readable is fine (e.g. `config.toml` — knobs, not personal data).
    Shareable,
    /// Personal activity data (stats, session history): on Unix the file is
    /// created owner-only (`0600`), like a shell history file. On other
    /// platforms the home directory's own ACLs apply.
    Private,
}

/// Atomically replace `path` with `contents`.
///
/// The write lands in a uniquely named temporary file **in the same directory**
/// (renames are only atomic within one filesystem), is flushed to disk, and is
/// then renamed over `path`. `std::fs::rename` replaces an existing destination
/// on both Unix and Windows, so the swap is a single step on every platform.
pub fn write_atomic(path: &Path, contents: &str, kind: FileKind) -> io::Result<()> {
    let dir = path.parent().unwrap_or_else(|| Path::new("."));
    let tmp = dir.join(tmp_name(path));

    let result = (|| {
        {
            let mut file = fs::File::create(&tmp)?;
            set_permissions(&file, kind)?;
            io::Write::write_all(&mut file, contents.as_bytes())?;
            // Make sure the bytes hit the disk before the rename publishes them.
            file.sync_all()?;
        }
        fs::rename(&tmp, path)
    })();

    if result.is_err() {
        // Never leave a stray temp file behind on failure.
        let _ = fs::remove_file(&tmp);
    }
    result
}

/// A collision-free temporary sibling name for `path`.
///
/// Process id + a process-wide counter keep concurrent writers (multiple
/// coffeebreak instances, parallel tests) from clobbering each other's temp
/// files; the final rename still makes the last writer win cleanly.
fn tmp_name(path: &Path) -> String {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let base = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "file".to_string());
    format!(".{base}.tmp-{}-{n}", std::process::id())
}

#[cfg(unix)]
fn set_permissions(file: &fs::File, kind: FileKind) -> io::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    if kind == FileKind::Private {
        file.set_permissions(fs::Permissions::from_mode(0o600))?;
    }
    Ok(())
}

#[cfg(not(unix))]
fn set_permissions(_file: &fs::File, _kind: FileKind) -> io::Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp_dir(tag: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "coffeebreak-fsutil-{tag}-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn writes_and_replaces() {
        let dir = tmp_dir("replace");
        let path = dir.join("state.json");
        write_atomic(&path, "one", FileKind::Private).unwrap();
        assert_eq!(fs::read_to_string(&path).unwrap(), "one");
        write_atomic(&path, "two", FileKind::Private).unwrap();
        assert_eq!(fs::read_to_string(&path).unwrap(), "two");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn leaves_no_temp_files_behind() {
        let dir = tmp_dir("clean");
        let path = dir.join("state.json");
        write_atomic(&path, "data", FileKind::Shareable).unwrap();
        let entries: Vec<_> = fs::read_dir(&dir)
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
            .collect();
        assert_eq!(entries, vec!["state.json"], "stray files: {entries:?}");
        let _ = fs::remove_dir_all(&dir);
    }

    #[cfg(unix)]
    #[test]
    fn private_files_are_owner_only() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tmp_dir("perms");
        let path = dir.join("private.jsonl");
        write_atomic(&path, "secret", FileKind::Private).unwrap();
        let mode = fs::metadata(&path).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o600);
        let _ = fs::remove_dir_all(&dir);
    }
}
