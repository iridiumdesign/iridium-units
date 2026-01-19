# iridium-units Backlog

## Completed Features

### Unit/Quantity String Parsing ✅

Implemented comprehensive parsing with:
- Unit symbols and names with case-insensitive matching
- Composite units: `"m/s"`, `"kg*m/s^2"`, `"m s^-1"`
- Unicode support: `µm`, `Ω`, `°`, `²`, `⁻¹`
- LaTeX notation: `m^{2}`, `\cdot`, `\times`
- Natural language: `"km per hour"`
- Astrophysical subscripts: `M_sun`, `R_jup`
- Parentheses: `"(kg m)/s^2"`
- Error suggestions: "did you mean...?"
- Custom `UnitRegistry` for user-defined units

### Logarithmic Units ✅

Implemented complete logarithmic unit support:
- **Magnitude units**: `MAG`, `APPARENT_MAG`, `ABSOLUTE_MAG`, `MILLIMAG`
- **Decibel units**: `DB`, `BEL`
- **Order of magnitude**: `DEX`

Features:
- All units use the `MAGNITUDE` dimension
- Conversion functions: `mag_to_flux_ratio()`, `flux_ratio_to_mag()`, etc.
- Equivalencies: `magnitude_flux()`, `db_power()`, `db_amplitude()`, `dex_ratio()`
- Quantity helper methods: `is_logarithmic()`, `mag_to_flux_ratio()`, `db_to_power_ratio()`, `dex_to_ratio()`
- Astronomical utilities: `combine_magnitudes()`, `distance_modulus()`, `distance_from_modulus()`

Example usage:
```rust
use iridium_units::prelude::*;
use iridium_units::systems::logarithmic::*;
use iridium_units::equivalencies::magnitude_flux;

// Create magnitude quantities
let star = 5.0 * &*MAG;
let sun = -26.74 * &*APPARENT_MAG;

// Convert to flux ratio using equivalency
let flux = star.to_equiv(&Unit::dimensionless(), magnitude_flux())?;

// Or use the helper method
let flux_ratio = star.mag_to_flux_ratio()?;  // 0.01

// Combine magnitudes (add fluxes)
let combined = combine_magnitudes(5.0, 5.0);  // ≈ 4.25 mag
```

### Brightness Temperature Equivalency ✅

Implemented brightness temperature conversions for radio astronomy:
- **Rayleigh-Jeans approximation**: `brightness_temperature(freq, beam)` - Fast, valid for hν << kT
- **Full Planck function**: `brightness_temperature_planck(freq, beam)` - Accurate at all temperatures
- **Spectral radiance variant**: `brightness_temperature_intensity(freq)` - For W/(m² Hz sr)

Features:
- Context-aware conversions (frequency and beam solid angle)
- Automatic validation of physical constraints
- Helper function `rayleigh_jeans_validity_temperature()` to check approximation validity

Example usage:
```rust
use iridium_units::prelude::*;
use iridium_units::equivalencies::brightness_temperature;
use iridium_units::systems::astrophysical::JANSKY;

// 21 cm hydrogen line observation
let freq = 1.420405751768e9 * &*HZ;
let beam = 1e-6 * &*SR;  // 1 µsr beam

let flux = 1.0 * &*JANSKY;
let temp = flux.to_equiv(&K, brightness_temperature(freq, beam))?;
```

### Performance Optimization ✅

Implemented batch conversion API and optimized reference operations:

**Batch Conversion API** (~80x faster for large datasets):
- `batch_convert(&[f64], &Unit, &Unit)` - Convert a slice of values
- `batch_convert_into(&[f64], &Unit, &Unit, &mut [f64])` - Zero-allocation variant
- `conversion_factor(&Unit, &Unit)` - Get factor for manual/SIMD operations

**Reference Operator Optimization**:
- `&Quantity + &Quantity` and `&Quantity - &Quantity` no longer clone operands
- Direct computation avoids intermediate allocations

**Benchmark Results** (1000 values, km → m):
- Individual conversion: ~12.4 µs
- Batch conversion: ~152 ns (~82x faster)

Example usage:
```rust
use iridium_units::prelude::*;
use iridium_units::quantity::{batch_convert, conversion_factor};

// Convert 10,000 distance values at once
let distances_km: Vec<f64> = (0..10000).map(|i| i as f64).collect();
let distances_m = batch_convert(&distances_km, &KM, &M)?;

// Or get factor for SIMD/external libraries
let factor = conversion_factor(&KM, &M)?;
let manual: Vec<f64> = distances_km.iter().map(|v| v * factor).collect();

// Zero-copy addition with references
let a = 100.0 * &*M;
let b = 50.0 * &*KM;
let sum = (&a + &b)?;  // No cloning of a or b
```

**Zero-Copy Patterns**:
- Use `&Quantity` operations when you need to preserve the original
- Use `batch_convert` for array operations instead of mapping `.to()`
- Get `conversion_factor` once for repeated manual conversions

---

## Planned Features

### Spectral Flux Density Conversions

**Status:** Not started
**Needed for:** Observational astronomy, photometry

Convert between:
- Fλ (per wavelength) ↔ Fν (per frequency)
- Flux density ↔ AB magnitude
- Jansky ↔ erg/s/cm²/Hz

---

### Dimensionless Angles Equivalency

**Status:** Not started
**Needed for:** Rotational mechanics, small angle approximations

Treat angles as dimensionless in specific contexts:
- Rotational energy: E = ½Iω² (ω in rad/s, but rad is dimensionless)
- Small angle: sin(θ) ≈ θ for θ << 1

---

### Pixel/Plate Scale

**Status:** Not started
**Needed for:** CCD astronomy, image processing

Convert between:
- Angular size ↔ pixel count (given plate scale)
- Arcsec/pixel ↔ focal length + pixel size

---

### Photometric Zero Points

**Status:** Not started
**Needed for:** Photometry, magnitude systems

Bridge between:
- Instrumental magnitudes
- Standard photometric systems (Johnson-Cousins, SDSS, etc.)
- Absolute flux calibration

---

### Lower Priority

- **Molar Mass AMU** - Chemistry applications
- **Magnetic Flux Density** - B ↔ H conversion
- **Structured Units** - Complex unit compositions
- **Physical Type Classification** - Velocity, acceleration, energy enums

---

## Comparison with AstroPy

See README.md for detailed comparison. Key differences:

| Feature | AstroPy | iridium-units |
|---------|---------|---------------|
| Dimensional exponents | float | Rational16 (exact) |
| Base dimensions | 7 | 11 |
| Parsing flexibility | Standard | Enhanced (Unicode, LaTeX, "per", subscripts) |
| Error messages | Basic | Suggestions ("did you mean?") |
| Language | Python | Rust (type-safe, no GIL) |
