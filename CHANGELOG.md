# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-01-18

Initial release.

### Added

#### Core Features
- `Quantity` type for values with units
- `Unit` type with `Base`, `Composite`, and `Dimensionless` variants
- `Dimension` type with 11 base dimensions and `Rational16` exponents
- Arithmetic operations with automatic dimensional analysis
- Unit conversion with `.to()` method

#### Unit Systems
- **SI units** (`systems::si`): Base units, derived units, prefixed variants
- **CGS units** (`systems::cgs`): Centimeter-gram-second system
- **Astrophysical units** (`systems::astrophysical`): Parsec, AU, solar units, Jansky, etc.
- **Imperial units** (`systems::imperial`): Foot, mile, pound, gallon, etc.
- **Logarithmic units** (`systems::logarithmic`): Magnitudes, decibels, dex

#### Equivalencies
- `spectral()` - Wavelength ↔ frequency ↔ energy ↔ wavenumber
- `spectral_density()` - Fλ ↔ Fν conversion
- `ab_magnitude()` - Flux density ↔ AB magnitude
- `doppler_radio()`, `doppler_optical()`, `doppler_relativistic()` - Frequency ↔ velocity
- `parallax()` - Parallax angle ↔ distance
- `mass_energy()` - Mass ↔ energy (E=mc²)
- `temperature()` - Temperature scale conversions
- `temperature_energy()` - Temperature ↔ thermal energy (kT)
- `brightness_temperature()`, `brightness_temperature_planck()` - Flux ↔ brightness temperature
- `magnitude_flux()`, `db_power()`, `db_amplitude()`, `dex_ratio()` - Logarithmic conversions
- `dimensionless_angles()` - Treat radians as dimensionless for rotational mechanics

#### Parsing
- Unit string parsing with `parse_unit()`
- Quantity string parsing with `parse_quantity()`
- Unicode support: `m²`, `µm`, `Ω`, `Å`, `°`
- LaTeX notation: `m^{2}`, `\cdot`, `\mu`
- Natural language: `km per hour`
- Astrophysical subscripts: `M_sun`, `R_jup`
- Parentheses: `(kg m)/s^2`
- Error suggestions: "did you mean...?"
- `UnitRegistry` for custom unit definitions

#### Performance
- `batch_convert()` - Convert slices of values (~80x faster)
- `batch_convert_into()` - Zero-allocation batch conversion
- `conversion_factor()` - Get factor for manual/SIMD operations
- Optimized `&Quantity + &Quantity` and `&Quantity - &Quantity` (no cloning)

#### Physical Constants
- CODATA 2018 values in `constants` module
- Fundamental constants: c, h, k, G, e, Nₐ
- Derived constants: Stefan-Boltzmann, Wien displacement, Rydberg
- Particle masses: electron, proton, neutron

#### Documentation
- Getting started guide (`docs/getting-started.md`)
- Equivalencies reference (`docs/equivalencies.md`)
- Unit systems reference (`docs/unit-systems.md`)
- Advanced topics (`docs/advanced.md`)

[0.1.0]: https://github.com/bsiegfreid/iridium-units/releases/tag/v0.1.0
