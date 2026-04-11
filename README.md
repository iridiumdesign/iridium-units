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
let speed_ms = speed.to(&(&*M / &*S)).unwrap();
println!("{}", speed_ms);  // 13.888... m/s

// Parse from strings
let distance = parse_quantity("100 km").unwrap();
let speed = parse_quantity("9.8 m/s^2").unwrap();
```

See the [documentation](docs/getting-started.md) for more examples.

## Overview

iridium-units is a runtime dimensional analysis library for Rust, designed for
production use. It catches unit errors at runtime with helpful error messages,
supports exact rational exponents to avoid floating-point rounding in
dimensional algebra, and provides comprehensive unit systems for SI, CGS,
astrophysics, and imperial measurement.

## Design Goals

- **Runtime dimensional safety**  Catch invalid unit operations at runtime,
with errors that explain what went wrong instead of just saying “no”.

- **Performance first**  This library is meant to run in real systems, not just
notebooks and experiments.

- **Low cognitive overhead**  You shouldn’t need to think about dimensional
algebra every time you write code.

- **Ergonomic API**  Natural arithmetic, readable unit expressions, and
explicit conversions where they matter.

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

## Capabilities

### Exact Rational Exponents

Dimensional exponents are stored as exact fractions (`Rational16`), so
dimensional analysis is always precise:

```rust
// √(m²) = m, exactly — no floating-point rounding
let area = M.pow(Rational16::new(2, 1));
let length = area.pow(Rational16::new(1, 2));
assert_eq!(length.dimension(), M.dimension());
```

### 11 Base Dimensions

iridium-units tracks 11 base dimensions, including those common in astrophysics:

- Amount
- Angle
- Current
- Length
- Luminous Intensity
- Magnitude
- Mass
- Photon Count
- Solid Angle
- Temperature
- Time

### Flexible Unit Parsing

Multiple input formats are supported:

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

When a unit isn’t recognized, alternatives are suggested:

```rust
let result = parse_unit("metrs");
// Error: unknown unit ‘metrs’, did you mean ‘meters’?
```

## Feature Flags

All features are enabled by default. Disable default features and enable only
what you need to reduce compile scope:

```toml
[dependencies]
iridium-units = { version = "0.1", default-features = false, features = ["astrophysics"] }
```

| Feature | What it includes |
|---------|-----------------|
| `cgs` | CGS unit system (centimeter, gram, dyne, erg, gauss, etc.) |
| `astrophysics` | Astrophysical units (parsec, AU, solar units, Jansky, etc.) and equivalencies (spectral, Doppler, parallax, brightness temperature, spectral density) |
| `logarithmic` | Logarithmic units (magnitudes, decibels, dex) and equivalencies |

The core library (SI, imperial, temperature, mass-energy, dimensionless angles,
parsing, batch conversion, and physical constants) is always available.

## Verification

Physical constants are sourced from CODATA 2018 and verified against published
values. Astronomical constants follow IAU 2015 nominal values. Unit conversion
factors are checked against authoritative references.

The test suite includes 200+ tests covering:

- Arithmetic with dimensional analysis
- Unit conversion round-trips
- Equivalency physics validated against known results
- Edge cases (zero values, negative inputs, invalid conversions)
- Parsing across all supported formats

## Development

This library was developed with AI assistance. The core type system, SI units,
imperial units, and fundamental physics equivalencies (temperature, mass-energy,
dimensionless angles) have been validated through direct use.

The CGS, astrophysics, and logarithmic modules were developed primarily through
AI assistance and verified against published references rather than personal
domain expertise. These modules are behind feature flags. Contributions and
corrections from domain experts are welcome.
