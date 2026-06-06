//! Developer quotes shown when a break begins.

/// A small, hand-picked set of programming quotes. Kept short so they fit on a
/// single terminal line.
pub const QUOTES: &[&str] = &[
    "\"Premature optimization is the root of all evil.\" — Donald Knuth",
    "\"Beware of bugs in the above code; I have only proved it correct, not tried it.\" — Donald Knuth",
    "\"Programs must be written for people to read, and only incidentally for machines to execute.\" — Harold Abelson",
    "\"Simplicity is prerequisite for reliability.\" — Edsger W. Dijkstra",
    "\"If debugging is the process of removing software bugs, then programming must be the process of putting them in.\" — Edsger W. Dijkstra",
    "\"Make it work, make it right, make it fast.\" — Kent Beck",
    "\"Talk is cheap. Show me the code.\" — Linus Torvalds",
    "\"Given enough eyeballs, all bugs are shallow.\" — Linus Torvalds",
    "\"First, solve the problem. Then, write the code.\" — John Johnson",
    "\"Code is like humor. When you have to explain it, it's bad.\" — Cory House",
    "\"The best error message is the one that never shows up.\" — Thomas Fuchs",
    "\"Deleted code is debugged code.\" — Jeff Sickel",
    "\"There are two hard things in computer science: cache invalidation and naming things.\" — Phil Karlton",
    "\"Any fool can write code that a computer can understand. Good programmers write code that humans can understand.\" — Martin Fowler",
    "\"Walking on water and developing software from a spec are easy if both are frozen.\" — Edward Berard",
    "\"The best way to predict the future is to invent it.\" — Alan Kay",
    "\"Simple things should be simple, complex things should be possible.\" — Alan Kay",
    "\"The most damaging phrase in the language is: it's always been done that way.\" — Grace Hopper",
    "\"A ship in port is safe, but that's not what ships are built for.\" — Grace Hopper",
    "\"Focus is a matter of deciding what things you're not going to do.\" — John Carmack",
    "\"Low-level programming is good for the programmer's soul.\" — John Carmack",
    "\"Controlling complexity is the essence of computer programming.\" — Brian Kernighan",
    "\"Debugging is twice as hard as writing the code in the first place.\" — Brian Kernighan",
    "\"The function of good software is to make the complex appear simple.\" — Grady Booch",
    "\"Programming isn't about what you know; it's about what you can figure out.\" — Chris Pine",
    "\"The most important property of a program is whether it accomplishes the intention of its user.\" — C.A.R. Hoare",
    "\"There are two ways of constructing a software design: make it so simple there are obviously no deficiencies, or so complicated there are no obvious deficiencies.\" — C.A.R. Hoare",
    "\"Software is a great combination between artistry and engineering.\" — Bill Gates",
    "\"Measuring programming progress by lines of code is like measuring aircraft building progress by weight.\" — Bill Gates",
    "\"Quality is not an act, it is a habit.\" — Aristotle",
    "\"It always seems impossible until it's done.\" — Nelson Mandela",
    "\"The only way to go fast is to go well.\" — Robert C. Martin",
    "\"Truth can only be found in one place: the code.\" — Robert C. Martin",
    "\"Programs are meant to be read by humans and only incidentally for computers to execute.\" — Donald Knuth",
    "\"Computers are good at following instructions, but not at reading your mind.\" — Donald Knuth",
    "\"Testing leads to failure, and failure leads to understanding.\" — Burt Rutan",
    "\"Make the change easy, then make the easy change.\" — Kent Beck",
    "\"Premature abstraction is as dangerous as premature optimization.\" — Anonymous",
    "\"Weeks of coding can save you hours of planning.\" — Anonymous",
    "\"Rest your eyes. The compiler will still be there in five minutes.\" — coffeebreak",
    "\"Step away from the keyboard; the best ideas arrive when your hands are around a warm mug.\" — coffeebreak",
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn has_a_healthy_collection() {
        assert!(
            QUOTES.len() >= 30,
            "expected at least 30 quotes, got {}",
            QUOTES.len()
        );
    }

    #[test]
    fn no_duplicate_quotes() {
        let mut seen = HashSet::new();
        for quote in QUOTES {
            assert!(seen.insert(*quote), "duplicate quote found: {quote}");
        }
    }

    #[test]
    fn quotes_are_attributed_and_single_line() {
        for quote in QUOTES {
            assert!(!quote.contains('\n'), "quote spans multiple lines: {quote}");
            assert!(
                quote.contains(" — "),
                "quote is missing an attribution: {quote}"
            );
        }
    }

    #[test]
    fn random_quote_avoids_previous() {
        // Pick an arbitrary quote to avoid, then draw many times and ensure we
        // never return it.
        let previous = QUOTES[0];
        for _ in 0..1000 {
            assert_ne!(random_quote(Some(previous)), previous);
        }
    }

    #[test]
    fn random_quote_returns_a_known_quote() {
        for _ in 0..100 {
            let pick = random_quote(None);
            assert!(QUOTES.contains(&pick));
        }
    }
}
