# iridium-units — the build.
#
# Everything here mirrors .github/workflows/rust.yml. If a recipe in the
# `gates` group passes locally, CI passes too; if the two ever disagree,
# the workflow is the authority and this file is the bug.

# The floor in Cargo.toml. Named once here so `just msrv` and the
# rust-version field cannot drift apart silently.
MSRV := "1.80"

# The book lives beside the crate, not inside it. See BOOK below.
BOOK := "../iridium-units-book"

# List the recipes. Hidden: `just` runs it, nobody types it.
[private]
default:
    @just --list --unsorted

# ── build ──────────────────────────────────────────────────────────

[doc('Build the library with every feature on.')]
[group('build')]
build:
    cargo build --all-features

[doc('Build the rustdoc and open it.')]
[group('build')]
doc:
    cargo doc --all-features --no-deps --open

# The book carries the crate version in its colophon and its Appendix A
# tables are generated from the crate, so it is not optional reading
# when behavior changes — the house rule is that library work updates
# the book in the same sitting. This builds it in place.

[doc('Build The Measure, the reference manual, in its own repo.')]
[group('build')]
book:
    #!/usr/bin/env sh
    if [ ! -d "{{ BOOK }}" ]; then
        echo "  no book at {{ BOOK }}" >&2
        echo "  clone iridium-units-book beside this repo." >&2
        exit 1
    fi
    cd "{{ BOOK }}" && make

# ── inspect ────────────────────────────────────────────────────────

[doc('Run the criterion benchmarks.')]
[group('inspect')]
bench:
    cargo bench

[doc('What ships in the published crate.')]
[group('inspect')]
manifest:
    cargo package --list --allow-dirty

# The crate version and the book's colophon are supposed to agree. This
# prints both rather than asserting, because between releases the book
# tracks crate main on purpose and a gate that fails on that would get
# switched off.

[doc('Crate version beside the version the book claims.')]
[group('inspect')]
version:
    #!/usr/bin/env sh
    crate=$(grep -m1 '^version' Cargo.toml | cut -d'"' -f2)
    printf '  %-14s %s\n' "crate" "$crate"
    if [ -f "{{ BOOK }}/iridium-units.typ" ]; then
        book=$(grep -m1 'crate-version =' "{{ BOOK }}/iridium-units.typ" \
               | cut -d'"' -f2)
        printf '  %-14s %s\n' "book colophon" "$book"
        [ "$crate" = "$book" ] || echo "  (differ — expected between releases)"
    else
        printf '  %-14s %s\n' "book colophon" "not found"
    fi

# ── gates ──────────────────────────────────────────────────────────

[doc('Formatting.')]
[group('gates')]
fmt:
    cargo fmt --all -- --check

[doc('Reformat in place.')]
[group('gates')]
fix:
    cargo fmt --all

[doc('Lints. Warnings are errors, same as CI.')]
[group('gates')]
clippy:
    cargo clippy --all-features --all-targets -- -D warnings

# CI runs the suite twice: every feature on, and every feature off. The
# second is the one that catches a use path or a doc link that only
# resolves because `astrophysics` happened to be enabled.

[doc('Tests, both feature extremes — same as CI.')]
[group('gates')]
test:
    cargo test --all-features
    cargo test --no-default-features

[doc('Tests with every feature on only. Quicker while iterating.')]
[group('gates')]
test-fast:
    cargo test --all-features

# A broken intra-doc link is a warning by default, so docs.rs renders it
# and nobody notices. CI promotes it, and so does this.

[doc('Rustdoc with warnings promoted to errors.')]
[group('gates')]
doc-check:
    RUSTDOCFLAGS="-D warnings" cargo doc --all-features --no-deps

# Builds rather than tests, and the distinction matters. `rust-version`
# is a promise to consumers, who pull only thiserror. The dev-dependency
# tree declares higher floors -- proptest 1.82, half 1.81 via criterion
# -- so `cargo test` refuses on the floor even though the code compiles
# and passes there. Testing the dev tree would gate the promise on
# packages no consumer ever sees.

[doc('Build every feature combination on the MSRV floor.')]
[group('gates')]
msrv:
    rustup toolchain install {{ MSRV }} --profile minimal
    cargo +{{ MSRV }} build --all-features
    cargo +{{ MSRV }} build --no-default-features

# `check` is the fast gate and deliberately leaves out `msrv`, which
# downloads a toolchain and runs the suite again. CI does run it, so
# `check` passing is not by itself proof that CI will pass — use
# `check-all` before opening a pull request.

[doc('The fast gate: fmt, clippy, tests, docs. No MSRV.')]
[group('gates')]
check: fmt clippy test doc-check

[doc('Everything CI runs, MSRV included.')]
[group('gates')]
check-all: check msrv

# ── ship ───────────────────────────────────────────────────────────

# Nothing here publishes. `cargo publish` is a one-way door and stays a
# thing you type yourself, on purpose.

[doc('Dry-run the release packaging.')]
[group('ship')]
package:
    cargo package --all-features

[doc('Preflight a release: full gate, package, and show both versions.')]
[group('ship')]
release-check: check-all package version

# ── tidy ───────────────────────────────────────────────────────────

[doc('Remove build artifacts.')]
[group('tidy')]
clean:
    cargo clean
