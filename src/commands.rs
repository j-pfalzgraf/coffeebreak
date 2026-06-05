//! Handlers for the non-timer subcommands (`stats`, `config`, `themes`,
//! `presets`). Each is small and side-effecting; the timer itself lives in
//! [`crate::app`] and the lifecycle commands in [`crate::selfcmd`].

use anyhow::{Context, Result};

use crate::cli::ConfigAction;
use crate::config::Config;
use crate::session::{self, PRESET_NAMES};
use crate::stats::Stats;
use crate::theme::{THEME_NAMES, Theme};

/// `coffeebreak stats` / `coffeebreak --stats`.
pub fn stats(theme: &Theme) {
    Stats::load_or_default().print_summary(theme);
}

/// `coffeebreak config <action>`.
pub fn config(action: &ConfigAction, theme: &Theme) -> Result<()> {
    match action {
        ConfigAction::Init => {
            let (path, created) = Config::init()?;
            if created {
                println!("{} {}", theme.bold("Created config:", theme.palette.success), path.display());
            } else {
                println!("Config already exists at {}", path.display());
            }
        }
        ConfigAction::Path => println!("{}", Config::path()?.display()),
        ConfigAction::Show => {
            let cfg = Config::load()?;
            let toml = toml::to_string_pretty(&cfg).context("failed to serialize config")?;
            print!("{toml}");
        }
    }
    Ok(())
}

/// `coffeebreak themes` — list themes with a colour swatch preview.
pub fn themes(color: bool) {
    println!("\nAvailable themes:\n");
    for name in THEME_NAMES {
        let t = Theme::resolve(name, color);
        let p = &t.palette;
        let sw = |c| t.paint("███", c);
        println!(
            "  {:<8} {}{}{}{}{}",
            name,
            sw(p.focus),
            sw(p.short_break),
            sw(p.long_break),
            sw(p.accent),
            sw(p.coffee_top),
        );
    }
    println!("\nUse one with: coffeebreak --theme <name>\n");
}

/// `coffeebreak presets` — list presets and what they configure.
pub fn presets(theme: &Theme) {
    println!("\nAvailable presets:\n");
    for name in PRESET_NAMES {
        if let Some(p) = session::preset(name) {
            let long = if p.long_enabled {
                format!(", long break {} min every {}", p.long, p.long_every)
            } else {
                String::new()
            };
            println!(
                "  {:<8} {} min focus / {} min break · {} cycle{}{}",
                theme.bold(name, theme.palette.accent),
                p.work,
                p.brk,
                p.cycles,
                if p.cycles == 1 { "" } else { "s" },
                theme.dim(long),
            );
        }
    }
    println!("\nUse one with: coffeebreak --preset <name>\n");
}
