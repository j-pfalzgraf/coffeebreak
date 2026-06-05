//! Shell completion scripts and the man page, generated from the clap command.
//!
//! Keeping these generated from the single [`Cli`] definition means they never
//! drift from the real flags and subcommands.

use std::io;

use anyhow::{Context, Result};
use clap_complete::{Shell, generate};

use crate::cli::Cli;

/// Write a completion script for `shell` to stdout.
pub fn print_completions(shell: Shell) {
    let mut cmd = Cli::command();
    let bin = cmd.get_name().to_string();
    generate(shell, &mut cmd, bin, &mut io::stdout());
}

/// Write a roff man page to stdout.
pub fn print_man() -> Result<()> {
    let cmd = Cli::command();
    clap_mangen::Man::new(cmd)
        .render(&mut io::stdout())
        .context("failed to render man page")
}
