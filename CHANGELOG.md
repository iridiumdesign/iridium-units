# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-06-25

Correctness fixes to unit parsing and the global registry. **Two changes alter
parsing behavior and are breaking** — see _Changed_.

### Added

- The nine SI derived units that were defined but never registered now parse:
  **tesla** (`T`), **weber** (`Wb`), **henry** (`H`), **siemens** (`S`),
  **lumen** (`lm`), **lux** (`lx`), **becquerel** (`Bq`), **gray** (`Gy`),
  and **sievert** (`Sv`), plus their long names (#51)
- **megajansky** (`MJy`) unit (#52)
- `register_unit_override` — escape hatch to deliberately replace a built-in
  name in the global registry (#53)

### Changed

- **BREAKING — unit symbols are now case-sensitive.** Lookup is exact-case
  first with a lowercase fallback, so long names and sloppy casing (`METER`,
  `Tesla`) still resolve, but canonical symbols no longer collapse (#52):
  - `T` now means **tesla** (was tonne); lowercase `t` is still tonne
  - `MJy` now means **megajansky** (was silently millijansky); `mJy` is still
    millijansky
  - `S` now means **siemens** (was second); lowercase `s` is still second
  - `H` now means **henry** (was hour); lowercase `h` is still hour
- **BREAKING — `register_unit` now returns `Result<(), UnitError>`** and refuses
  to silently overwrite a built-in name, returning `UnitError::NameTaken`. Use
  `register_unit_override` to replace a built-in deliberately. It also surfaces
  registry lock poisoning instead of silently doing nothing. The owned
  `UnitRegistry::register` is unchanged (#53)

### Fixed

- `Rational16`'s `Mul<i8>` and `Neg` widen intermediate arithmetic to `i32`
  before reducing, removing an `i16` overflow path (panic in debug, wrap in
  release) (#54)
- `benches/performance.rs` compiles against the current API again (#49)
- `cargo clippy --all-targets --all-features` is clean; documented the
  approximate, non-transitive nature of `PartialEq for Quantity` (#50, #54)

## [0.1.0] - 2026-04-12

Initial public release.

### Added

- **Core types**: `Quantity`, `Unit`, `Dimension`, and `BaseUnit` for runtime dimensional analysis
- **Exact rational exponents** via `Rational16` — no floating-point drift in dimension tracking
- **Unit systems**:
  - SI (base units, derived units, and SI-prefixed variants)
  - CGS (centimeter-gram-second with electromagnetic units)
  - Astrophysical (parsec, AU, solar mass/radius/luminosity, Jansky, etc.)
  - Imperial (mile, foot, inch, pound, yard, Fahrenheit)
  - Logarithmic (magnitudes, decibels, dex) with dedicated conversion functions
- **12 equivalencies** for cross-dimensional conversions:
  - Spectral: wavelength ↔ frequency ↔ energy ↔ wavenumber
  - Doppler: radio, optical, and relativistic velocity ↔ frequency
  - Parallax: angle ↔ distance
  - Mass-energy: mass ↔ energy (E=mc²)
  - Temperature-energy: Kelvin ↔ electronvolt (via kT)
  - Brightness temperature: flux density ↔ temperature (Rayleigh-Jeans and Planck)
  - Dimensionless angles: radian/steradian ↔ dimensionless
  - Logarithmic: magnitude ↔ flux ratio, dB ↔ power ratio, dex ↔ linear ratio
- **Unit parsing** with support for Unicode superscripts/symbols, LaTeX braces, natural language ("km per hour"), astrophysical subscripts (M_sun), and parenthesized groups
- **Quantity parsing** from strings (e.g., `"100 km".parse::<Quantity>()`)
- **Custom unit registry** (`UnitRegistry`) for application-defined units
- **Batch conversion API** (`batch_convert`, `batch_convert_into`) for high-throughput workloads
- **Standalone `conversion_factor` function** for getting scale factors without constructing quantities
- **Offset unit support** for temperature scales (Celsius, Fahrenheit) with affine conversions
- **`checked_add` / `checked_sub`** for fallible addition/subtraction that returns `Result`
- **CODATA 2018 physical constants** (speed of light, Planck constant, Boltzmann constant, gravitational constant, and astronomical constants)

[0.2.0]: https://github.com/iridiumdesign/iridium-units/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/iridiumdesign/iridium-units/releases/tag/v0.1.0
