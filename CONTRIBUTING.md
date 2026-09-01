# Contributing

Thanks for looking. I maintain this crate on my own time, so the ground
rules are short.

## Before you write code

**Open an issue first, or comment on an existing one.** Say what you
intend to change and roughly how. This costs you a few minutes and saves
us both the situation where I decline a patch you already finished.

Issues labeled *good first issue* are the easiest place to start, but
comment before you do — the label marks the size of the work, not an
open invitation. Anything already assigned to you needs no further
ceremony.

An open issue is not a claim. If a ticket has sat untouched for months,
that means I haven't gotten to it, not that it's waiting for a stranger
to close it.

## Unsolicited agent-generated pull requests

**I don't accept them.** If a coding agent wrote the patch and you are
opening the PR without having discussed the work with me first, I will
close it unreviewed.

This is not a position on the tools. It's a position on the cost. A
patch takes minutes to generate and an hour to review properly — the
dimensional analysis has to be right, the constants have to match
CODATA, and the parser has to keep holding up against hostile input.
Review is the scarce thing here, and I'd rather spend it on work someone
has actually run.

Concretely, a PR gets closed on sight if it:

- arrives against an issue you never commented on
- was written by an agent working from the issue text alone
- says the tests couldn't be run in your environment

That last one is the tell. **If you didn't compile it, don't send it.**

Use whatever tools you like on work we've agreed on. Bring the same
judgment to the output that you'd bring to your own.

## If you do send a patch

The repository has a `justfile`. `just check` runs formatting, clippy
with warnings as errors, the test suite at both feature extremes, and
rustdoc with warnings promoted. `just check-all` adds the MSRV job,
which is the fifth thing CI runs — so `check-all`, not `check`, is what
matches a green CI.

```sh
cargo install --locked just   # once
just                          # list every recipe
just check                    # the fast gate
just check-all                # the gate CI actually runs
```

`check` leaves MSRV out because it downloads a toolchain and runs the
suite a second time; that is a fine trade while you iterate and a bad
one right before you open a pull request.

`just fix` reformats in place and `just test-fast` skips the second
feature pass.

If you'd rather not install anything, the same gate by hand:

```sh
cargo fmt --all -- --check
cargo clippy --all-features --all-targets -- -D warnings
cargo test --all-features
cargo test --no-default-features
RUSTDOCFLAGS="-D warnings" cargo doc --all-features --no-deps
```

Beyond the gate: new behavior comes with a test, and the MSRV in
`Cargo.toml` still has to hold.

The CI workflow runs all of this. First-time contributors need me to
approve the run manually, so there may be a wait.

Correctness changes need a citation — CODATA, SI Brochure, IAU
resolution, or an equivalent primary source. "The other library does it
this way" isn't one.

## Reporting bugs

A wrong conversion, a bad constant, a parse that should have worked —
open an issue. Include the input, what you expected, and what you got.
These are the most useful reports I get.

Security problems go through [SECURITY.md](SECURITY.md) instead, not the
public tracker.

## License

This crate is dual licensed under the MIT license and the Apache License,
Version 2.0, at the user's option. Unless you state otherwise, anything you
submit for inclusion is dual licensed the same way, with no additional
terms. There is no CLA to sign.
