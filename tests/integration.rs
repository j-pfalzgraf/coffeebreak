//! Black-box integration tests for the `coffeebreak` binary.
//!
//! These run the *built* binary (via `CARGO_BIN_EXE_coffeebreak`) as a child
//! process and assert on its stdout/stderr/exit status — no internals are
//! imported, so they exercise the program exactly as a user would.
//!
//! Hermeticity: every test gets its own throwaway home directory under
//! `std::env::temp_dir()`, wired up through both `HOME` and `XDG_CONFIG_HOME`.
//! Because `coffeebreak` resolves all of its files relative to those variables
//! (config at `$HOME/.config/coffeebreak` or `$XDG_CONFIG_HOME/coffeebreak`,
//! stats at `$HOME/.coffeebreak`), the tests never touch the real home and
//! never see each other's state. The directories are removed on success.
//!
//! Only the standard library is used (no extra dev-dependencies). The child's
//! stdout is a pipe rather than a TTY, so the binary automatically selects its
//! plain, non-animated path and the runs terminate promptly instead of hanging.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// Path to the binary under test, provided by Cargo at compile time.
const BIN: &str = env!("CARGO_BIN_EXE_coffeebreak");

/// A unique temporary home directory that cleans itself up when dropped.
///
/// Uniqueness is derived from the pid, a monotonically increasing counter, and
/// a timestamp so that parallel tests (Cargo runs tests on multiple threads)
/// never collide on a path.
struct TempHome {
    path: PathBuf,
}

impl TempHome {
    fn new(tag: &str) -> TempHome {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let name = format!("coffeebreak-it-{tag}-{}-{n}-{nanos}", std::process::id());
        let path = std::env::temp_dir().join(name);
        std::fs::create_dir_all(&path)
            .unwrap_or_else(|e| panic!("could not create temp home {}: {e}", path.display()));
        TempHome { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempHome {
    fn drop(&mut self) {
        // Best-effort cleanup; a leftover temp dir must never fail a test.
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

/// Build a `Command` for the binary with a hermetic, per-test environment.
///
/// We pin `HOME` and `XDG_CONFIG_HOME` to `home`, and clear `NO_COLOR` so the
/// process's own colour logic (TTY detection) is the only thing in play.
fn cmd(home: &TempHome, args: &[&str]) -> Command {
    let mut c = Command::new(BIN);
    c.args(args)
        .env("HOME", home.path())
        // On Windows the home directory is resolved from USERPROFILE, not HOME,
        // so pin it too to keep these tests hermetic there.
        .env("USERPROFILE", home.path())
        .env("XDG_CONFIG_HOME", home.path().join(".config"))
        .env_remove("NO_COLOR");
    c
}

/// Run the binary and capture its output, failing loudly if it cannot spawn.
fn run(home: &TempHome, args: &[&str]) -> Output {
    cmd(home, args)
        .output()
        .unwrap_or_else(|e| panic!("failed to run `{BIN}` with {args:?}: {e}"))
}

/// Decode captured bytes to a `String` (lossy, so invalid UTF-8 never panics).
fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

/// Convenience: combined stdout for assertions, plus a debug rendering of the
/// whole result for failure messages.
fn stdout(out: &Output) -> String {
    text(&out.stdout)
}

fn describe(args: &[&str], out: &Output) -> String {
    format!(
        "command: coffeebreak {args:?}\nstatus: {}\n--- stdout ---\n{}\n--- stderr ---\n{}",
        out.status,
        text(&out.stdout),
        text(&out.stderr),
    )
}

#[test]
fn version_prints_name_and_number() {
    let home = TempHome::new("version");
    let args = ["--version"];
    let out = run(&home, &args);

    assert!(out.status.success(), "--version should exit 0\n{}", describe(&args, &out));
    let s = stdout(&out);
    let s = s.trim();
    assert!(
        s.starts_with("coffeebreak "),
        "expected `--version` to start with `coffeebreak `, got: {s:?}\n{}",
        describe(&args, &out)
    );
    // Something that looks like a version (e.g. starts with a digit) follows.
    let rest = s.trim_start_matches("coffeebreak ").trim();
    assert!(
        rest.chars().next().is_some_and(|c| c.is_ascii_digit()),
        "expected a version number after `coffeebreak `, got: {s:?}\n{}",
        describe(&args, &out)
    );
}

#[test]
fn help_mentions_pomodoro() {
    let home = TempHome::new("help");
    let args = ["--help"];
    let out = run(&home, &args);

    assert!(out.status.success(), "--help should exit 0\n{}", describe(&args, &out));
    let s = stdout(&out);
    assert!(
        s.contains("Pomodoro"),
        "expected `--help` to mention `Pomodoro`\n{}",
        describe(&args, &out)
    );
}

#[test]
fn presets_lists_all_presets() {
    let home = TempHome::new("presets");
    let args = ["presets"];
    let out = run(&home, &args);

    assert!(out.status.success(), "`presets` should exit 0\n{}", describe(&args, &out));
    let s = stdout(&out);
    for name in ["classic", "deep", "short", "sprint"] {
        assert!(
            s.contains(name),
            "expected `presets` output to list preset `{name}`\n{}",
            describe(&args, &out)
        );
    }
}

#[test]
fn themes_lists_all_themes() {
    let home = TempHome::new("themes");
    // `--no-color` keeps the output free of escape codes so the names are plain.
    let args = ["themes", "--no-color"];
    let out = run(&home, &args);

    assert!(out.status.success(), "`themes` should exit 0\n{}", describe(&args, &out));
    let s = stdout(&out);
    for name in ["coffee", "ocean", "forest", "grape", "mono"] {
        assert!(
            s.contains(name),
            "expected `themes` output to list theme `{name}`\n{}",
            describe(&args, &out)
        );
    }
}

#[test]
fn completions_bash_outputs_a_script() {
    let home = TempHome::new("completions");
    let args = ["completions", "bash"];
    let out = run(&home, &args);

    assert!(out.status.success(), "`completions bash` should exit 0\n{}", describe(&args, &out));
    let s = stdout(&out);
    assert!(
        s.contains("coffeebreak"),
        "expected a bash completion script mentioning `coffeebreak`\n{}",
        describe(&args, &out)
    );
}

#[test]
fn man_outputs_roff() {
    let home = TempHome::new("man");
    let args = ["man"];
    let out = run(&home, &args);

    assert!(out.status.success(), "`man` should exit 0\n{}", describe(&args, &out));
    let s = stdout(&out);
    assert!(
        s.contains(".TH"),
        "expected `man` to emit a roff page containing the `.TH` macro\n{}",
        describe(&args, &out)
    );
}

#[test]
fn config_path_points_at_config_toml() {
    let home = TempHome::new("config-path");
    let args = ["config", "path"];
    let out = run(&home, &args);

    assert!(out.status.success(), "`config path` should exit 0\n{}", describe(&args, &out));
    let s = stdout(&out);
    let s = s.trim();
    assert!(
        s.ends_with("config.toml"),
        "expected `config path` to end with `config.toml`, got: {s:?}\n{}",
        describe(&args, &out)
    );
}

#[test]
fn config_init_then_show_round_trips() {
    let home = TempHome::new("config-roundtrip");

    // First `init` creates the file.
    let init_args = ["config", "init"];
    let init = run(&home, &init_args);
    assert!(
        init.status.success(),
        "`config init` should exit 0\n{}",
        describe(&init_args, &init)
    );

    // The file should now exist on disk under the temp home.
    let cfg_path = home.path().join(".config/coffeebreak/config.toml");
    assert!(
        cfg_path.exists(),
        "expected `config init` to create {}\n{}",
        cfg_path.display(),
        describe(&init_args, &init)
    );

    // `show` renders the effective config, which must round-trip the default theme.
    let show_args = ["config", "show"];
    let show = run(&home, &show_args);
    assert!(
        show.status.success(),
        "`config show` should exit 0\n{}",
        describe(&show_args, &show)
    );
    let s = stdout(&show);
    assert!(
        s.contains(r#"theme = "coffee""#),
        "expected `config show` to contain `theme = \"coffee\"`\n{}",
        describe(&show_args, &show)
    );
}

#[test]
fn invalid_preset_fails_with_message() {
    let home = TempHome::new("bad-preset");
    let args = ["--preset", "nope"];
    let out = run(&home, &args);

    assert!(
        !out.status.success(),
        "an invalid `--preset` should exit non-zero\n{}",
        describe(&args, &out)
    );
    // clap reports the error on stderr; check both streams to be robust.
    let combined = format!("{}{}", text(&out.stdout), text(&out.stderr));
    assert!(
        combined.contains("invalid value"),
        "expected an `invalid value` error for `--preset nope`\n{}",
        describe(&args, &out)
    );
}

#[test]
fn stats_with_empty_home_reports_nothing_yet() {
    let home = TempHome::new("empty-stats");
    let args = ["--stats"];
    let out = run(&home, &args);

    assert!(out.status.success(), "`--stats` should exit 0\n{}", describe(&args, &out));
    let s = stdout(&out);
    assert!(
        s.contains("No pomodoros") || s.to_lowercase().contains("statistics"),
        "expected empty stats to mention `No pomodoros` (or statistics)\n{}",
        describe(&args, &out)
    );
}

#[test]
fn quick_plain_run_records_a_pomodoro() {
    let home = TempHome::new("quick-run");
    // `--seconds` reinterprets `-w 1` as a one-second focus block; one cycle has
    // no trailing break, so this finishes almost immediately. The piped stdout
    // forces plain mode regardless of `--plain`, so the process cannot hang.
    let args = [
        "--plain",
        "--seconds",
        "-w",
        "1",
        "--cycles",
        "1",
        "--no-sound",
        "--no-notify",
    ];
    let out = run(&home, &args);

    assert!(
        out.status.success(),
        "a quick plain run should exit 0\n{}",
        describe(&args, &out)
    );

    // Stats are written to `$HOME/.coffeebreak/stats.json`.
    let stats_path = home.path().join(".coffeebreak/stats.json");
    assert!(
        stats_path.exists(),
        "expected a quick run to create {}\n{}",
        stats_path.display(),
        describe(&args, &out)
    );

    let contents = std::fs::read_to_string(&stats_path)
        .unwrap_or_else(|e| panic!("could not read {}: {e}", stats_path.display()));
    assert!(
        contents.contains("completed_pomodoros"),
        "expected stats.json to contain `completed_pomodoros`, got:\n{contents}\n{}",
        describe(&args, &out)
    );
    // Parse the count out of the JSON (no serde available here): find the field
    // and read the integer after it, asserting at least one pomodoro landed.
    let count = parse_first_completed_pomodoros(&contents).unwrap_or_else(|| {
        panic!(
            "could not find a `completed_pomodoros` value in stats.json:\n{contents}\n{}",
            describe(&args, &out)
        )
    });
    assert!(
        count >= 1,
        "expected completed_pomodoros >= 1, got {count}\nstats.json:\n{contents}\n{}",
        describe(&args, &out)
    );
}

/// Minimal, dependency-free extraction of the first `completed_pomodoros`
/// integer value from the stats JSON text.
fn parse_first_completed_pomodoros(json: &str) -> Option<u64> {
    let key = "\"completed_pomodoros\"";
    let start = json.find(key)? + key.len();
    let after = &json[start..];
    // Skip everything up to and including the colon, then any whitespace.
    let colon = after.find(':')?;
    let digits: String = after[colon + 1..]
        .trim_start()
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect();
    digits.parse().ok()
}
