# iridium-units

A high-performance runtime unit-of-measure library for Rust.

[![Crates.io](https://img.shields.io/crates/v/iridium-units.svg)](https://crates.io/crates/iridium-units)
[![Documentation](https://docs.rs/iridium-units/badge.svg)](https://docs.rs/iridium-units)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- **Runtime dimensional analysis** - Catch unit errors at runtime with helpful error messages
- **High performance** - Batch conversion API (~80x faster), optimized operators
- **Flexible parsing** - Unicode (`m²`, `µm`, `Ω`), LaTeX (`m^{2}`), natural language (`km per hour`)
- **Comprehensive unit systems** - SI, CGS, astrophysical, imperial, logarithmic
- **12 equivalencies** - Spectral, Doppler, parallax, mass-energy, temperature, and more
- **Exact rational exponents** - No floating-point errors in dimensional analysis

## Quick Start

```rust
use iridium_units::prelude::*;

// Create quantities
let distance = 100.0 * &*KM;
let time = 2.0 * &*H;
let speed = &distance / &time;

// Convert units
let speed_ms = speed.to(&(&*M / &*S))?;
println!("{}", speed_ms);  // 13.888... m/s

// Parse from strings
let wavelength = parse_quantity("500 nm")?;
let frequency = wavelength.to_equiv(&HZ, spectral())?;
```

See the [documentation](docs/getting-started.md) for more examples.

## The Story So Far

I built this library because I needed a units system in a real production
environment—and I’m not a mathematician, a physicist, or someone who spends
their days thinking deeply about dimensional analysis. I’m a software engineer
who just wanted code involving units to be *correct*, *safe*, and *fast*,
without constantly second-guessing myself.

I’ve used AstroPy’s units library before, and it’s genuinely excellent. It set
a very high bar for what a unit system can look like in terms of clarity and
expressiveness. Unfortunately, when I tried to use it in a production setting
with real runtime constraints, the performance just wasn’t where I needed it to
be.

So instead of trying to force an existing solution to fit, I decided to learn
from it.

With the help of AI, I analyzed AstroPy and a number of other unit-of-measure
libraries—looking at their APIs, internal models, and performance
characteristics. The result is **iridium-units**: a runtime dimensional
analysis library inspired by AstroPy’s ergonomics, but designed from the ground
up with production performance in mind.

This project exists because I wanted something that felt *safe* without feeling
*fragile*, and *correct* without being *slow*.

## Motivation

Unit errors are one of those problems that feel obvious in hindsight and
painful in production. They’re easy to introduce, hard to spot in review, and
often invisible until the worst possible moment.

In my experience, most unit libraries fall into one of two camps:

- **Compile-time unit systems**, which are powerful but rigid,
language-specific, and often overkill for day-to-day work
- **Runtime unit systems**, which are flexible and expressive, but frequently
too slow for high-throughput or latency-sensitive code

iridium-units is my attempt to sit comfortably between those two extremes: a
runtime unit system that’s fast enough for production use, strict enough to
catch real mistakes, and approachable enough that you don’t need a physics
background to use it confidently.

## Design Goals

- **Runtime dimensional safety**  Catch invalid unit operations at runtime,
with errors that explain what went wrong instead of just saying “no”.

- **Performance first**  This library is meant to run in real systems, not just
notebooks and experiments.

- **Low cognitive overhead**  You shouldn’t need to think about dimensional
algebra every time you write code.

- **AstroPy-inspired ergonomics**  Natural arithmetic, readable unit
expressions, and explicit conversions where they matter.

- **Extensible unit systems**  Start with SI units, but don’t paint yourself
into a corner if your domain needs something else.

## Non-Goals

There are some things this library intentionally does *not* try to be:

- A full symbolic algebra system
- A compile-time unit checker
- A physics reasoning engine
- An encyclopedic catalog of every unit system ever invented

iridium-units prioritizes correctness, clarity, and performance over
theoretical completeness.

## Improvements Over AstroPy

While inspired by AstroPy's excellent design, iridium-units makes several
deliberate improvements:

### Exact Rational Exponents

AstroPy uses floating-point for dimensional exponents, which can lead to
rounding errors in edge cases. iridium-units uses `Rational16` (exact fractions)
to ensure dimensional analysis is always precise:

```rust
// √(m²) = m, exactly (exponent: 2 × 1/2 = 1, not 0.9999999...)
let area = M.pow(Rational16::new(2, 1));
let length = area.pow(Rational16::new(1, 2));
assert_eq!(length.dimension(), M.dimension());  // Always passes
```

### Extended Base Dimensions

AstroPy tracks 7 base dimensions. iridium-units tracks 11, adding native support
for quantities common in astrophysics:

| Dimension | AstroPy | iridium-units |
|-----------|---------|---------------|
| Length, Time, Mass, Current, Temperature | ✅ | ✅ |
| Amount (moles) | ✅ | ✅ |
| Luminous Intensity | ✅ | ✅ |
| **Angle** | ❌ | ✅ |
| **Solid Angle** | ❌ | ✅ |
| **Magnitude** | ❌ | ✅ |
| **Photon Count** | ❌ | ✅ |

### Flexible Unit Parsing

iridium-units accepts a wider variety of input formats:

```rust
// Unicode symbols
parse_unit("m²")?;        // Superscript
parse_unit("µm")?;        // Micro sign
parse_unit("Ω")?;         // Ohm symbol

// LaTeX notation
parse_unit("m^{2}")?;     // Braced exponents
parse_unit(r"kg \cdot m")?;  // \cdot multiplication

// Natural language
parse_unit("km per hour")?;

// Astrophysical subscripts
parse_unit("M_sun")?;     // Solar mass
parse_unit("R_jup")?;     // Jupiter radius
```

### Helpful Error Messages

When a unit isn't recognized, iridium-units suggests alternatives:

```rust
let result = parse_unit("metrs");
// Error: unknown unit 'metrs', did you mean 'meters'?
```

### Rust Advantages

- **Type Safety**: Compile-time guarantees that Python can't provide
- **No GIL**: True parallelism for batch processing
- **Zero-Cost Abstractions**: Performance without sacrificing expressiveness
- **Memory Safety**: No null pointer exceptions or buffer overflows

## About iridiumdesign

Iridiumdesign—and the iridiumdesign.com domain—started back in 2000 while I was
finishing design school. At the time, it was meant to support freelance work in
graphic design and web development.

Over the years, as I moved into full-time corporate software engineering,
Iridiumdesign became less of a business and more of a sandbox. It’s where I
experiment, learn, and build things that don’t always fit neatly into my day
job.

These days I’m a senior software engineer and don’t do much design work
anymore, but the *iridium* name stuck. I use it as a prefix for my personal
libraries and projects so they’re easy to identify and group together.

iridium-units is one of those projects: something I built because I needed it,
learned from, and decided was worth sharing.

*Brad Siegfreid*
