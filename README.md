<p align="center">
  <img
    src="https://raw.githubusercontent.com/iridiumdesign/iridium-units/main/branding/iridium-units-github-banner.png"
    alt="iridium-units — runtime quantities :: rust performance"
    width="880">
</p>

A high-performance runtime unit-of-measure library for Rust.

[![Crates.io](https://img.shields.io/crates/v/iridium-units.svg)](https://crates.io/crates/iridium-units)
[![Documentation](https://docs.rs/iridium-units/badge.svg)](https://docs.rs/iridium-units)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- **Runtime dimensional analysis** — Catch unit errors at runtime with helpful error messages
- **High performance** — Batch conversion API (~80x faster), optimized operators
- **Flexible parsing** — Unicode (`m²`, `µm`, `Ω`), LaTeX (`m^{2}`), natural language (`km per hour`)
- **Comprehensive unit systems** — SI, CGS, astrophysical, imperial, logarithmic
- **12 equivalencies** — Spectral, Doppler, parallax, mass-energy, temperature, and more
- **Exact rational exponents** — No floating-point errors in dimensional analysis

## Quick Start

```rust
use iridium_units::prelude::*;

// Create quantities by multiplying values with units
let distance = 100.0 * KM;
let time = 2.0 * H;
let speed = &distance / &time;

// Convert between compatible units
let speed_ms = speed.to(M / S).unwrap();
println!("{}", speed_ms);  // 13.888... m/s

// Parse from strings
let dist = parse_quantity("42.195 km").unwrap();
let in_miles = dist.to(MILE).unwrap();
println!("{}", in_miles);  // 26.219... mi
```

See the [documentation](docs/getting-started.md) for more examples.

## Where it fits

Rust already has [`uom`](https://crates.io/crates/uom), an excellent
compile-time dimensional-analysis library. Python's astrophysics community
has [`astropy.units`](https://docs.astropy.org/en/stable/units/), an
excellent runtime one. `iridium-units` covers a third corner: runtime-typed,
Rust-fast, with the dimensions and equivalencies astronomy actually uses.

The library exists because four pain points came up while trying to extend
a compile-time-typed approach for astrodynamics:

1. **Units known only at runtime.** File formats like CCSDS OEM and FITS
   declare units as metadata strings. Compile-time libraries want the
   unit as a type.

2. **Cross-dimension equivalencies.** Wavelength ↔ frequency, mass ↔ energy,
   parallax ↔ parsec are physically routine but dimensionally illegal —
   they need a named equivalency, not a phantom-type rewrite.

3. **Non-SI base dimensions.** Astronomical magnitude, solid angle, and
   photon count are first-class in astrophysics but not in an
   SI-by-default dimension set.

4. **Open-world unit registries.** Adding a unit shouldn't require a new
   type plus `Mul`/`Div` impls for every combination it appears in.

For the hot paths where even a runtime check is too much, `conversion_factor`
returns a bare `f64` you can apply yourself in SIMD loops or pass to
external array code. Batch conversion is also available for in-place value
arrays. None of this substitutes for a compile-time-typed library when
that's the right fit — different domains, different costs, different
libraries.

## Overview

iridium-units provides physical units and quantities with automatic dimensional
analysis at runtime. It catches unit errors with helpful error messages, supports
exact rational exponents, and provides comprehensive unit systems for science,
engineering, and everyday calculations.

## Design Goals

- **Runtime dimensional safety** — Catch invalid unit operations at runtime,
with errors that explain what went wrong.

- **Performance first** — Designed for real systems, not just notebooks.
Batch conversion API processes 10,000 values in ~2 µs.

- **Low cognitive overhead** — Natural arithmetic syntax. `100.0 * KM` just works.

- **Ergonomic API** — Readable unit expressions and explicit conversions
where they matter.

- **Extensible** — Start with SI, add domain-specific units through registries.

## Non-Goals

- A full symbolic algebra system
- A compile-time unit checker
- A physics reasoning engine
- An encyclopedic catalog of every unit system ever invented

## Capabilities

### Exact Rational Exponents

Dimensional exponents are stored as exact fractions (`Rational16`), so
dimensional analysis is always precise:

```rust
use iridium_units::prelude::*;

// √(m²) = m, exactly — no floating-point rounding
let area = 100.0 * &M.pow(2);
let side = area.pow(Rational16::new(1, 2));
assert_eq!(side.unit().dimension(), M.dimension());
```

### 11 Base Dimensions

iridium-units tracks 11 base dimensions, including those common in astrophysics:

Length, Time, Mass, Current, Temperature, Angle, Solid Angle,
Luminous Intensity, Magnitude, Amount, Photon Count

### Flexible Unit Parsing

Multiple input formats are supported:

```rust
use iridium_units::prelude::*;

// Standard notation
parse_unit("m/s^2")?;

// Unicode symbols
parse_unit("m²")?;        // Superscript
parse_unit("µm")?;        // Micro sign

// LaTeX notation
parse_unit("m^{2}")?;     // Braced exponents

// Natural language
parse_unit("km per hour")?;
# Ok::<(), iridium_units::error::UnitError>(())
```

### Helpful Error Messages

When a unit isn't recognized, alternatives are suggested:

```text
Error: unknown unit 'metrs', did you mean 'meters'?
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
values. Astronomical constants follow IAU 2015 nominal values.

The test suite includes 335+ tests covering:

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
