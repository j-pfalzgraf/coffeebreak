# Contributing to coffeebreak

Thanks for your interest! coffeebreak is a small, friendly Rust CLI and
contributions are very welcome.

## Prerequisites

- A recent Rust toolchain (the project's MSRV is **1.88**, edition 2024).
- For the optional `sound` feature on Linux: ALSA dev headers
  (`sudo apt install libasound2-dev`).

## Development loop

```sh
cargo build                 # build
cargo test                  # unit + integration + doc tests
cargo run -- --seconds -w 3 -b 2 --cycles 2   # try the timer quickly (seconds, not minutes)
```

Before opening a PR, run the same checks CI does:

```sh
cargo fmt --all --check                              # formatting (enforced)
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

`cargo fmt --all` auto-formats; CI fails on any diff.

## Project layout

Behaviour lives in the library (`src/lib.rs` re-exports the modules); `main.rs`
is a thin entry point. Highlights:

- `app` — the orchestrator (animated TUI + plain fallback).
- `render` / `widgets` / `charts` / `theme` — the rendering engine and visuals.
- `i18n` — localisation (see below).
- `session` / `config` / `cli` — how a run is configured.
- `stats` / `commands` / `selfcmd` — statistics, subcommands, lifecycle.

## Adding or improving a translation

All user-facing text is centralised in [`src/i18n`](src/i18n):

1. English is canonical in `Msg::en` (an exhaustive `match`).
2. Each locale is a `&[(Msg, &str)]` table (`src/i18n/<code>.rs`). Anything a
   locale omits falls back to English, so partial translations are safe.

To add a language, create `src/i18n/<code>.rs`, register it in `mod.rs`
(`LANGUAGES`, `LANG_CODES`, `table`), and translate the `Msg` variants. Keep
`{placeholder}` tokens and technical terms (flags, `coffeebreak`, key names)
unchanged.

## Releases

Pushing a tag like `v0.1.0` triggers [`release.yml`](.github/workflows/release.yml),
which re-checks fmt/clippy/tests, verifies the tag matches `Cargo.toml`,
cross-builds binaries for all supported targets, and publishes a GitHub Release
with a `SHA256SUMS` file. Update `CHANGELOG.md` before tagging.

## License

By contributing you agree that your contributions are licensed under the
project's dual **MIT OR Apache-2.0** license.
