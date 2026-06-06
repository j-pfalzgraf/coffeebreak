//! Handlers for the non-timer subcommands (`stats`, `config`, `themes`,
//! `presets`, `languages`). Each is small and side-effecting; the timer lives in
//! [`crate::app`] and the lifecycle commands in [`crate::selfcmd`]. All output is
//! localised via [`I18n`].

use anyhow::{Context, Result};

use crate::cli::ConfigAction;
use crate::config::Config;
use crate::i18n::{I18n, LANGUAGES, Msg, Noun};
use crate::session::{self, PRESET_NAMES};
use crate::stats::Stats;
use crate::theme::{THEME_NAMES, Theme};

/// `coffeebreak stats` / `coffeebreak --stats`.
pub fn stats(theme: &Theme, i18n: &I18n) {
    Stats::load_or_default(i18n).print_summary(theme, i18n);
}

/// `coffeebreak config <action>`.
pub fn config(action: &ConfigAction, theme: &Theme, i18n: &I18n) -> Result<()> {
    match action {
        ConfigAction::Init => {
            let (path, created) = Config::init()?;
            if created {
                println!(
                    "{} {}",
                    theme.bold(i18n.t(Msg::ConfigCreated), theme.palette.success),
                    path.display()
                );
            } else {
                println!("{} {}", i18n.t(Msg::ConfigExists), path.display());
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
pub fn themes(theme: &Theme, i18n: &I18n) {
    println!("\n{}\n", i18n.t(Msg::ThemesTitle));
    for name in THEME_NAMES {
        let t = Theme::resolve(name, theme.color());
        let p = &t.palette;
        let sw = |c| t.paint("███", c);
        println!(
            "  {name:<8} {}{}{}{}{}",
            sw(p.focus),
            sw(p.short_break),
            sw(p.long_break),
            sw(p.accent),
            sw(p.coffee_top),
        );
    }
    println!("\n{}\n", i18n.t(Msg::ThemesHint));
}

/// `coffeebreak presets` — list presets and what they configure.
pub fn presets(theme: &Theme, i18n: &I18n) {
    println!("\n{}\n", i18n.t(Msg::PresetsTitle));
    for name in PRESET_NAMES {
        if let Some(p) = session::preset(name) {
            let mut cadence = i18n.tf(
                Msg::PresetCadence,
                &[
                    ("work", &p.work.to_string()),
                    ("brk", &p.brk.to_string()),
                    ("count", &i18n.count(p.cycles, Noun::Cycle)),
                ],
            );
            if p.long_enabled {
                cadence.push_str(&i18n.tf(
                    Msg::PresetLong,
                    &[("long", &p.long.to_string()), ("every", &p.long_every.to_string())],
                ));
            }
            println!("  {} {}", theme.bold(name, theme.palette.accent), theme.dim(cadence));
        }
    }
    println!("\n{}\n", i18n.t(Msg::PresetsHint));
}

/// `coffeebreak languages` — list interface languages, marking the active one.
pub fn languages(theme: &Theme, i18n: &I18n) {
    println!("\n{}\n", i18n.t(Msg::LanguagesTitle));
    for (code, native) in LANGUAGES {
        let active = *code == i18n.code();
        let marker = if active { "●" } else { " " };
        let line = format!("{marker} {code:<4} {native}");
        if active {
            println!("  {}", theme.bold(&line, theme.palette.accent));
        } else {
            println!("  {line}");
        }
    }
    println!("\n{}\n", i18n.t(Msg::LanguagesHint));
}
