# Changelog

All notable changes to **coffeebreak** (crate `coffeebreak-cli`, command `coffeebreak`) are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **Opt-in session history** (`history = true` in the config): every completed
  focus block is appended to `~/.coffeebreak/history.jsonl` — one JSON object
  per line (`ts`, `work_min`, `label`, `completed`), created owner-only (0600)
  on Unix. New **`coffeebreak history [--limit N]`** subcommand renders the log
  as a table (`--limit 0` shows everything). A missing file is an empty
  history and a torn/corrupt line is skipped, never an error. Fully localised.
- **`coffeebreak config get <key>`** and **`coffeebreak config set <key>
  <value>`** — read and change individual config keys from the command line.
  Values are validated (type, range, and domain — themes, languages, and
  indicators are checked against the shipped lists) before anything is
  written, and `set` creates the config file on first use.

### Changed

- Completed focus blocks now credit the minutes **actually focused** — the
  interactive `+`/`-` controls can resize a phase mid-run — instead of the
  planned duration.
- `NO_COLOR` handling now follows the [no-color.org](https://no-color.org)
  spec: an empty value no longer disables colour.

### Fixed

- **Data safety:** all saves (`stats.json`, `config.toml`) are now atomic
  (temp file + rename), so a crash or full disk can never leave a torn file;
  personal data files are created owner-only (0600) on Unix. A corrupt
  `stats.json` is quarantined to `stats.json.corrupt` (and reported) instead
  of being silently overwritten by the next save.
- The stats dashboard and achievements reveal animations now also check the
  terminal **height**; previously a short terminal scrolled the output and the
  in-place repaint garbled it.
- Sparklines render a zero as a blank cell again instead of the same glyph as
  the smallest non-zero value.

## [1.1.0] - 2026-06-11

A feature release that builds on the 1.0.0 core: a motivational **achievements**
board, an animation **showcase** (`demo`), five new colour themes and three new
presets, a circular **ring** countdown and an opt-in **brewing** intro, a new
**Dutch** locale, and a broader CI suite — all backward compatible (no config or
data migration needed).

### Added

#### Achievements

- **`coffeebreak achievements`** — a motivational badge board derived entirely
  from your existing statistics (no new saved state). 18 badges across five
  tiers (first steps, volume milestones, streak milestones, single-day feats,
  consistency), with an animated reveal, a mastery bar, and a hint toward your
  next badge. Fully localised.

#### Animations & UI

- **`coffeebreak demo`** — a guided showcase that cycles through every widget
  and animation (brewing, the coffee cup, the countdown, the ring gauge, the
  spinner, the charts, and the celebration). Respects `--theme` and `--lang`.
- **Brewing intro** — opt-in pour-and-steam animation before the first focus
  block (`--brew`, or `brew = true`). Any key skips it.
- **Ring indicator** — `--indicator ring` (or `indicator = "ring"`) swaps the
  big block digits for a circular gauge that fills as the phase elapses, with
  the countdown centred inside.
- **Organic steam** — the cup's steam plume is now a five-wisp system with a
  per-wisp sway and out-of-phase flicker (still deterministic).
- A reusable **spinner** now animates while `self update --check` queries GitHub.

#### Themes & presets

- Five new truecolour themes: **`dracula`**, **`nord`**, **`gruvbox`**,
  **`solarized`**, **`rose-pine`** — preview them with `coffeebreak themes`.
- Three new presets: **`5217`** (the 52/17 rule), **`flow`** (90-minute
  ultradian blocks), and **`animedoro`** (60/20).

#### Internationalisation

- New interface language: **Dutch (`nl`, Nederlands)**.
- Every new string (achievements, demo, brewing, the ring indicator) is
  localised across all seven languages.

### Changed

- The `themes`/`presets` listings now size their columns to the widest name so
  they stay aligned as the lists grow.

### Internal

- Extracted a shared `ui` module (`row_from_cells`, `LineBuf`, `CursorGuard`),
  removing the duplicated row-builder that lived in both `widgets` and `charts`
  and the cursor guard that lived in `stats`.
- The coffee cup is now composed from reusable `steam_rows` + `cup_body` halves.
- New tests: theme/preset name resolution, the new widgets' width invariants,
  the achievements catalogue and rendering, a placeholder-integrity check across
  every translated string, and integration coverage for the new commands.

### CI

- The test matrix now also builds and runs with the `sound` feature on all three
  operating systems (previously `sound` was only ever built by clippy).
- New workflows: **actionlint** (lint the workflows), **cargo-shear** (unused
  dependencies), **cargo-semver-checks** (API breakage on PRs, advisory),
  **lychee** (Markdown link checking — offline on PRs, a weekly online sweep),
  and **direct-minimal-versions** (a scheduled lower-bound check).

## [1.0.0] - 2026-06-08

coffeebreak's first stable release: a full-screen, animated terminal Pomodoro
timer that reacts to keyboard input mid-session and ships themes, presets, full
internationalisation, and a statistics dashboard — all on top of a modular,
well-tested codebase. The original Pomodoro core is included below.

### Added

#### Timer & session

- Pomodoro focus cycles with configurable work and break durations
  (`-w/--work`, `-b/--break`, `--cycles`).
- Long breaks (`--long`, `--long-break`, `--long-every`).
- **Animated full-screen TUI.** On a TTY, sessions run in the alternate screen
  with an ASCII coffee cup that **drains** as you focus and **refills** during a
  break, animated rising steam, a shimmering liquid surface, a large block-digit
  countdown, and a gradient progress bar.
- **Interactive keyboard controls** during a session:
  - `space` / `p` — pause / resume
  - `s` / `n` — skip the current phase
  - `+` / `=` / `Up` — add a minute to the current phase
  - `-` / `_` / `Down` — remove a minute from the current phase
  - `q` / `Esc` / `Ctrl+C` — quit (stats are saved)
- **Manual phase advancement.** `--wait` (or `auto_advance = false` in the
  config) pauses between phases on an animated "press any key to continue"
  screen instead of auto-starting the next phase. Piped/non-interactive runs
  still auto-advance so scripts never block.
- **Presets** via `--preset NAME`: `classic` (4×25/5, ending on a long break),
  `deep` (3×50/10, ending on a long break), `short` (6×15/3 with a long break
  every 4 blocks), and `sprint` (1×20/5, no long break).
- Session labels: `-l/--label TEXT` and `--git-label` (uses the current git
  branch).
- New flags: `--fps N` (2–60, default 15) to tune animation smoothness,
  `--plain` to force plain line output, and `--theme NAME` to pick a theme.

#### Appearance & internationalisation

- **Colour themes** (truecolour): `coffee` (default), `ocean`, `forest`,
  `grape`, and `mono`. Select with `--theme NAME` or via config.
- **Custom colour themes.** Define your own palette under `[custom_theme]` in the
  config (per-field `#RRGGBB` overrides on top of the `coffee` base) and select it
  with `--theme custom`.
- **Full internationalisation**, defaulting to English. Every user-facing
  string — the live UI, status line, statistics, command output, notifications,
  footers, and even the `--help`/man text — is localised. Ships English,
  German, Spanish, French, Italian, and Portuguese. The language is resolved from
  `--lang CODE`, the `language` config key, or the `LC_ALL`/`LC_MESSAGES`/`LANG`/
  `LANGUAGE` environment, with English fallback for anything untranslated.
- Developer quotes shown between phases.

#### Statistics

- Daily stats with streak and best-day tracking in `~/.coffeebreak/stats.json`,
  viewable via `coffeebreak stats` or `--stats`.
- **Animated statistics dashboard.** `coffeebreak stats` renders a daily-goal
  progress bar, a 14-day vertical bar chart, and a GitHub-style 12-week
  contribution heatmap (plus current/longest streak), with a short grow-in reveal
  animation on a colour terminal. New `charts` module (`sparkline`, `bar_chart`,
  `heatmap`, `goal_bar`).
- **Daily goal**: `--goal N` flag and `daily_goal` config key, shown in the
  dashboard.
- **Machine-readable stats export.** `coffeebreak stats --format json|csv` prints
  a structured summary + per-day history (JSON) or rows (CSV) — no colour, no
  animation, pipe-friendly for scripts and dashboards. `text` (the animated
  dashboard) remains the default.

#### Commands, configuration & notifications

- **`coffeebreak config init|path|show`** subcommands to scaffold, locate, and
  inspect the config file.
- **`coffeebreak doctor`** — localised environment diagnostics (terminal,
  truecolour, language, config/data paths, notifications, sound backend).
- **`coffeebreak themes`**, **`coffeebreak presets`**, and **`coffeebreak
  languages`** subcommands to preview themes, list presets, and list locales.
- **`coffeebreak completions <bash|zsh|fish|powershell|elvish>`** for shell
  completion scripts, and **`coffeebreak man`** to emit a man page.
- Config keys: `theme` (string, `"coffee"`) and `fps` (u32, `15`), alongside
  `language`, `daily_goal`, `auto_advance`, and the `[custom_theme]` palette.
- Desktop notifications on phase changes.
- Terminal-bell sound by default, with an optional `rodio` chime behind the
  `sound` build feature; `--no-sound` to mute.
- `--no-color` / `NO_COLOR` support.

#### Distribution

- Self-update lifecycle: `coffeebreak self update [--check]` and
  `coffeebreak self uninstall [-y]`.
- Install/uninstall scripts for Unix (`install.sh`, `uninstall.sh`) and Windows
  (`install.ps1`), plus `cargo install coffeebreak-cli`.

### Engineering

- A custom, flicker-free renderer that diffs and redraws only what changed,
  keeping idle CPU near zero (replacing `indicatif`).
- A modular, OOP/DRY codebase split into focused modules: `theme`, `render`,
  `widgets`, `charts`, `app`, `feedback`, `clock`, `session`, `config`, `cli`,
  `stats`, `commands`, `selfcmd`, and `i18n`.
- Plain line output is selected **automatically** when stdout/stdin are not TTYs
  (pipes, CI), and can be forced with `--plain`.

### CI / Infrastructure

- **Continuous integration** ([`ci.yml`](.github/workflows/ci.yml)): rustfmt,
  clippy (`-D warnings`, with and without the `sound` feature), tests on
  Linux/macOS/Windows, an MSRV (1.88) check, and a docs build that denies broken
  links. Cargo caching throughout.
- **Security audit** ([`audit.yml`](.github/workflows/audit.yml)) scanning for
  RustSec advisories on every dependency change and weekly.
- **Supply-chain policy** via cargo-deny ([`deny.yml`](.github/workflows/deny.yml),
  [`deny.toml`](deny.toml)): advisory, license, banned-crate, and source checks.
- **Code coverage** with `cargo-llvm-cov`
  ([`coverage.yml`](.github/workflows/coverage.yml)); lcov uploaded as an artifact.
- **Spell checking** with `typos` ([`spellcheck.yml`](.github/workflows/spellcheck.yml)).
- **Install-script linting** ([`scripts.yml`](.github/workflows/scripts.yml)):
  shellcheck for the shell installers and PSScriptAnalyzer for `install.ps1`.
- **Dependabot** ([`dependabot.yml`](.github/dependabot.yml)) for Cargo and
  GitHub Actions updates.
- **Hardened release pipeline**: a pre-release gate (tag↔`Cargo.toml` version
  match plus fmt/clippy/tests), release notes drawn from `CHANGELOG.md`,
  **shell completions and a man page** as release assets, **build-provenance
  attestations** for the published archives, and an optional **crates.io publish**
  (gated on a `CARGO_REGISTRY_TOKEN` secret).
- Added issue forms, a pull-request template, and `CONTRIBUTING.md`.

[Unreleased]: https://github.com/j-pfalzgraf/coffeebreak/compare/v1.1.0...HEAD
[1.1.0]: https://github.com/j-pfalzgraf/coffeebreak/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/j-pfalzgraf/coffeebreak/releases/tag/v1.0.0
