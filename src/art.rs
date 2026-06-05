//! The ASCII coffee cup whose steam tracks the remaining time.
//!
//! The rendered block is **always the same number of lines** (3 steam rows that
//! may be blank + 5 cup rows). Constant height matters: the timer feeds this to
//! a multi-line progress message and a jumping line count would corrupt the
//! redraw.

/// Total lines the rendered cup occupies (3 steam + 5 cup).
pub const CUP_LINES: usize = 8;

/// Render the cup for the given `remaining_fraction` in `0.0..=1.0`
/// (1.0 = full time left → hottest → most steam) and animation `frame`.
pub fn coffee_cup(remaining_fraction: f64, frame: usize) -> String {
    let f = remaining_fraction.clamp(0.0, 1.0);

    // Hotter coffee (more time left) steams more. Below ~5% we let it go cold.
    let steam_rows = if f > 0.75 {
        3
    } else if f > 0.40 {
        2
    } else if f > 0.05 {
        1
    } else {
        0
    };

    let mut out = String::new();

    // Three reserved rows; the lowest `steam_rows` are active. The wiggle phase
    // shifts with frame and row so the steam appears to rise.
    for row in 0..3 {
        let active = row >= 3 - steam_rows;
        if active {
            let (a, b) = if (frame + row) % 2 == 0 {
                ('(', ')')
            } else {
                (')', '(')
            };
            out.push_str(&format!("         {a}   {b}\n"));
        } else {
            out.push('\n');
        }
    }

    out.push_str("       .------.\n");
    out.push_str("      |        |]\n");
    out.push_str("      |        |\n");
    out.push_str("       \\      /\n");
    out.push_str("        `----'");

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn always_constant_height() {
        for &f in &[0.0, 0.05, 0.4, 0.75, 1.0, 1.5, -0.2] {
            for frame in 0..4 {
                let lines = coffee_cup(f, frame).lines().count();
                assert_eq!(lines, CUP_LINES, "f={f} frame={frame}");
            }
        }
    }

    #[test]
    fn steam_fades_as_time_runs_out() {
        let count_steam = |s: &str| {
            s.lines()
                .take(3)
                .filter(|l| l.contains('(') || l.contains(')'))
                .count()
        };
        assert!(count_steam(&coffee_cup(1.0, 0)) > count_steam(&coffee_cup(0.5, 0)));
        assert!(count_steam(&coffee_cup(0.5, 0)) > count_steam(&coffee_cup(0.0, 0)));
        assert_eq!(count_steam(&coffee_cup(0.0, 0)), 0);
    }
}
