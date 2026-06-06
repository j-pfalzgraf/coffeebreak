# Changelog

All notable changes to **coffeebreak** (crate `coffeebreak-cli`, command `coffeebreak`) are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

A major UI and architecture upgrade. coffeebreak now renders a full-screen,
animated terminal interface, reacts to keyboard input mid-session, and ships
themes and presets — all on top of a refactored, modular codebase.

### Added

- **Animated statistics dashboard.** `coffeebreak stats` now renders a daily-goal
  progress bar, a 14-day vertical bar chart, and a GitHub-style 12-week
  contribution heatmap (plus current/longest streak), with a short grow-in reveal
  animation on a colour terminal. New `charts` module (`sparkline`, `bar_chart`,
  `heatmap`, `goal_bar`).
- **Daily goal**: `--goal N` flag and `daily_goal` config key, shown in the
  dashboard.
- **`coffeebreak doctor`** — localised environment diagnostics (terminal,
  truecolour, language, config/data paths, notifications, sound backend).
- **Full internationalisation**, defaulting to English. Every user-facing
  string — the live UI, status line, statistics, command output, notifications,
  footers, and even the `--help`/man text — is localised. Ships English,
  German, Spanish, French, Italian, and Portuguese. The language is resolved from
  `--lang CODE`, the `language` config key, or the `LC_ALL`/`LC_MESSAGES`/`LANG`/
  `LANGUAGE` environment, with English fallback for anything untranslated. Added
  a `coffeebreak languages` subcommand.
- **Animated full-screen TUI.** On a TTY, sessions now run in the alternate
  screen with an ASCII coffee cup that **drains** as you focus and **refills**
  during a break, animated rising steam, a shimmering liquid surface, a large
  block-digit countdown, and a gradient progress bar.
- **Interactive keyboard controls** during a session:
  - `space` / `p` — pause / resume
  - `s` / `n` — skip the current phase
  - `+` / `=` / `Up` — add a minute to the current phase
  - `-` / `_` / `Down` — remove a minute from the current phase
  - `q` / `Esc` / `Ctrl+C` — quit (stats are saved)
- **Colour themes** (truecolour): `coffee` (default), `ocean`, `forest`,
  `grape`, and `mono`. Select with `--theme NAME` or via config.
- **`coffeebreak themes`** subcommand to preview every theme.
- **Presets** via `--preset NAME`: `classic` (4×25/5, ending on a long break),
  `deep` (3×50/10, ending on a long break), `short` (6×15/3 with a long break
  every 4 blocks), and `sprint` (1×20/5, no long break).
- **`coffeebreak presets`** subcommand to list available presets.
- **`coffeebreak config init|path|show`** subcommands to scaffold, locate, and
  inspect the config file.
- **`coffeebreak completions <bash|zsh|fish|powershell|elvish>`** for shell
  completion scripts.
- **`coffeebreak man`** to emit a man page.
- New flags: `--fps N` (2–60, default 15) to tune animation smoothness,
  `--plain` to force plain line output, and `--theme NAME` to pick a theme.
- New config keys: `theme` (string, `"coffee"`) and `fps` (u32, `15`).
- Expanded collection of developer quotes shown between phases.

### Changed

- **Replaced `indicatif`** with a custom, flicker-free renderer that diffs and
  redraws only what changed, keeping idle CPU near zero.
- **OOP/DRY refactor** splitting the monolith into focused modules:
  `theme`, `render`, `widgets`, `app`, `feedback`, and `clock`.
- Plain line output is now selected **automatically** when stdout/stdin are not
  TTYs (pipes, CI), and can be forced with `--plain`.

## [0.1.0] - TBD

Initial release — the original Pomodoro MVP.

### Added

- Pomodoro focus cycles with configurable work and break durations
  (`-w/--work`, `-b/--break`, `--cycles`).
- Long breaks (`--long`, `--long-break`, `--long-every`).
- An ASCII coffee cup and a live countdown progress bar.
- Desktop notifications on phase changes.
- Developer quotes shown between phases.
- Daily stats with streak and best-day tracking in `~/.coffeebreak/stats.json`,
  viewable via `coffeebreak stats` or `--stats`.
- Session labels: `-l/--label TEXT` and `--git-label` (uses the current git
  branch).
- Terminal-bell sound by default, with an optional `rodio` chime behind the
  `sound` build feature; `--no-sound` to mute.
- `--no-color` / `NO_COLOR` support.
- Self-update lifecycle: `coffeebreak self update [--check]` and
  `coffeebreak self uninstall [-y]`.
- Install/uninstall scripts for Unix (`install.sh`, `uninstall.sh`) and Windows
  (`install.ps1`), plus `cargo install coffeebreak-cli`.
- Continuous integration.

[Unreleased]: https://github.com/j-pfalzgraf/coffeebreak/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/j-pfalzgraf/coffeebreak/releases/tag/v0.1.0
