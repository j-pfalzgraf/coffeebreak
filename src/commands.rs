//! Handlers for the non-timer subcommands (`stats`, `config`, `themes`,
//! `presets`, `languages`). Each is small and side-effecting; the timer lives in
//! [`crate::app`] and the lifecycle commands in [`crate::selfcmd`]. All output is
//! localised via [`I18n`].

use std::io::IsTerminal;

use anyhow::{Context, Result};

use crate::cli::{ConfigAction, StatsFormat};
use crate::config::Config;
use crate::i18n::{I18n, LANGUAGES, Msg, Noun};
use crate::session::{self, PRESET_NAMES};
use crate::stats::Stats;
use crate::theme::{THEME_NAMES, Theme};

/// `coffeebreak stats` / `coffeebreak --stats`.
///
/// `text` renders the animated dashboard; `json`/`csv` print machine-readable
/// output (no colour, no animation) suitable for piping.
pub fn stats(theme: &Theme, i18n: &I18n, goal: u64, format: StatsFormat) {
    let stats = Stats::load_or_default(i18n);
    match format {
        StatsFormat::Text => stats.print_summary(theme, i18n, goal),
        StatsFormat::Json => println!("{}", stats.to_json()),
        StatsFormat::Csv => print!("{}", stats.to_csv()),
    }
}

/// `coffeebreak doctor` — print localised environment diagnostics.
pub fn doctor(theme: &Theme, i18n: &I18n) {
    let p = &theme.palette;
    println!("\n{}\n", theme.bold(i18n.t(Msg::DoctorTitle), p.accent));

    // Gather all rows first so the label column can be sized to the widest
    // (localised) label rather than a fixed width that long words overflow.
    let mut rows: Vec<(Msg, bool, String)> = Vec::new();

    let tty = std::io::stdout().is_terminal() && std::io::stdin().is_terminal();
    rows.push((
        Msg::DoctorTerminal,
        tty,
        i18n.t(if tty {
            Msg::DoctorTtyYes
        } else {
            Msg::DoctorTtyNo
        })
        .to_string(),
    ));

    let truecolor = std::env::var("COLORTERM")
        .map(|v| v.contains("truecolor") || v.contains("24bit"))
        .unwrap_or(false);
    rows.push((
        Msg::DoctorColor,
        truecolor,
        i18n.t(if truecolor {
            Msg::DoctorColorYes
        } else {
            Msg::DoctorColorNo
        })
        .to_string(),
    ));

    rows.push((
        Msg::DoctorLang,
        true,
        format!("{} ({})", i18n.code(), i18n.name()),
    ));

    let cfg_path = Config::path().ok();
    let cfg_exists = cfg_path.as_ref().map(|p| p.exists()).unwrap_or(false);
    rows.push((
        Msg::DoctorConfig,
        cfg_exists,
        format!(
            "{} {}",
            i18n.t(if cfg_exists {
                Msg::DoctorConfigExists
            } else {
                Msg::DoctorConfigMissing
            }),
            cfg_path
                .map(|p| theme.dim(p.display().to_string()))
                .unwrap_or_default(),
        ),
    ));

    let data_dir = crate::paths::data_dir().ok();
    let writable = data_dir
        .as_ref()
        .map(|d| dir_is_writable(d))
        .unwrap_or(false);
    rows.push((
        Msg::DoctorData,
        writable,
        format!(
            "{} {}",
            i18n.t(if writable {
                Msg::DoctorDataOk
            } else {
                Msg::DoctorDataNo
            }),
            data_dir
                .map(|d| theme.dim(d.display().to_string()))
                .unwrap_or_default(),
        ),
    ));

    let notify_ok = notifications_available();
    rows.push((
        Msg::DoctorNotify,
        notify_ok,
        i18n.t(if notify_ok {
            Msg::DoctorNotifyYes
        } else {
            Msg::DoctorNotifyNo
        })
        .to_string(),
    ));

    let chime = cfg!(feature = "sound");
    rows.push((
        Msg::DoctorSound,
        true,
        i18n.t(if chime {
            Msg::DoctorSoundChime
        } else {
            Msg::DoctorSoundBell
        })
        .to_string(),
    ));

    let label_w = rows
        .iter()
        .map(|(m, _, _)| i18n.t(*m).chars().count())
        .max()
        .unwrap_or(16);
    for (label, ok, detail) in rows {
        let (glyph, color) = if ok {
            ("✓", p.success)
        } else {
            ("!", p.warn)
        };
        let padded = format!("{:<label_w$}", i18n.t(label));
        println!(
            "  {} {} {}",
            theme.bold(padded, p.accent),
            theme.paint(glyph, color),
            detail
        );
    }
    println!();
}

/// Whether `dir` can actually be written to: ensures it exists, then probes with
/// a temporary file (so a pre-existing read-only directory reports correctly).
fn dir_is_writable(dir: &std::path::Path) -> bool {
    if std::fs::create_dir_all(dir).is_err() {
        return false;
    }
    let probe = dir.join(".coffeebreak-doctor-probe");
    let ok = std::fs::write(&probe, b"").is_ok();
    let _ = std::fs::remove_file(&probe);
    ok
}

/// Best-effort check for a desktop notification service.
fn notifications_available() -> bool {
    if cfg!(target_os = "linux") {
        std::env::var_os("DBUS_SESSION_BUS_ADDRESS").is_some()
    } else {
        // macOS and Windows provide a system notification centre.
        cfg!(any(target_os = "macos", target_os = "windows"))
    }
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
                    &[
                        ("long", &p.long.to_string()),
                        ("every", &p.long_every.to_string()),
                    ],
                ));
            }
            println!(
                "  {} {}",
                theme.bold(name, theme.palette.accent),
                theme.dim(cadence)
            );
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
