//! Developer quotes shown when a break begins.

/// A small, hand-picked set of programming quotes. Kept short so they fit on a
/// single terminal line.
pub const QUOTES: &[&str] = &[
    "\"Premature optimization is the root of all evil.\" — Donald Knuth",
    "\"Programs must be written for people to read.\" — Harold Abelson",
    "\"Simplicity is prerequisite for reliability.\" — Edsger W. Dijkstra",
    "\"Make it work, make it right, make it fast.\" — Kent Beck",
    "\"Talk is cheap. Show me the code.\" — Linus Torvalds",
    "\"First, solve the problem. Then, write the code.\" — John Johnson",
    "\"Code is like humor. When you have to explain it, it's bad.\" — Cory House",
    "\"The best error message is the one that never shows up.\" — Thomas Fuchs",
    "\"Deleted code is debugged code.\" — Jeff Sickel",
    "\"Weeks of coding can save you hours of planning.\" — Anonymous",
    "\"There are two hard things in CS: cache invalidation and naming things.\" — Phil Karlton",
    "\"Any fool can write code a computer understands. Good programmers write code humans understand.\" — Martin Fowler",
    "\"Walking on water and developing software from a spec are easy if both are frozen.\" — Edward Berard",
    "\"It's not a bug — it's an undocumented feature.\" — Anonymous",
    "\"Rest your eyes. The compiler will still be there in five minutes.\" — coffeebreak",
];

/// A pseudo-random quote. Avoids returning the same quote as `previous`
/// (when given) so consecutive breaks don't repeat.
pub fn random_quote(previous: Option<&str>) -> &'static str {
    debug_assert!(!QUOTES.is_empty());
    if QUOTES.len() == 1 {
        return QUOTES[0];
    }
    loop {
        let idx = rand::random_range(0..QUOTES.len());
        let pick = QUOTES[idx];
        if Some(pick) != previous {
            return pick;
        }
    }
}
