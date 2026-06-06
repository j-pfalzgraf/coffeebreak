<!-- Thanks for contributing to coffeebreak! -->

## What & why

<!-- A short description of the change and the motivation. Link any issue: "Closes #123". -->

## Checklist

- [ ] `cargo fmt --all` (formatting is CI-enforced)
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` is clean
- [ ] `cargo test` passes
- [ ] Docs updated (README / CHANGELOG / `--help`) if user-facing behaviour changed
- [ ] New user-facing strings go through `i18n` (English in `Msg::en`, with translations or English fallback)
