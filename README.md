<p align="center">
  <img
    src="https://raw.githubusercontent.com/iridiumdesign/iridium-units/main/branding/iridium-units-github-banner.png"
    alt="iridium-units — runtime-typed units of measure for Rust"
    width="880">
</p>

A Rust library for units of measure with runtime dimensional analysis.

[![Crates.io](https://img.shields.io/crates/v/iridium-units.svg)](https://crates.io/crates/iridium-units)
[![Documentation](https://docs.rs/iridium-units/badge.svg)](https://docs.rs/iridium-units)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-yellow.svg)](#license)

In iridium-units a unit is a value, not a type. You parse it, store it, pass
it around, and combine it with other units, and the library tracks dimensions
and catches mismatched operations as you go — all at runtime.

```rust
use iridium_units::prelude::*;

// Create quantities by multiplying values with units
let distance = 100.0 * KM;
let time = 2.0 * H;
let speed = &distance / &time;

// Convert between compatible units
let speed_ms = speed.to(M / S).unwrap();
println!("{}", speed_ms);  // 13.888... m/s

// Parse a unit that you only know as a string at runtime
let dist = parse_quantity("42.195 km").unwrap();
let in_miles = dist.to(MILE).unwrap();
println!("{}", in_miles);  // 26.219... mi
```

## Where it fits

Rust already has [`uom`](https://crates.io/crates/uom), an excellent
compile-time dimensional-analysis library. When every unit is known at compile
time, encoding dimensions in the type system is hard to beat — the checks cost
nothing at runtime and mismatches never compile.

iridium-units covers the other case: **units that aren't known until runtime.**
Three things follow from treating a unit as data rather than a type:

1. **Units as data.** File formats, APIs, configuration, and databases declare
   units as strings — `"km/h"`, `"mg/L"`, `"N·m"`. Here a unit is a value you
   parse and carry, not a type you have to know when you compile.

2. **Open-world registry.** Adding a unit is a registry entry, not a new type
   plus `Mul`/`Div` impls for every combination it appears in. An application
   can load its units from a database at startup and parse against them.

3. **Cross-dimension equivalencies.** Some conversions are physically routine
   but dimensionally illegal — mass ↔ energy (E=mc²), temperature ↔ energy
   (kT), wavelength ↔ frequency. These are named, opt-in conversions you ask
   for explicitly, not something the type system silently allows or forbids.

The cost is that dimension checks happen at runtime and return a `Result`
instead of failing to compile. That's the trade: a compile-time-typed library
is the better choice when the units are fixed and known up front.

## Capabilities

### Runtime dimensional analysis

Addition and subtraction require matching dimensions and return a `Result`;
multiplication and division combine dimensions automatically. Incompatible
operations report what went wrong rather than producing a wrong answer.

### Exact rational exponents

Dimensional exponents are stored as exact fractions (`Rational16`), so taking
roots and powers of dimensions never accumulates floating-point error:

```rust
use iridium_units::prelude::*;

// √(m²) = m, exactly
let area = 100.0 * &M.pow(2);
let side = area.pow(Rational16::new(1, 2));
assert_eq!(side.unit().dimension(), M.dimension());
```

### Flexible parsing

Units and quantities parse from several common notations:

```rust
use iridium_units::prelude::*;

parse_unit("m/s^2")?;       // Standard notation
parse_unit("m²")?;          // Unicode superscript
parse_unit("µm")?;          // Unicode micro sign
parse_unit("m^{2}")?;       // LaTeX braces
parse_unit("km per hour")?; // Natural language
# Ok::<(), iridium_units::error::UnitError>(())
```

### Helpful error messages

When a unit isn't recognized, close matches are suggested:

```text
Error: unknown unit 'metrs', did you mean 'meters'?
```

### Custom registries

Applications that define their own units build a registry and parse against it:

```rust
use iridium_units::prelude::*;

let registry = UnitRegistry::with_builtins()
    .with_unit(&["my_unit", "mu"], Unit::from(M));
let unit = registry.parse_unit("my_unit").unwrap();
```

### Base dimensions

Quantities are tracked over eleven base dimensions — the seven SI base
dimensions (length, time, mass, current, temperature, amount, luminous
intensity) plus angle, solid angle, and a couple of domain-specific
extensions — so dimensionally distinct quantities stay distinct.

### High-throughput conversion

For converting many values of the same unit, `batch_convert` /
`batch_convert_into` apply a single conversion factor across a slice, and
`conversion_factor` returns the bare `f64` factor so you can apply it yourself
in a tight loop or hand it to external array code.

## Non-Goals

- A compile-time unit checker
- A full symbolic algebra system
- A physics reasoning engine
- An encyclopedic catalog of every unit system ever invented

## Feature Flags

The core library — SI and imperial units, parsing, custom registries, batch
conversion, mass-energy and temperature equivalencies, dimensionless angles,
and physical constants — is always available. Optional unit systems sit behind
feature flags, all enabled by default:

```toml
[dependencies]
iridium-units = { version = "0.2", default-features = false, features = ["cgs"] }
```

| Feature | What it includes |
|---------|-----------------|
| `cgs` | CGS unit system (centimeter, gram, dyne, erg, gauss, etc.) |
| `astrophysics` | Additional units and equivalencies for astronomy |
| `logarithmic` | Logarithmic units (magnitudes, decibels, dex) and equivalencies |

## Verification

Physical constants are sourced from CODATA 2018; astronomical constants follow
IAU 2015 nominal values. The test suite covers arithmetic with dimensional
analysis, conversion round-trips, equivalencies validated against known
results, parsing across all supported formats, and edge cases (zero, negative,
and invalid inputs).

## Documentation

See the [getting-started guide](docs/getting-started.md) for a fuller tour, or
the [API documentation](https://docs.rs/iridium-units) on docs.rs.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  <https://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in this crate by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
