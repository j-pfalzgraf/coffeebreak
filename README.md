# coffeebreak ☕

A Pomodoro focus timer that lives in your terminal — an ASCII coffee cup whose
**steam fades** as the timer counts down, a live progress bar, phase-change
notifications, and a fresh developer quote with every break.

> The cup steams when you start a focus block and the wisps thin out the closer
> you get to your break — by the time it's empty, so is the timer.

---

## Features

- **Configurable Pomodoro cycles** — set focus length, break length, and how many focus→break rounds to run.
- **Live countdown + progress bar** — a smooth, in-place rendering of time remaining.
- **Steaming ASCII coffee cup** — the steam fades as the current phase winds down.
- **Phase-change cues** — a desktop notification plus a terminal bell (or an optional rodio chime) when focus or break ends.
- **Dev quote at each break** — a little programming wisdom to enjoy while you step away.
- **Daily statistics with streaks** — track today, all-time, your current streak, and your best day.
- **Long breaks** — automatically take a longer break after every N focus blocks.
- **Git-branch session labels** — tag a session with the current branch name so your status line reflects what you're working on.
- **Near-zero idle CPU** — the timer sleeps between frames instead of spinning.
- **Clean Ctrl+C** — interrupt at any time; your stats are still saved before exit.
- **Self update / uninstall** — pull the latest release or remove everything with a single command.

---

## Install

The installers download the release asset **and** the `SHA256SUMS` file, then
verify the asset's SHA-256 checksum **before** installing. They print the
resolved version and source URL first, and abort on any mismatch.

### Unix (Linux / macOS)

```sh
curl -fsSL https://raw.githubusercontent.com/leuchtturm/coffeebreak/main/install.sh | sh
```

This installs to `~/.local/bin/coffeebreak`. Override the location with
`COFFEEBREAK_INSTALL_DIR`, or pin a version with `COFFEEBREAK_VERSION`:

```sh
COFFEEBREAK_INSTALL_DIR="$HOME/bin" COFFEEBREAK_VERSION=v0.1.0 \
  curl -fsSL https://raw.githubusercontent.com/leuchtturm/coffeebreak/main/install.sh | sh
```

If the install directory isn't on your `PATH`, the script prints a hint to add it.

### Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/leuchtturm/coffeebreak/main/install.ps1 | iex
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
brew install leuchtturm/tap/coffeebreak
```

> Note: the Homebrew tap may not exist yet — if the formula isn't published,
> use one of the methods above.

---

## Usage

Run `coffeebreak` with no arguments for a classic 25-minute focus / 5-minute
break, one cycle.

| Command | Description |
| --- | --- |
| `coffeebreak` | Default 25 min focus / 5 min break, 1 cycle |
| `coffeebreak -w 50 -b 10` | Custom focus / break minutes |
| `coffeebreak --cycles 4` | Run multiple focus→break rounds |
| `coffeebreak --long` | Enable a long break after every N focus blocks |
| `coffeebreak --long-break 20` | Long-break length in minutes (implies `--long`) |
| `coffeebreak --long-every 4` | Focus blocks before a long break (default 4) |
| `coffeebreak -l "label"` | Session label in the status line (alias `--label`) |
| `coffeebreak --git-label` | Use the current git branch as the label |
| `coffeebreak --stats` | Show today / all-time / current streak / best day |
| `coffeebreak --no-sound` | Mute the audible cue |
| `coffeebreak --no-notify` | No desktop notification |
| `coffeebreak --no-color` | Disable coloured output |
| `coffeebreak --version` | Print the version |
| `coffeebreak self update` | Update to the latest GitHub release |
| `coffeebreak self update --check` | Only check whether a newer version exists |
| `coffeebreak self uninstall` | Remove binary + config dir + data dir (asks first; `-y` to skip) |

### Examples

```sh
# A deep-work session: 50/10, four rounds, labelled by git branch
coffeebreak -w 50 -b 10 --cycles 4 --git-label

# Classic flow with a 20-minute long break after every 3 focus blocks
coffeebreak --cycles 8 --long-break 20 --long-every 3

# Quiet mode: no sound, no desktop notification
coffeebreak --no-sound --no-notify

# Peek at your progress without starting a timer
coffeebreak --stats
```

---

## Configuration

coffeebreak works with no config file at all. To set your own defaults, create
`~/.config/coffeebreak/config.toml` (or `$XDG_CONFIG_HOME/coffeebreak/config.toml`
if that variable is set). This location is the same on Linux, macOS, and Windows.
Command-line flags always take precedence over the file.

All keys, with their built-in defaults:

```toml
# ~/.config/coffeebreak/config.toml

work_minutes      = 25     # focus block length, in minutes
break_minutes     = 5      # short break length, in minutes
long_break_minutes = 15    # long break length, in minutes
cycles            = 1      # number of focus->break rounds
long_break_every  = 4      # focus blocks before a long break
long_break        = false  # enable long breaks
sound             = true   # play the audible cue at phase changes
notifications     = true   # send a desktop notification at phase changes
git_label         = false  # label the session with the current git branch
```

The file is optional — any key you omit falls back to the default shown above.

---

## Statistics

coffeebreak records completed focus blocks to `~/.coffeebreak/stats.json`. Run:

```sh
coffeebreak --stats
```

to see:

- **Today** — focus time logged so far today
- **All-time** — your cumulative total
- **Current streak** — consecutive days with at least one completed focus block
- **Best day** — your most productive day on record

Stats are saved even if you interrupt a session with Ctrl+C.

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
curl -fsSL https://raw.githubusercontent.com/leuchtturm/coffeebreak/main/uninstall.sh | sh
```

---

## Build from source

```sh
git clone https://github.com/leuchtturm/coffeebreak
cd coffeebreak
cargo build --release          # binary at target/release/coffeebreak
cargo test                     # run the test suite
```

Add `--features sound` to either command to enable the rodio chime.

---

## Security

- Every release ships a `SHA256SUMS` file listing the SHA-256 of each asset.
- The install scripts download both the asset and `SHA256SUMS`, then **verify
  the checksum before extracting or installing** — and abort on a mismatch.
- All downloads are over HTTPS only.
- Updates run **only on explicit command** (`coffeebreak self update`); the tool
  never updates itself silently in the background.

> **Forking?** The repository owner/name `leuchtturm/coffeebreak` is a
> placeholder used throughout the installers, release workflow, and self-update
> logic. Swap it for your own `owner/name` everywhere before publishing releases.

---

## License

Licensed under either of:

- MIT license ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

at your option.
