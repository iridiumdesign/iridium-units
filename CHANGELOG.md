# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- **BREAKING — `OERSTED` is now a unit of magnetic field strength H
  (A/m, scale 1000/4π ≈ 79.5775), not of flux density B.** Previously it
  carried B's dimension with an incorrect scale, so `1 Oe` converted to
  79.58 G (the Gaussian correspondence is 1 G). Converting oersted to
  gauss or tesla now returns `DimensionMismatch`; the B ↔ H crossing is
  physics (μ₀) and will arrive as an equivalency (#14, #64)
- `src/systems/cgs.rs` module docs now state the electromagnetic
  policy: CGS EM units map by numerical SI correspondence, so Gaussian
  dimensional identities (e.g. statV/cm ≡ G) deliberately do not hold
- **Dual licensed under MIT OR Apache-2.0**, the Rust ecosystem
  convention, adding Apache-2.0's express patent grant. `LICENSE` is now
  `LICENSE-MIT`, alongside a new `LICENSE-APACHE`. This is not
  retroactive: 0.1.0 through 0.2.1 remain MIT on crates.io

## [0.2.1] - 2026-08-06

Parser correctness fixes. **One change alters parsing behavior** — see
_Changed_: four short mixed-case tokens that previously parsed silently to
the wrong unit now return an error.

### Changed

- **Short mixed-case tokens no longer fall back to lowercase lookup.**
  Tokens of 4 characters or fewer that mix upper- and lowercase must match
  an exact-case key or fail with `UnknownUnit`. Previously `mW`, `Mg`,
  `meV`, and `Ms` silently folded to the lowercase keys and parsed as
  **megawatt**, **milligram**, **MeV**, and **millisecond** — errors of up
  to nine orders of magnitude. They now return an error (with suggestions).
  Long names and single-case tokens (`METER`, `Tesla`, `AU`, `KM`) still
  resolve via the fallback

### Added

- Exact-case registry keys for the canonical mixed-case symbols, so they
  parse as written: `Hz`, `kHz`, `MHz`, `GHz`, `THz`, `Pa`, `eV`, `keV`,
  `MeV`, `GeV`, `kW`, `MW`, `degC`, `degF`, `Ohm`, `Mpc`, `Gpc`

### Fixed

- Display output for pure-inverse units round-trips through the parser:
  the bare token `1` now parses as a dimensionless numerator, so `"1 / s"`
  (the display form of `s^-1`) parses. Other bare numbers are still
  rejected
- Parenthesized units with an exponent followed by a division now parse:
  `"(m^2/s)"` failed with "invalid power denominator: s" while `"m^2/s"`
  worked. The top-level division scanner now uses the same
  fraction-lookahead as the splitter, so `"(m^2/s)"`, `"(kg m^2/s^2)"`,
  and `"(m^1/2/s)"` all parse correctly

### Security

- The parenthesis-aware parser now enforces a recursion depth limit
  (64 nesting levels, 128 total recursion). Previously a hostile input —
  deep nesting, a long division chain, or a long run of adjacent groups —
  could exhaust the stack and abort the process through `parse_unit`,
  `parse_quantity`, or `FromStr`. Such inputs now return
  `UnitError::ParseError`

### Documentation

- README and guides repositioned to lead with runtime-typed units and the
  `uom` comparison; astrophysical units documented behind their feature flag

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

[0.2.1]: https://github.com/iridiumdesign/iridium-units/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/iridiumdesign/iridium-units/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/iridiumdesign/iridium-units/releases/tag/v0.1.0
