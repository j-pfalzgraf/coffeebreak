# coffeebreak ☕

[![CI](https://github.com/j-pfalzgraf/coffeebreak/actions/workflows/ci.yml/badge.svg)](https://github.com/j-pfalzgraf/coffeebreak/actions/workflows/ci.yml)
[![Security audit](https://github.com/j-pfalzgraf/coffeebreak/actions/workflows/audit.yml/badge.svg)](https://github.com/j-pfalzgraf/coffeebreak/actions/workflows/audit.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)

**A Pomodoro focus timer for your terminal — with a live, animated coffee cup that *drains* while you focus and *refills* on your break, behind big block-digit countdowns.**

Start a focus block and watch the cup empty cup by cup, steam curling off the surface, the liquid shimmering, a gradient progress bar filling at the bottom — all in a full-screen terminal UI. When the break comes, the cup pours back full. No window, no browser, no distraction. Just you, the terminal, and the next cup.

---

## What it looks like

coffeebreak takes over the alternate screen and renders a single, living frame:

- a **coffee cup that drains during focus and refills during a break**, the liquid level tracking the time left;
- **animated steam** rising off the top and a **shimmering liquid surface** that ripples frame to frame;
- a **large block-digit countdown** of the time remaining in the current phase;
- a **gradient progress bar** showing how far through the phase you are;
- the active **theme colours**, session label, cycle counter, and a status line for controls.

When stdout/stdin aren't TTYs (pipes, CI) — or with `--plain` — it drops to clean, line-based output instead.

```text
  ╭──────────────────────────────────────────────╮
  │  coffeebreak · FOCUS · "refactor-auth" · 2/4  │
  │                                                │
  │            ~  ~~   ~                           │
  │             ~   ~~    ~      ████  ████        │
  │            (  steam  )       █  █  █  █        │
  │           .-=========-.      █  █     █        │
  │           |~~~~~~~~~~~|      █  █  █  █        │
  │           |▓▓▓▓▓▓▓▓▓▓▓|      ████  ████        │
  │           |▓▓▓▓▓▓▓▓▓▓▓|                        │
  │           |░░░░░░░░░░░|      1 8 : 0 7         │
  │           |░░░░░░░░░░░|                        │
  │           '-_________-'                        │
  │                                                │
  │  ▰▰▰▰▰▰▰▰▰▰▰▰▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱  28%       │
  │                                                │
  │  space pause · s skip · ±1 min · q quit        │
  ╰──────────────────────────────────────────────╯
```

> Representative mock — the real UI is in truecolour, animates every frame, and the cup level moves continuously as time passes.

---

## Features

**Animated UI, with a plain fallback**
- Full-screen animated coffee cup on the alternate screen: drains on focus, refills on break.
- Animated steam, a shimmering liquid surface, big block-digit countdown, and a gradient progress bar.
- Automatic **plain line output** when stdout/stdin aren't TTYs (pipes/CI), or force it with `--plain`.
- Tunable frame rate with `--fps N` (2–60, default 15); near-zero idle CPU between frames.

**Interactive keyboard controls**
- Pause / resume, skip a phase, add or remove a minute on the fly, and quit — all without leaving the session.

**Themes**
- Five truecolour themes: `coffee` (default), `ocean`, `forest`, `grape`, `mono`. Preview them with `coffeebreak themes`.

**Presets**
- Four built-in cadences via `--preset NAME`: `classic`, `deep`, `short`, `sprint`. List them with `coffeebreak presets`.

**Statistics & streaks**
- Daily stats in `~/.coffeebreak/stats.json`: today, all-time, current streak, and best day. View with `coffeebreak stats` or `--stats`.

**Long breaks**
- Automatically take a longer break after every N focus blocks, with configurable length.

**Git labels**
- Tag a session with `-l/--label TEXT`, or use the current git branch with `--git-label`.

**Notifications & sound**
- Desktop notification on each phase change (`--no-notify` to disable).
- Terminal bell by default; build with `--features sound` for a rodio chime. `--no-sound` mutes.

**Completions & man page**
- Shell completions for bash, zsh, fish, PowerShell, and elvish; a generated man page.

**Self-update lifecycle**
- `coffeebreak self update [--check]` and `coffeebreak self uninstall [-y]` — updates only ever run on an explicit command.

**Solid by default**
- Near-zero idle CPU, clean Ctrl+C / quit that always saves your stats, and works with no config file at all.

---

## Install

The installers download the release asset **and** the `SHA256SUMS` file, then
verify the asset's SHA-256 checksum **before** installing. They print the
resolved version and source URL first, and abort on any mismatch.

### Unix (Linux / macOS)

```sh
curl -fsSL https://raw.githubusercontent.com/j-pfalzgraf/coffeebreak/main/install.sh | sh
```

This installs to `~/.local/bin/coffeebreak`. Override the location with
`COFFEEBREAK_INSTALL_DIR`, or pin a version with `COFFEEBREAK_VERSION`:

```sh
COFFEEBREAK_INSTALL_DIR="$HOME/bin" COFFEEBREAK_VERSION=v0.1.0 \
  curl -fsSL https://raw.githubusercontent.com/j-pfalzgraf/coffeebreak/main/install.sh | sh
```

If the install directory isn't on your `PATH`, the script prints a hint to add it.

### Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/j-pfalzgraf/coffeebreak/main/install.ps1 | iex
```

This installs to `%LOCALAPPDATA%\Programs\coffeebreak\coffeebreak.exe` and adds
that directory to your user `PATH`. Restart your shell afterwards so the new
`PATH` takes effect.

### Cargo

```sh
cargo install coffeebreak-cli
```

The published crate is `coffeebreak-cli`; the installed command is `coffeebreak`.

### Homebrew

```sh
brew install j-pfalzgraf/tap/coffeebreak
```

> Note: the Homebrew tap may not exist yet — if the formula isn't published,
> use one of the methods above.

---

## Usage

Run `coffeebreak` with no arguments for a 25-minute focus / 5-minute break, one
cycle, in the full-screen animated UI. Command-line flags always take precedence
over the config file.

### Flags

| Flag                    | Description                                              |
| ----------------------- | ------------------------------------------------------- |
| `-w, --work MIN`        | Focus block length, in minutes                          |
| `-b, --break MIN`       | Short break length, in minutes                          |
| `--cycles N`            | Number of focus→break rounds                            |
| `--goal N`              | Daily pomodoro goal shown in the stats dashboard        |
| `--wait`                | Wait for a keypress between phases instead of auto-advancing |
| `--long`                | Enable a long break after every N focus blocks          |
| `--long-break MIN`      | Long-break length, in minutes (implies `--long`)        |
| `--long-every N`        | Focus blocks before a long break (default 4)            |
| `--preset NAME`         | Use a built-in cadence: `classic`, `deep`, `short`, `sprint` |
| `--theme NAME`          | Colour theme: `coffee`, `ocean`, `forest`, `grape`, `mono`   |
| `-l, --label TEXT`      | Session label shown in the status line                  |
| `--git-label`           | Use the current git branch as the label                 |
| `--plain`               | Force plain line output (no animated UI)                |
| `--fps N`               | Animation frame rate, 2–60 (default 15)                 |
| `--lang CODE`           | Interface language: `en`, `de`, `es`, `fr`, `it`, `pt`  |
| `--stats`               | Show today / all-time / current streak / best day       |
| `--no-sound`            | Mute the audible cue                                     |
| `--no-notify`           | Disable desktop notifications                           |
| `--no-color`            | Disable coloured output (also honours `NO_COLOR`)       |
| `--version`             | Print the version                                       |

### Subcommands

| Subcommand                              | Description                                                      |
| --------------------------------------- | ---------------------------------------------------------------- |
| `coffeebreak stats`                     | Show today / all-time / current streak / best day               |
| `coffeebreak config init`               | Write a starter config file with all defaults                    |
| `coffeebreak config path`               | Print the resolved config file path                              |
| `coffeebreak config show`              | Print the effective configuration                                |
| `coffeebreak themes`                    | Preview the five colour themes                                   |
| `coffeebreak presets`                   | List the built-in presets and their timings                     |
| `coffeebreak languages`                 | List the interface languages (marks the active one)             |
| `coffeebreak doctor`                    | Environment diagnostics (terminal, colour, locale, config, …)   |
| `coffeebreak completions <shell>`       | Emit completions for `bash`, `zsh`, `fish`, `powershell`, `elvish` |
| `coffeebreak man`                       | Emit the man page                                               |
| `coffeebreak self update [--check]`     | Update to the latest release (`--check` only checks)            |
| `coffeebreak self uninstall [-y]`       | Remove binary + config dir + data dir (`-y` skips the prompt)   |

### Interactive controls

During a session in the animated UI:

| Key(s)                  | Action                                |
| ----------------------- | ------------------------------------- |
| `space` / `p`           | Pause / resume                        |
| `s` / `n`               | Skip to the next phase                |
| `+` / `=` / `Up`        | Add a minute to the current phase     |
| `-` / `_` / `Down`      | Remove a minute from the current phase|
| `q` / `Esc` / `Ctrl+C`  | Quit (stats are saved)                |

With `--wait` (or `auto_advance = false` in the config), the timer pauses on an
animated "press any key to continue" screen between phases instead of starting
the next one automatically — handy if you don't want breaks to begin the moment
you step away. Piped/non-interactive runs always auto-advance.

### Examples

```sh
# Classic Pomodoro cadence via a preset
coffeebreak --preset classic

# Deep-work session in the ocean theme, labelled by git branch
coffeebreak --preset deep --theme ocean --git-label

# Custom deep work: 50/10, four rounds, with a long break
coffeebreak -w 50 -b 10 --cycles 4 --long --long-break 20 --long-every 3

# Run inside a pipe / CI with plain line output, no sound
coffeebreak --plain --no-sound

# Try the forest theme at a smoother frame rate
coffeebreak --theme forest --fps 30

# Peek at your progress without starting a timer
coffeebreak --stats
```

---

## Themes

Five truecolour themes are built in. Preview them all with `coffeebreak themes`,
then select one with `--theme NAME` or the `theme` config key.

- **coffee** — warm browns and creams (default)
- **ocean** — cool blues and teals
- **forest** — greens and earth tones
- **grape** — purples and magentas
- **mono** — grayscale, minimal

---

## Presets

Four cadences are built in. Use them with `--preset NAME`, or list them with
`coffeebreak presets`.

| Preset    | Cadence                                           |
| --------- | ------------------------------------------------- |
| `classic` | 4 × 25/5, finishing with a 15 min long break      |
| `deep`    | 3 × 50/10, finishing with a 20 min long break     |
| `short`   | 6 × 15/3, with a 10 min long break every 4 blocks |
| `sprint`  | 1 × 20/5 (no long break)                          |

---

## Languages

coffeebreak is fully localised and **defaults to English**. It ships interface
translations for English, German, Spanish, French, Italian, and Portuguese.

The language is chosen in this order: the `--lang` flag → the `language` config
key → your `LC_ALL` / `LC_MESSAGES` / `LANG` / `LANGUAGE` environment → English.
Anything not yet translated falls back to English.

```sh
coffeebreak --lang de            # run in German
coffeebreak languages            # list available languages (marks the active one)
LANG=es_ES.UTF-8 coffeebreak     # auto-detected from the environment
```

| Code | Language   |
| ---- | ---------- |
| `en` | English    |
| `de` | Deutsch    |
| `es` | Español    |
| `fr` | Français   |
| `it` | Italiano   |
| `pt` | Português  |

---

## Configuration

coffeebreak works with no config file at all. To set your own defaults, create
`~/.config/coffeebreak/config.toml` (or `$XDG_CONFIG_HOME/coffeebreak/config.toml`
if that variable is set). This location is the **same on Linux, macOS, and
Windows**. Generate a starter file with all defaults using:

```sh
coffeebreak config init
```

The file is validated strictly — only the keys below are accepted, and any key
you omit falls back to its default. Command-line flags always override the file.

```toml
# ~/.config/coffeebreak/config.toml

work_minutes       = 25       # focus block length, in minutes
break_minutes      = 5        # short break length, in minutes
long_break_minutes = 15       # long break length, in minutes
cycles             = 1        # number of focus->break rounds
long_break_every   = 4        # focus blocks before a long break
long_break         = false    # enable long breaks
sound              = true      # play the audible cue at phase changes
notifications      = true      # send a desktop notification at phase changes
git_label          = false    # label the session with the current git branch
theme              = "coffee" # colour theme: coffee, ocean, forest, grape, mono
fps                = 15        # animation frame rate, 2-60
language           = ""       # interface language: en, de, es, fr, it, pt ("" = auto-detect)
daily_goal         = 0        # daily pomodoro goal shown in the stats dashboard (0 = off)
```

Use `coffeebreak config path` to see where the file is resolved, and
`coffeebreak config show` to print the effective configuration.

---

## Statistics

coffeebreak records completed focus blocks to `~/.coffeebreak/stats.json`. Run
`coffeebreak stats` (or `coffeebreak --stats`) for an animated dashboard:

- **Today / All-time** — pomodoros and focus minutes
- **Current streak** & **Longest streak** — consecutive active days
- **Best day** — your most productive day on record
- **Daily goal** — a progress bar toward `--goal N` (or the `daily_goal` config key)
- **Last 14 days** — a vertical bar chart
- **Last 12 weeks** — a GitHub-style contribution heatmap

On a colour terminal the charts grow in with a short reveal animation; piped or
with `--no-color` it prints the final dashboard once. Stats are saved even if you
interrupt a session — quitting with `q`, `Esc`, or Ctrl+C still writes your
progress before exit.

```sh
coffeebreak --goal 8 stats   # dashboard with a goal of 8 pomodoros/day
```

---

## Diagnostics

`coffeebreak doctor` prints a quick, localised environment report — whether the
terminal is interactive, truecolour support, the active language, the config and
data paths, notification availability, and the sound backend — so you can see at
a glance how coffeebreak will behave on your machine.

---

## Sound

By default, coffeebreak uses the **terminal bell** (`\a`) for its audible cue —
no extra dependencies, works everywhere.

For a richer chime, build with the `sound` feature, which pulls in
[`rodio`](https://crates.io/crates/rodio):

```sh
cargo install coffeebreak-cli --features sound
# or, from a source checkout:
cargo build --release --features sound
```

> On Linux, the `sound` feature needs the ALSA development headers. Install
> them first, e.g. `sudo apt install libasound2-dev` (Debian/Ubuntu).

Either way, `--no-sound` mutes the cue entirely.

---

## Lifecycle

Update to the latest GitHub release (verified against `SHA256SUMS`):

```sh
coffeebreak self update
```

Just check whether a newer version is available, without installing:

```sh
coffeebreak self update --check
```

Remove the binary, the config directory, and the data directory. It asks for
confirmation first; pass `-y` to skip the prompt:

```sh
coffeebreak self uninstall
```

You can also uninstall via the standalone script (handy if the binary won't run):

```sh
curl -fsSL https://raw.githubusercontent.com/j-pfalzgraf/coffeebreak/main/uninstall.sh | sh
```

---

## Build from source

```sh
git clone https://github.com/j-pfalzgraf/coffeebreak
cd coffeebreak
cargo build --release             # binary at target/release/coffeebreak
cargo test                        # run the test suite
cargo build --release --features sound   # enable the rodio chime
```

---

## Development & CI

Continuous integration runs on every push and pull request
([`ci.yml`](.github/workflows/ci.yml)):

- **rustfmt** — `cargo fmt --all --check` (formatting is enforced)
- **clippy** — `-D warnings`, with and without the `sound` feature
- **tests** — on Linux, macOS, and Windows
- **MSRV** — `cargo check` on Rust 1.85
- **docs** — `cargo doc` with `-D warnings` (catches broken doc links)

A weekly [security audit](.github/workflows/audit.yml) scans dependencies for
RustSec advisories, and [Dependabot](.github/dependabot.yml) keeps crates and
Actions up to date. Pushing a `v*` tag runs the
[release pipeline](.github/workflows/release.yml) (cross-builds, checksums,
GitHub Release). See [CONTRIBUTING.md](CONTRIBUTING.md) for the full workflow.

---

## Security

- Every release ships a `SHA256SUMS` file listing the SHA-256 of each asset.
- The install scripts download both the asset and `SHA256SUMS`, then **verify
  the checksum before extracting or installing** — and abort on a mismatch.
- All downloads are over HTTPS only.
- Updates run **only on explicit command** (`coffeebreak self update`); the tool
  never updates itself silently in the background.

> **Forking?** The repository owner/name `j-pfalzgraf/coffeebreak` is a
> placeholder used throughout the installers, release workflow, and self-update
> logic. Swap it for your own `owner/name` everywhere before publishing releases.

---

## License

Licensed under either of:

- MIT license ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

at your option.
