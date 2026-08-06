# Security Policy

## Reporting a vulnerability

Please **don't** open a public issue for a security problem.

Use GitHub's [private vulnerability
reporting](https://github.com/iridiumdesign/iridium-units/security/advisories/new)
on this repository. If that isn't available to you, email
**brad@iridiumdesign.com**.

I maintain this crate on my own time, so I can't offer a same-day
response. I aim to acknowledge a report within a week and to have a fix
or a clear explanation within thirty days.

Please give me a reasonable window to ship a fix before publishing
details. I'll credit you in the advisory unless you'd rather I didn't.

## Supported versions

This crate is pre-1.0. Only the most recent release receives fixes —
there are no maintained back-branches.

## Scope

Most of this crate is pure computation over values you supply, with no
I/O, no network, and no filesystem access. That makes for a small attack
surface.

The exception is the parser. `parse_unit`, `parse_quantity`, and the
`FromStr` implementations accept arbitrary strings, and an application
may well feed them user-supplied input — a query parameter, a CSV
column, a config file. That is the part worth attacking.

**In scope:**

- Panics reachable from `parse_unit`, `parse_quantity`, or `FromStr` on
  attacker-controlled input. A panic in a library is a denial of service
  in the application embedding it.
- Stack exhaustion from deeply nested input. Parenthesized expressions
  are parsed recursively, so pathological nesting is a plausible vector.
- Unbounded memory or CPU consumption from an input that is small on the
  wire but expensive to parse.

**No `unsafe`.** The crate contains no `unsafe` blocks, so memory-safety
issues would have to come from a dependency rather than from this code.

## Out of scope

**Incorrect results are bugs, not vulnerabilities.** A wrong conversion,
a dimensional-analysis mistake, a constant that disagrees with CODATA,
or floating-point precision loss — please do report these, but as a
normal public issue. They're important and I want to know, they just
aren't security reports.

Arithmetic that overflows to infinity or produces `NaN` through ordinary
floating-point behavior is likewise a correctness matter rather than a
security one, unless you can show it causing a panic or a hang.
