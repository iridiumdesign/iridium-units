# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - Unreleased

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

[0.1.0]: https://github.com/iridiumdesign/iridium-units/releases/tag/v0.1.0
