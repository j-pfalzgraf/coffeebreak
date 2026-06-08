//! Colour theming — the single source of truth for every colour in coffeebreak.
//!
//! Centralising colour here keeps the rest of the codebase DRY: widgets and the
//! stats view ask the [`Theme`] for a colour or a styled string instead of
//! hard-coding ANSI. Themes are truecolour (24-bit RGB) and degrade to plain
//! text when colour is disabled (`--no-color`, a non-tty, or `NO_COLOR`).

use std::fmt;

use crossterm::style::{Color, Stylize};

/// A 24-bit RGB colour. Small, `Copy`, and easy to interpolate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl Rgb {
    /// Linear interpolation between two colours; `t` is clamped to `0.0..=1.0`.
    pub fn lerp(self, other: Rgb, t: f64) -> Rgb {
        let t = t.clamp(0.0, 1.0);
        let mix = |a: u8, b: u8| (f64::from(a) + (f64::from(b) - f64::from(a)) * t).round() as u8;
        Rgb(
            mix(self.0, other.0),
            mix(self.1, other.1),
            mix(self.2, other.2),
        )
    }

    /// Scale brightness by `factor` (e.g. `0.6` to dim), clamped per channel.
    pub fn shade(self, factor: f64) -> Rgb {
        let s = |c: u8| (f64::from(c) * factor).clamp(0.0, 255.0).round() as u8;
        Rgb(s(self.0), s(self.1), s(self.2))
    }

    /// Convert to a crossterm colour.
    pub fn into_color(self) -> Color {
        Color::Rgb {
            r: self.0,
            g: self.1,
            b: self.2,
        }
    }
}

/// The full palette a theme provides. Every visible colour comes from here.
#[derive(Debug, Clone, Copy)]
pub struct Palette {
    pub focus: Rgb,
    pub short_break: Rgb,
    pub long_break: Rgb,
    pub accent: Rgb,
    pub text: Rgb,
    pub muted: Rgb,
    pub cup: Rgb,
    pub coffee_top: Rgb,
    pub coffee_bottom: Rgb,
    pub steam: Rgb,
    pub bar_start: Rgb,
    pub bar_end: Rgb,
    pub success: Rgb,
    pub warn: Rgb,
}

/// A named palette plus a colour-enabled switch.
#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub name: &'static str,
    pub palette: Palette,
    enabled: bool,
}

/// All built-in theme names, in display order.
pub const THEME_NAMES: &[&str] = &["coffee", "ocean", "forest", "grape", "mono"];

/// Theme names accepted on the CLI — the built-ins plus the config-defined
/// `custom` palette.
pub const THEME_CHOICES: &[&str] = &["coffee", "ocean", "forest", "grape", "mono", "custom"];

/// The default theme name.
pub const DEFAULT_THEME: &str = "coffee";

/// The palette-field names a user may override in a `[custom_theme]` config.
pub const PALETTE_KEYS: &[&str] = &[
    "focus",
    "short_break",
    "long_break",
    "accent",
    "text",
    "muted",
    "cup",
    "coffee_top",
    "coffee_bottom",
    "steam",
    "bar_start",
    "bar_end",
    "success",
    "warn",
];

impl Theme {
    /// Resolve a theme by name (case-insensitive), falling back to the default
    /// for an unknown name. `enabled` toggles all colour output.
    pub fn resolve(name: &str, enabled: bool) -> Theme {
        let (name, palette) = match name.to_ascii_lowercase().as_str() {
            "ocean" => ("ocean", OCEAN),
            "forest" => ("forest", FOREST),
            "grape" => ("grape", GRAPE),
            "mono" => ("mono", MONO),
            _ => ("coffee", COFFEE),
        };
        Theme {
            name,
            palette,
            enabled,
        }
    }

    /// Resolve a theme, supporting the config-defined `custom` palette.
    ///
    /// When `name` is `"custom"` and `custom` is `Some`, that palette is used;
    /// otherwise this behaves like [`Theme::resolve`].
    pub fn build(name: &str, enabled: bool, custom: Option<Palette>) -> Theme {
        if name.eq_ignore_ascii_case("custom")
            && let Some(palette) = custom
        {
            return Theme {
                name: "custom",
                palette,
                enabled,
            };
        }
        Theme::resolve(name, enabled)
    }

    /// Whether colour output is on.
    pub fn color(&self) -> bool {
        self.enabled
    }

    /// Return a copy with colour forced on or off.
    pub fn with_color(mut self, on: bool) -> Theme {
        self.enabled = on;
        self
    }

    /// The accent colour for a phase.
    pub fn phase_color(&self, phase: crate::Phase) -> Rgb {
        match phase {
            crate::Phase::Focus => self.palette.focus,
            crate::Phase::ShortBreak => self.palette.short_break,
            crate::Phase::LongBreak => self.palette.long_break,
        }
    }

    // --- Inline styling helpers (produce ANSI strings, or plain when off) ----

    /// Paint `text` in `color` (foreground). Plain text when colour is off.
    pub fn paint(&self, text: impl fmt::Display, color: Rgb) -> String {
        if self.enabled {
            format!("{}", format!("{text}").with(color.into_color()))
        } else {
            format!("{text}")
        }
    }

    /// Bold `text` (and optionally coloured). Plain when colour is off.
    pub fn bold(&self, text: impl fmt::Display, color: Rgb) -> String {
        if self.enabled {
            format!("{}", format!("{text}").with(color.into_color()).bold())
        } else {
            format!("{text}")
        }
    }

    /// Dim/muted `text`.
    pub fn dim(&self, text: impl fmt::Display) -> String {
        if self.enabled {
            format!(
                "{}",
                format!("{text}").with(self.palette.muted.into_color())
            )
        } else {
            format!("{text}")
        }
    }
}

/// Parse a `#RRGGBB` or `RRGGBB` hex colour. Returns `None` if malformed.
pub fn parse_hex(s: &str) -> Option<Rgb> {
    let h = s.trim().strip_prefix('#').unwrap_or(s.trim());
    if h.len() != 6 || !h.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    let byte = |i: usize| u8::from_str_radix(&h[i..i + 2], 16).ok();
    Some(Rgb(byte(0)?, byte(2)?, byte(4)?))
}

impl Palette {
    /// Return a copy with the given fields overridden from a map of
    /// `palette-field-name -> hex`. Unknown keys and malformed values are
    /// ignored, so a partial or slightly wrong custom theme still works.
    pub fn with_overrides<'a, I>(mut self, overrides: I) -> Palette
    where
        I: IntoIterator<Item = (&'a str, &'a str)>,
    {
        for (key, value) in overrides {
            let Some(rgb) = parse_hex(value) else {
                continue;
            };
            match key {
                "focus" => self.focus = rgb,
                "short_break" => self.short_break = rgb,
                "long_break" => self.long_break = rgb,
                "accent" => self.accent = rgb,
                "text" => self.text = rgb,
                "muted" => self.muted = rgb,
                "cup" => self.cup = rgb,
                "coffee_top" => self.coffee_top = rgb,
                "coffee_bottom" => self.coffee_bottom = rgb,
                "steam" => self.steam = rgb,
                "bar_start" => self.bar_start = rgb,
                "bar_end" => self.bar_end = rgb,
                "success" => self.success = rgb,
                "warn" => self.warn = rgb,
                _ => {}
            }
        }
        self
    }
}

/// Build a custom palette from hex overrides, starting from the `coffee` base
/// (so any field the user omits keeps a sensible default).
pub fn custom_palette<'a, I>(overrides: I) -> Palette
where
    I: IntoIterator<Item = (&'a str, &'a str)>,
{
    COFFEE.with_overrides(overrides)
}

// ---------------------------------------------------------------------------
// Built-in palettes
// ---------------------------------------------------------------------------

const COFFEE: Palette = Palette {
    focus: Rgb(0xE6, 0x7E, 0x22),         // warm amber
    short_break: Rgb(0x3F, 0xB9, 0x50),   // green
    long_break: Rgb(0x9B, 0x59, 0xB6),    // purple
    accent: Rgb(0xF1, 0xC4, 0x0F),        // gold
    text: Rgb(0xEC, 0xE3, 0xD4),          // cream
    muted: Rgb(0x8A, 0x7E, 0x6E),         // taupe
    cup: Rgb(0xD8, 0xCB, 0xB8),           // porcelain
    coffee_top: Rgb(0x8B, 0x5A, 0x2B),    // crema
    coffee_bottom: Rgb(0x3E, 0x26, 0x12), // espresso
    steam: Rgb(0xCF, 0xCF, 0xCF),
    bar_start: Rgb(0xE6, 0x7E, 0x22),
    bar_end: Rgb(0xF1, 0xC4, 0x0F),
    success: Rgb(0x2E, 0xCC, 0x71),
    warn: Rgb(0xE7, 0x4C, 0x3C),
};

const OCEAN: Palette = Palette {
    focus: Rgb(0x1A, 0xBC, 0x9C),
    short_break: Rgb(0x3D, 0x9B, 0xE9),
    long_break: Rgb(0x6C, 0x5C, 0xE7),
    accent: Rgb(0x48, 0xDB, 0xFB),
    text: Rgb(0xDF, 0xEE, 0xF5),
    muted: Rgb(0x6B, 0x84, 0x90),
    cup: Rgb(0xCB, 0xDD, 0xE6),
    coffee_top: Rgb(0x12, 0x7A, 0x9C),
    coffee_bottom: Rgb(0x0A, 0x2A, 0x43),
    steam: Rgb(0xC6, 0xE6, 0xF0),
    bar_start: Rgb(0x1A, 0xBC, 0x9C),
    bar_end: Rgb(0x48, 0xDB, 0xFB),
    success: Rgb(0x1A, 0xBC, 0x9C),
    warn: Rgb(0xE7, 0x4C, 0x3C),
};

const FOREST: Palette = Palette {
    focus: Rgb(0x2E, 0xCC, 0x71),
    short_break: Rgb(0x7B, 0xC0, 0x43),
    long_break: Rgb(0xD3, 0x9E, 0x00),
    accent: Rgb(0xA3, 0xE0, 0x48),
    text: Rgb(0xE4, 0xEE, 0xDA),
    muted: Rgb(0x76, 0x84, 0x6A),
    cup: Rgb(0xD3, 0xDD, 0xC6),
    coffee_top: Rgb(0x55, 0x6B, 0x2F),
    coffee_bottom: Rgb(0x22, 0x30, 0x16),
    steam: Rgb(0xD8, 0xE8, 0xCC),
    bar_start: Rgb(0x2E, 0xCC, 0x71),
    bar_end: Rgb(0xA3, 0xE0, 0x48),
    success: Rgb(0x2E, 0xCC, 0x71),
    warn: Rgb(0xE6, 0x7E, 0x22),
};

const GRAPE: Palette = Palette {
    focus: Rgb(0x9B, 0x59, 0xB6),
    short_break: Rgb(0xE0, 0x4F, 0x9B),
    long_break: Rgb(0x34, 0x98, 0xDB),
    accent: Rgb(0xF8, 0x6F, 0xC9),
    text: Rgb(0xF0, 0xE6, 0xF5),
    muted: Rgb(0x84, 0x6E, 0x8C),
    cup: Rgb(0xE2, 0xD4, 0xE8),
    coffee_top: Rgb(0x6A, 0x2C, 0x82),
    coffee_bottom: Rgb(0x2A, 0x12, 0x33),
    steam: Rgb(0xEC, 0xD9, 0xF0),
    bar_start: Rgb(0x9B, 0x59, 0xB6),
    bar_end: Rgb(0xF8, 0x6F, 0xC9),
    success: Rgb(0x2E, 0xCC, 0x71),
    warn: Rgb(0xE7, 0x4C, 0x3C),
};

const MONO: Palette = Palette {
    focus: Rgb(0xE0, 0xE0, 0xE0),
    short_break: Rgb(0xB0, 0xB0, 0xB0),
    long_break: Rgb(0xFF, 0xFF, 0xFF),
    accent: Rgb(0xFF, 0xFF, 0xFF),
    text: Rgb(0xDA, 0xDA, 0xDA),
    muted: Rgb(0x80, 0x80, 0x80),
    cup: Rgb(0xC8, 0xC8, 0xC8),
    coffee_top: Rgb(0x90, 0x90, 0x90),
    coffee_bottom: Rgb(0x40, 0x40, 0x40),
    steam: Rgb(0xC0, 0xC0, 0xC0),
    bar_start: Rgb(0x9A, 0x9A, 0x9A),
    bar_end: Rgb(0xF0, 0xF0, 0xF0),
    success: Rgb(0xFF, 0xFF, 0xFF),
    warn: Rgb(0xBB, 0xBB, 0xBB),
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lerp_endpoints_and_midpoint() {
        let a = Rgb(0, 0, 0);
        let b = Rgb(100, 200, 50);
        assert_eq!(a.lerp(b, 0.0), a);
        assert_eq!(a.lerp(b, 1.0), b);
        assert_eq!(a.lerp(b, 0.5), Rgb(50, 100, 25));
        // clamps out-of-range t
        assert_eq!(a.lerp(b, 2.0), b);
        assert_eq!(a.lerp(b, -1.0), a);
    }

    #[test]
    fn resolve_is_case_insensitive_with_fallback() {
        assert_eq!(Theme::resolve("OCEAN", true).name, "ocean");
        assert_eq!(Theme::resolve("nope", true).name, "coffee");
    }

    #[test]
    fn paint_is_plain_when_disabled() {
        let t = Theme::resolve("coffee", false);
        assert_eq!(t.paint("hi", Rgb(1, 2, 3)), "hi");
        assert_eq!(t.dim("hi"), "hi");
    }

    #[test]
    fn parse_hex_handles_valid_and_invalid() {
        assert_eq!(parse_hex("#FF8800"), Some(Rgb(0xFF, 0x88, 0x00)));
        assert_eq!(parse_hex("00ff00"), Some(Rgb(0, 255, 0)));
        assert_eq!(parse_hex("  #1a2b3c "), Some(Rgb(0x1A, 0x2B, 0x3C)));
        assert_eq!(parse_hex("xyz"), None);
        assert_eq!(parse_hex("#12345"), None); // wrong length
        assert_eq!(parse_hex("#gg0000"), None); // non-hex
    }

    #[test]
    fn custom_palette_overrides_only_named_fields() {
        let p = custom_palette([
            ("focus", "#010203"),
            ("warn", "0a0b0c"),
            ("bogus", "#ffffff"),
        ]);
        assert_eq!(p.focus, Rgb(1, 2, 3));
        assert_eq!(p.warn, Rgb(0x0A, 0x0B, 0x0C));
        // Unset fields keep the coffee base; malformed/unknown keys are ignored.
        assert_eq!(p.accent, COFFEE.accent);
    }

    #[test]
    fn build_uses_custom_palette_when_named() {
        let custom = custom_palette([("focus", "#112233")]);
        let t = Theme::build("custom", true, Some(custom));
        assert_eq!(t.name, "custom");
        assert_eq!(t.palette.focus, Rgb(0x11, 0x22, 0x33));
        // Without a custom palette, "custom" falls back to the default.
        assert_eq!(Theme::build("custom", true, None).name, "coffee");
        // A built-in name ignores any provided custom palette.
        assert_eq!(Theme::build("ocean", true, Some(custom)).name, "ocean");
    }
}
