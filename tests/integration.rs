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
use std::process::{Command, Output, Stdio};
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
        .env_remove("NO_COLOR")
        // Clear the locale environment so output is deterministic (English)
        // regardless of the developer's/CI's locale; tests that need another
        // language pass `--lang` explicitly, which overrides this.
        .env_remove("LANG")
        .env_remove("LANGUAGE")
        .env_remove("LC_ALL")
        .env_remove("LC_MESSAGES");
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

    assert!(
        out.status.success(),
        "--version should exit 0\n{}",
        describe(&args, &out)
    );
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

    assert!(
        out.status.success(),
        "--help should exit 0\n{}",
        describe(&args, &out)
    );
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

    assert!(
        out.status.success(),
        "`presets` should exit 0\n{}",
        describe(&args, &out)
    );
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

    assert!(
        out.status.success(),
        "`themes` should exit 0\n{}",
        describe(&args, &out)
    );
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

    assert!(
        out.status.success(),
        "`completions bash` should exit 0\n{}",
        describe(&args, &out)
    );
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

    assert!(
        out.status.success(),
        "`man` should exit 0\n{}",
        describe(&args, &out)
    );
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

    assert!(
        out.status.success(),
        "`config path` should exit 0\n{}",
        describe(&args, &out)
    );
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
fn config_set_and_get_round_trip() {
    let home = TempHome::new("config-set");

    // `set` creates the file on first use (no `init` required).
    let set_args = ["config", "set", "work_minutes", "50"];
    let out = run(&home, &set_args);
    assert!(out.status.success(), "{}", describe(&set_args, &out));
    assert!(
        stdout(&out).contains("work_minutes = 50"),
        "expected a confirmation\n{}",
        describe(&set_args, &out)
    );

    let get_args = ["config", "get", "work_minutes"];
    let out = run(&home, &get_args);
    assert!(out.status.success(), "{}", describe(&get_args, &out));
    assert_eq!(stdout(&out).trim(), "50", "{}", describe(&get_args, &out));

    // The new value must drive the next run (visible in the plan summary).
    let cfg = std::fs::read_to_string(home.path().join(".config/coffeebreak/config.toml"))
        .expect("config.toml should exist after `config set`");
    assert!(cfg.contains("work_minutes = 50"), "config was: {cfg}");
}

#[test]
fn config_set_rejects_invalid_values() {
    let home = TempHome::new("config-set-bad");
    for bad in [
        ["config", "set", "fps", "99"],
        ["config", "set", "theme", "sepia"],
        ["config", "set", "no_such_key", "1"],
    ] {
        let out = run(&home, &bad);
        assert!(
            !out.status.success(),
            "expected failure\n{}",
            describe(&bad, &out)
        );
        let combined = format!("{}{}", stdout(&out), text(&out.stderr));
        assert!(
            combined.contains("invalid value") || combined.contains("unknown config key"),
            "{}",
            describe(&bad, &out)
        );
    }
    // A rejected set must not create or corrupt the config file.
    assert!(
        !home.path().join(".config/coffeebreak/config.toml").exists(),
        "a failed `config set` must not write the config file"
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

    assert!(
        out.status.success(),
        "`--stats` should exit 0\n{}",
        describe(&args, &out)
    );
    let s = stdout(&out);
    assert!(
        s.contains("No pomodoros") || s.to_lowercase().contains("statistics"),
        "expected empty stats to mention `No pomodoros` (or statistics)\n{}",
        describe(&args, &out)
    );
}

#[test]
fn corrupt_stats_are_quarantined_not_overwritten() {
    let home = TempHome::new("corrupt-stats");
    let data_dir = home.path().join(".coffeebreak");
    std::fs::create_dir_all(&data_dir).unwrap();
    let stats_path = data_dir.join("stats.json");
    std::fs::write(&stats_path, "{ this is not json").unwrap();

    // Reading stats must not fail the command, and the corrupt file must be
    // preserved under a backup name rather than silently clobbered later.
    let args = ["--stats"];
    let out = run(&home, &args);
    assert!(
        out.status.success(),
        "--stats should survive a corrupt stats file\n{}",
        describe(&args, &out)
    );
    let backup = data_dir.join("stats.json.corrupt");
    assert!(
        backup.exists(),
        "expected the corrupt file to be moved to {}\n{}",
        backup.display(),
        describe(&args, &out)
    );
    assert_eq!(
        std::fs::read_to_string(&backup).unwrap(),
        "{ this is not json",
        "backup must preserve the original bytes"
    );

    // A subsequent run starts fresh and writes a valid stats file.
    let run_args = [
        "--plain",
        "--seconds",
        "-w",
        "1",
        "--cycles",
        "1",
        "--no-sound",
        "--no-notify",
    ];
    let out = run(&home, &run_args);
    assert!(out.status.success(), "{}", describe(&run_args, &out));
    let contents = std::fs::read_to_string(&stats_path).unwrap_or_default();
    assert!(
        contents.contains("completed_pomodoros"),
        "expected a fresh, valid stats.json after the quarantine, got:\n{contents}"
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

// --- Internationalisation ---------------------------------------------------

#[test]
fn languages_lists_all_locales() {
    let home = TempHome::new("langs");
    let args = ["languages", "--no-color"];
    let out = run(&home, &args);
    assert!(out.status.success(), "{}", describe(&args, &out));
    let s = stdout(&out);
    for code in ["en", "de", "es", "fr", "it", "pt", "nl"] {
        assert!(
            s.contains(code),
            "languages output missing `{code}`\n{}",
            describe(&args, &out)
        );
    }
    assert!(
        s.contains("Deutsch") && s.contains("Português") && s.contains("Nederlands"),
        "{}",
        describe(&args, &out)
    );
}

#[test]
fn lang_flag_localizes_help() {
    let home = TempHome::new("helplang");
    let en = run(&home, &["--lang", "en", "--help"]);
    let de = run(&home, &["--lang", "de", "--help"]);
    assert!(en.status.success() && de.status.success());
    // German help must differ from English (localisation actually applied).
    assert_ne!(
        stdout(&en),
        stdout(&de),
        "German --help should differ from English"
    );
}

#[test]
fn invalid_language_is_rejected() {
    let home = TempHome::new("badlang");
    let args = ["--lang", "xx", "--help"];
    let out = run(&home, &args);
    assert!(
        !out.status.success(),
        "invalid --lang should fail\n{}",
        describe(&args, &out)
    );
    let combined = format!("{}{}", stdout(&out), text(&out.stderr));
    assert!(
        combined.contains("invalid value"),
        "{}",
        describe(&args, &out)
    );
}

#[test]
fn german_run_writes_stats_and_localizes_footer() {
    let home = TempHome::new("derun");
    let args = [
        "--lang",
        "de",
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
    assert!(out.status.success(), "{}", describe(&args, &out));
    let stats = home.path().join(".coffeebreak").join("stats.json");
    let contents = std::fs::read_to_string(&stats).unwrap_or_default();
    assert!(
        contents.contains("completed_pomodoros"),
        "expected stats written\n{}",
        describe(&args, &out)
    );
}

// --- Dashboard & doctor -----------------------------------------------------

#[test]
fn doctor_reports_environment() {
    let home = TempHome::new("doctor");
    let args = ["doctor", "--no-color"];
    let out = run(&home, &args);
    assert!(out.status.success(), "{}", describe(&args, &out));
    let s = stdout(&out);
    for needle in ["Language", "Config file", "Data directory", "Sound"] {
        assert!(
            s.contains(needle),
            "doctor output missing `{needle}`\n{}",
            describe(&args, &out)
        );
    }
}

#[test]
fn stats_dashboard_renders_charts_after_a_run() {
    let home = TempHome::new("dash");
    // Complete one focus block so there is data, then view the dashboard.
    let run_args = [
        "--plain",
        "--seconds",
        "-w",
        "1",
        "--cycles",
        "1",
        "--no-sound",
        "--no-notify",
    ];
    assert!(run(&home, &run_args).status.success());

    let args = ["--goal", "4", "stats", "--no-color"];
    let out = run(&home, &args);
    assert!(out.status.success(), "{}", describe(&args, &out));
    let s = stdout(&out);
    for needle in ["Daily goal:", "Last 14 days", "Last 12 weeks"] {
        assert!(
            s.contains(needle),
            "dashboard missing `{needle}`\n{}",
            describe(&args, &out)
        );
    }
}

#[test]
fn wait_mode_with_non_tty_stdin_auto_advances_and_does_not_hang() {
    // `--wait` waits for a keypress only on an interactive stdin. With piped/null
    // stdin it must auto-advance so scripts and CI never block.
    let home = TempHome::new("wait");
    let mut c = cmd(
        &home,
        &[
            "--wait",
            "--plain",
            "--seconds",
            "-w",
            "1",
            "-b",
            "1",
            "--cycles",
            "2",
            "--no-sound",
            "--no-notify",
        ],
    );
    c.stdin(Stdio::null());
    let out = c.output().unwrap_or_else(|e| panic!("failed to run: {e}"));
    assert!(
        out.status.success(),
        "wait+piped should complete, not hang\n{}",
        text(&out.stderr)
    );
    let contents = std::fs::read_to_string(home.path().join(".coffeebreak").join("stats.json"))
        .unwrap_or_default();
    assert!(
        contents.contains("completed_pomodoros"),
        "expected stats written"
    );
}

// --- Stats export formats ---------------------------------------------------

#[test]
fn stats_json_export_is_structured() {
    let home = TempHome::new("statsjson");
    // Produce one completed focus block, then export JSON.
    let run_args = [
        "--plain",
        "--seconds",
        "-w",
        "1",
        "--cycles",
        "1",
        "--no-sound",
        "--no-notify",
    ];
    assert!(run(&home, &run_args).status.success());

    let args = ["stats", "--format", "json"];
    let out = run(&home, &args);
    assert!(out.status.success(), "{}", describe(&args, &out));
    let s = stdout(&out);
    let s = s.trim();
    assert!(
        s.starts_with('{') && s.ends_with('}'),
        "expected a JSON object\n{}",
        describe(&args, &out)
    );
    for key in [
        "\"summary\"",
        "\"total_pomodoros\"",
        "\"days\"",
        "\"current_streak\"",
    ] {
        assert!(
            s.contains(key),
            "json missing {key}\n{}",
            describe(&args, &out)
        );
    }
}

#[test]
fn stats_csv_export_has_header_and_rows() {
    let home = TempHome::new("statscsv");
    let run_args = [
        "--plain",
        "--seconds",
        "-w",
        "1",
        "--cycles",
        "1",
        "--no-sound",
        "--no-notify",
    ];
    assert!(run(&home, &run_args).status.success());

    let args = ["stats", "--format", "csv"];
    let out = run(&home, &args);
    assert!(out.status.success(), "{}", describe(&args, &out));
    let s = stdout(&out);
    let mut lines = s.lines();
    assert_eq!(
        lines.next(),
        Some("date,completed_pomodoros,focus_minutes"),
        "{}",
        describe(&args, &out)
    );
    // At least one data row, shaped date,int,int.
    let row = lines.next().unwrap_or("");
    let cols: Vec<&str> = row.split(',').collect();
    assert_eq!(
        cols.len(),
        3,
        "expected 3 CSV columns, got {row:?}\n{}",
        describe(&args, &out)
    );
    assert!(
        cols[1].parse::<u64>().is_ok(),
        "completed_pomodoros not an int: {row:?}"
    );
}

#[test]
fn stats_invalid_format_is_rejected() {
    let home = TempHome::new("statsfmt");
    let args = ["stats", "--format", "xml"];
    let out = run(&home, &args);
    assert!(
        !out.status.success(),
        "invalid --format should fail\n{}",
        describe(&args, &out)
    );
    let combined = format!("{}{}", stdout(&out), text(&out.stderr));
    assert!(
        combined.contains("invalid value"),
        "{}",
        describe(&args, &out)
    );
}

// --- Session history ---------------------------------------------------------

#[test]
fn history_is_off_by_default() {
    let home = TempHome::new("hist-off");
    let run_args = [
        "--plain",
        "--seconds",
        "-w",
        "1",
        "--cycles",
        "1",
        "--no-sound",
        "--no-notify",
    ];
    assert!(run(&home, &run_args).status.success());
    assert!(
        !home.path().join(".coffeebreak/history.jsonl").exists(),
        "history.jsonl must not be created unless history = true is configured"
    );

    // The command still works, with a friendly empty state.
    let args = ["history", "--no-color"];
    let out = run(&home, &args);
    assert!(out.status.success(), "{}", describe(&args, &out));
    assert!(
        stdout(&out).contains("history = true"),
        "empty history should hint at the config key\n{}",
        describe(&args, &out)
    );
}

#[test]
fn history_logs_and_renders_when_enabled() {
    let home = TempHome::new("hist-on");
    let cfg_dir = home.path().join(".config/coffeebreak");
    std::fs::create_dir_all(&cfg_dir).unwrap();
    std::fs::write(cfg_dir.join("config.toml"), "history = true\n").unwrap();

    let run_args = [
        "--plain",
        "--seconds",
        "-w",
        "1",
        "--cycles",
        "1",
        "--no-sound",
        "--no-notify",
        "--label",
        "api-refactor",
    ];
    let out = run(&home, &run_args);
    assert!(out.status.success(), "{}", describe(&run_args, &out));

    let log_path = home.path().join(".coffeebreak/history.jsonl");
    let log = std::fs::read_to_string(&log_path)
        .unwrap_or_else(|e| panic!("expected {} to exist: {e}", log_path.display()));
    assert_eq!(
        log.lines().count(),
        1,
        "one completed block → one line: {log}"
    );
    for needle in [
        "\"ts\"",
        "\"work_min\"",
        "\"api-refactor\"",
        "\"completed\":true",
    ] {
        assert!(
            needle.is_empty() || log.contains(needle),
            "log missing {needle}: {log}"
        );
    }

    // A torn/corrupt trailing line (crash mid-append) must not break the view.
    {
        use std::io::Write;
        let mut f = std::fs::OpenOptions::new()
            .append(true)
            .open(&log_path)
            .unwrap();
        write!(f, "{{ torn line").unwrap();
    }

    let args = ["history", "--no-color"];
    let out = run(&home, &args);
    assert!(out.status.success(), "{}", describe(&args, &out));
    let s = stdout(&out);
    assert!(
        s.contains("api-refactor") && s.contains("1 pomodoro"),
        "history table should show the logged session\n{}",
        describe(&args, &out)
    );
}

// --- Achievements, demo, and the new themes/presets/indicator ---------------

#[test]
fn achievements_board_renders_after_a_run() {
    let home = TempHome::new("ach");
    // Earn the first badge, then view the board.
    let run_args = [
        "--plain",
        "--seconds",
        "-w",
        "1",
        "--cycles",
        "1",
        "--no-sound",
        "--no-notify",
    ];
    assert!(run(&home, &run_args).status.success());

    let args = ["achievements", "--no-color"];
    let out = run(&home, &args);
    assert!(out.status.success(), "{}", describe(&args, &out));
    let s = stdout(&out);
    for needle in ["achievements", "Unlocked:", "First Sip", "First steps"] {
        assert!(
            s.contains(needle),
            "achievements board missing `{needle}`\n{}",
            describe(&args, &out)
        );
    }
}

#[test]
fn achievements_empty_state_is_friendly() {
    let home = TempHome::new("ach-empty");
    let args = ["achievements", "--no-color"];
    let out = run(&home, &args);
    assert!(out.status.success(), "{}", describe(&args, &out));
    let s = stdout(&out);
    assert!(
        s.to_lowercase().contains("no badges") || s.contains("achievements"),
        "expected an empty-state achievements message\n{}",
        describe(&args, &out)
    );
}

#[test]
fn demo_without_a_tty_prints_a_hint_and_exits() {
    // With piped stdout (not a TTY), `demo` cannot animate; it must print a hint
    // and exit cleanly rather than hang or error.
    let home = TempHome::new("demo");
    let args = ["demo", "--no-color"];
    let out = run(&home, &args);
    assert!(
        out.status.success(),
        "`demo` on a non-TTY should exit 0\n{}",
        describe(&args, &out)
    );
    let s = stdout(&out);
    assert!(
        s.to_lowercase().contains("terminal"),
        "expected a non-TTY hint mentioning a terminal\n{}",
        describe(&args, &out)
    );
}

#[test]
fn presets_list_includes_the_new_cadences() {
    let home = TempHome::new("newpresets");
    let args = ["presets", "--no-color"];
    let out = run(&home, &args);
    assert!(out.status.success(), "{}", describe(&args, &out));
    let s = stdout(&out);
    for name in ["5217", "flow", "animedoro"] {
        assert!(
            s.contains(name),
            "presets missing new preset `{name}`\n{}",
            describe(&args, &out)
        );
    }
}

#[test]
fn themes_list_includes_the_new_palettes() {
    let home = TempHome::new("newthemes");
    let args = ["themes", "--no-color"];
    let out = run(&home, &args);
    assert!(out.status.success(), "{}", describe(&args, &out));
    let s = stdout(&out);
    for name in ["dracula", "nord", "gruvbox", "solarized", "rose-pine"] {
        assert!(
            s.contains(name),
            "themes missing new theme `{name}`\n{}",
            describe(&args, &out)
        );
    }
}

#[test]
fn ring_indicator_and_new_theme_complete_a_run() {
    // A new theme + the ring indicator must drive a plain run to completion.
    let home = TempHome::new("ringrun");
    let args = [
        "--plain",
        "--seconds",
        "-w",
        "1",
        "--cycles",
        "1",
        "--no-sound",
        "--no-notify",
        "--theme",
        "nord",
        "--indicator",
        "ring",
    ];
    let out = run(&home, &args);
    assert!(
        out.status.success(),
        "ring + nord run should exit 0\n{}",
        describe(&args, &out)
    );
    let stats_path = home.path().join(".coffeebreak/stats.json");
    assert!(
        stats_path.exists(),
        "expected stats written\n{}",
        describe(&args, &out)
    );
}

#[test]
fn invalid_indicator_is_rejected() {
    let home = TempHome::new("badind");
    let args = ["--indicator", "spiral", "--help"];
    let out = run(&home, &args);
    assert!(
        !out.status.success(),
        "invalid --indicator should fail\n{}",
        describe(&args, &out)
    );
    let combined = format!("{}{}", stdout(&out), text(&out.stderr));
    assert!(
        combined.contains("invalid value"),
        "{}",
        describe(&args, &out)
    );
}
