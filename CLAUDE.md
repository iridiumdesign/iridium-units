# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

iridium-units is a Rust library for units of measure with runtime dimensional analysis. It is designed to support astrophysics calculations.

## Build Commands

```bash
cargo build          # Build the project
cargo test           # Run all tests
cargo test test_name # Run a single test
cargo clippy         # Run linter
cargo doc --open     # Build and view documentation
```

## Architecture

### Core Types

- **`Dimension`** (`src/dimension.rs`): Represents physical dimensions using 11 base dimensions (length, time, mass, current, temperature, angle, solid_angle, luminous_intensity, magnitude, amount, photon) with rational exponents via `Rational8`.

- **`Unit`** (`src/unit/mod.rs`): Enum representing physical units:
  - `Base(BaseUnit)` - Irreducible units like meter, second
  - `Composite(CompositeUnit)` - Compound units like m/s, kg·m/s²
  - `Dimensionless { scale }` - Dimensionless with optional scale

- **`Quantity`** (`src/quantity.rs`): A value (f64) paired with a Unit. Supports arithmetic with automatic dimensional analysis. Created via `value * &*UNIT`.

### Unit Systems (`src/systems/`)

- **`si.rs`**: SI units (M, S, KG, A, K, etc.) plus derived units (N, J, W, Hz, Pa, etc.) and prefixed variants (KM, MM, NM, GHZ, etc.)
- **`cgs.rs`**: CGS units (CENTIMETER, GRAM, DYNE, ERG, GAUSS, etc.)
- **`astrophysical.rs`**: Astrophysical units (PARSEC, AU, LIGHT_YEAR, SOLAR_MASS, SOLAR_RADIUS, JANSKY, ANGSTROM, etc.)
- **`imperial.rs`**: Imperial units (FOOT, INCH, MILE, POUND, etc.)

Units are defined using `lazy_static!` and must be dereferenced: `100.0 * &*M` or `&*KM / &*H`.

### Equivalencies (`src/equivalencies/`)

Enable conversion between different physical dimensions:

- **`spectral()`**: wavelength ↔ frequency ↔ energy ↔ wavenumber
- **`doppler_radio/optical/relativistic(rest_freq)`**: frequency ↔ velocity
- **`temperature_energy()`**: Kelvin ↔ electronvolt (via kT)
- **`parallax()`**: parallax angle ↔ distance
- **`mass_energy()`**: mass ↔ energy (E=mc²)

Usage: `quantity.to_equiv(&target_unit, equivalency())`

### Physical Constants (`src/constants.rs`)

CODATA 2018 values: `SPEED_OF_LIGHT`, `PLANCK_CONSTANT`, `BOLTZMANN_CONSTANT`, `GRAVITATIONAL_CONSTANT`, plus astronomical constants (`SOLAR_MASS`, `PARSEC`, etc.).

## Key Patterns

### Creating Quantities
```rust
use iridium_units::prelude::*;
let distance = 100.0 * &*M;           // 100 meters
let speed = &distance / &(9.58 * &*S); // velocity
```

### Unit Conversion
```rust
let km_value = distance.to(&*KM)?;    // Convert to km
let si_value = distance.decompose();   // Convert to SI base units
```

### Equivalency Conversion
```rust
use iridium_units::equivalencies::spectral;
let freq = wavelength.to_equiv(&*HZ, spectral())?;
```

### Dimensional Analysis
- Addition/subtraction require matching dimensions (returns `Result`)
- Multiplication/division combine dimensions automatically
- Incompatible operations return `UnitError::DimensionMismatch`
