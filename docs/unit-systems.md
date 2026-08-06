# Unit Systems Reference

iridium-units provides several unit systems. SI and imperial units are always
available; CGS, astrophysical, and logarithmic units sit behind feature flags
of the same name.

## SI Units (`iridium_units::systems::si`)

The International System of Units.

### Base Units

| Unit | Symbol | Quantity |
|------|--------|----------|
| `M` | m | Length (meter) |
| `S` | s | Time (second) |
| `KG` | kg | Mass (kilogram) |
| `A` | A | Electric current (ampere) |
| `K` | K | Temperature (kelvin) |
| `MOL` | mol | Amount of substance (mole) |
| `CD` | cd | Luminous intensity (candela) |
| `RAD` | rad | Angle (radian) |
| `SR` | sr | Solid angle (steradian) |

### Derived Units

| Unit | Symbol | Quantity | Definition |
|------|--------|----------|------------|
| `HZ` | Hz | Frequency | s⁻¹ |
| `N` | N | Force | kg·m/s² |
| `PA` | Pa | Pressure | N/m² |
| `J` | J | Energy | N·m |
| `W` | W | Power | J/s |
| `C` | C | Electric charge | A·s |
| `V` | V | Voltage | W/A |
| `F` | F | Capacitance | C/V |
| `OHM` | Ω | Resistance | V/A |
| `SIEMENS` | S | Conductance | 1/Ω |
| `WB` | Wb | Magnetic flux | V·s |
| `T` | T | Magnetic field | Wb/m² |
| `H` | H | Inductance | Wb/A |
| `LM` | lm | Luminous flux | cd·sr |
| `LX` | lx | Illuminance | lm/m² |
| `BQ` | Bq | Radioactivity | s⁻¹ |
| `GY` | Gy | Absorbed dose | J/kg |
| `SV` | Sv | Equivalent dose | J/kg |
| `KAT` | kat | Catalytic activity | mol/s |

### Prefixed Units

**Length:**
| Unit | Value |
|------|-------|
| `KM` | 10³ m |
| `CM` | 10⁻² m |
| `MM` | 10⁻³ m |
| `UM` | 10⁻⁶ m (micrometer) |
| `NM` | 10⁻⁹ m (nanometer) |
| `PM` | 10⁻¹² m (picometer) |
| `FM` | 10⁻¹⁵ m (femtometer) |

**Mass:**
| Unit | Value |
|------|-------|
| `G` | 10⁻³ kg (gram) |
| `MG` | 10⁻⁶ kg (milligram) |
| `UG` | 10⁻⁹ kg (microgram) |
| `TONNE` | 10³ kg |

**Time:**
| Unit | Value |
|------|-------|
| `MS` | 10⁻³ s (millisecond) |
| `US` | 10⁻⁶ s (microsecond) |
| `NS` | 10⁻⁹ s (nanosecond) |
| `PS` | 10⁻¹² s (picosecond) |
| `MIN` | 60 s |
| `H` | 3600 s (hour) |
| `DAY` | 86400 s |
| `YEAR` | 365.25 × 86400 s (Julian year) |

**Frequency:**
| Unit | Value |
|------|-------|
| `KHZ` | 10³ Hz |
| `MHZ` | 10⁶ Hz |
| `GHZ` | 10⁹ Hz |
| `THZ` | 10¹² Hz |

**Energy:**
| Unit | Value |
|------|-------|
| `KJ` | 10³ J |
| `MJ` | 10⁶ J |
| `EV` | 1.602×10⁻¹⁹ J (electronvolt) |
| `KEV` | 10³ eV |
| `MEV` | 10⁶ eV |
| `GEV` | 10⁹ eV |

**Angles:**
| Unit | Value |
|------|-------|
| `DEG` | π/180 rad (degree) |
| `ARCMIN` | π/10800 rad (arcminute) |
| `ARCSEC` | π/648000 rad (arcsecond) |
| `MAS` | 10⁻³ arcsec (milliarcsecond) |

---

## Astrophysical Units (`iridium_units::systems::astrophysical`)

Behind the `astrophysics` feature flag.

### Distance Units

| Unit | Symbol | Value |
|------|--------|-------|
| `AU` | AU | 1.496×10¹¹ m (astronomical unit) |
| `PARSEC` | pc | 3.086×10¹⁶ m |
| `KPC` | kpc | 10³ pc |
| `MPC` | Mpc | 10⁶ pc |
| `GPC` | Gpc | 10⁹ pc |
| `LIGHT_YEAR` | lyr | 9.461×10¹⁵ m |
| `LIGHT_SECOND` | ls | 2.998×10⁸ m |

### Solar Units

| Unit | Symbol | Value |
|------|--------|-------|
| `SOLAR_MASS` | M☉ | 1.989×10³⁰ kg |
| `SOLAR_RADIUS` | R☉ | 6.957×10⁸ m |
| `SOLAR_LUMINOSITY` | L☉ | 3.828×10²⁶ W |

### Planetary Units

| Unit | Symbol | Value |
|------|--------|-------|
| `JUPITER_MASS` | M_jup | 1.898×10²⁷ kg |
| `JUPITER_RADIUS` | R_jup | 6.991×10⁷ m |
| `EARTH_MASS` | M_earth | 5.972×10²⁴ kg |
| `EARTH_RADIUS` | R_earth | 6.378×10⁶ m |

### Spectroscopic Units

| Unit | Symbol | Dimension | Value |
|------|--------|-----------|-------|
| `ANGSTROM` | Å | Length | 10⁻¹⁰ m |
| `JANSKY` | Jy | Flux density | 10⁻²⁶ W/m²/Hz |
| `MJY` | mJy | Flux density | 10⁻²⁹ W/m²/Hz |
| `UJY` | µJy | Flux density | 10⁻³² W/m²/Hz |

### CGS Spectral Flux Density

| Unit | Symbol | SI Equivalent |
|------|--------|---------------|
| `FLAM_NU` | erg/(s cm² Hz) | 10⁻³ W/m²/Hz |
| `FLAM` | erg/(s cm² Å) | 10⁷ W/m³ |
| `PHOTLAM` | photon/(s cm² Å) | 10¹⁴ photon/m³/s |
| `PHOTNU` | photon/(s cm² Hz) | 10⁴ photon/m²/s/Hz |

### Other Astrophysical Units

| Unit | Symbol | Quantity |
|------|--------|----------|
| `RAYLEIGH` | R | Photon surface brightness |
| `BARN` | barn | Cross section (10⁻²⁸ m²) |
| `ERG` | erg | Energy (10⁻⁷ J) |
| `DYN` | dyn | Force (10⁻⁵ N) |
| `GAUSS` | G | Magnetic field (10⁻⁴ T) |
| `PHOTON` | ph | Photon count |

### Time Units

| Unit | Symbol | Value |
|------|--------|-------|
| `SIDEREAL_DAY` | sday | 86164.09 s |
| `TROPICAL_YEAR` | tyr | 365.24219 days |
| `SIDEREAL_YEAR` | syr | 365.25636 days |

---

## CGS Units (`iridium_units::systems::cgs`)

Centimeter-gram-second system, behind the `cgs` feature flag.

### Base Units

| Unit | Symbol | SI Equivalent |
|------|--------|---------------|
| `CENTIMETER` | cm | 10⁻² m |
| `GRAM` | g | 10⁻³ kg |
| `SECOND` | s | 1 s |

### Derived Units

| Unit | Symbol | SI Equivalent |
|------|--------|---------------|
| `ERG` | erg | 10⁻⁷ J |
| `DYNE` | dyn | 10⁻⁵ N |
| `POISE` | P | 0.1 Pa·s |
| `STOKES` | St | 10⁻⁴ m²/s |
| `GAUSS` | G | 10⁻⁴ T |
| `OERSTED` | Oe | 79.577 A/m |
| `MAXWELL` | Mx | 10⁻⁸ Wb |
| `STILB` | sb | 10⁴ cd/m² |
| `PHOT` | ph | 10⁴ lx |

---

## Logarithmic Units (`iridium_units::systems::logarithmic`)

Units for logarithmic quantities.

### Magnitude Units

| Unit | Symbol | Description |
|------|--------|-------------|
| `MAG` | mag | Generic magnitude |
| `APPARENT_MAG` | m_app | Apparent magnitude |
| `ABSOLUTE_MAG` | M_abs | Absolute magnitude |
| `MILLIMAG` | mmag | 10⁻³ mag |

### Decibel Units

| Unit | Symbol | Description |
|------|--------|-------------|
| `DB` | dB | Decibel |
| `BEL` | B | Bel (10 dB) |

### Other

| Unit | Symbol | Description |
|------|--------|-------------|
| `DEX` | dex | Order of magnitude (log₁₀) |

### Conversion Functions

```rust
use iridium_units::systems::logarithmic::*;

// Magnitude ↔ Flux ratio
let flux_ratio = mag_to_flux_ratio(5.0);      // 0.01
let mag = flux_ratio_to_mag(0.01).unwrap();   // 5.0

// Decibels ↔ Power ratio
let power = db_to_power_ratio(10.0);          // 10.0
let db = power_ratio_to_db(100.0).unwrap();   // 20.0

// Dex ↔ Linear ratio
let ratio = dex_to_ratio(2.0);                // 100.0
let dex = ratio_to_dex(1000.0).unwrap();      // 3.0

// Astronomical utilities
let combined = combine_magnitudes(5.0, 5.0).unwrap();  // ~4.25 mag
let dm = distance_modulus(10.0);              // Distance modulus at 10 pc
let d = distance_from_modulus(5.0);           // Distance from DM = 5
```

---

## Imperial Units (`iridium_units::systems::imperial`)

US customary and Imperial units.

### Length

| Unit | Symbol | SI Equivalent |
|------|--------|---------------|
| `INCH` | in | 0.0254 m |
| `FOOT` | ft | 0.3048 m |
| `YARD` | yd | 0.9144 m |
| `MILE` | mi | 1609.344 m |

### Mass

| Unit | Symbol | SI Equivalent |
|------|--------|---------------|
| `OUNCE` | oz | 0.02835 kg |
| `POUND` | lb | 0.4536 kg |
| `TON_US` | ton | 907.185 kg |

### Volume

| Unit | Symbol | SI Equivalent |
|------|--------|---------------|
| `GALLON` | gal | 3.785×10⁻³ m³ |
| `QUART` | qt | 9.464×10⁻⁴ m³ |
| `PINT` | pt | 4.732×10⁻⁴ m³ |
| `FLUID_OUNCE` | fl oz | 2.957×10⁻⁵ m³ |

---

## Usage Examples

```rust
use iridium_units::prelude::*;

// Convert across systems; conversions are explicit
let trip = 250.0 * MILE;
let in_km = trip.to(KM)?;               // ~402 km

// Dimensional analysis is automatic
let mass = 75.0 * KG;
let accel = 9.8 * &(M / S.pow(2));
let force = (&mass * &accel).to(N)?;    // ~735 N

// Parse a unit known only at runtime
let speed = parse_quantity("65 mi/h")?;
let in_ms = speed.to(M / S)?;           // ~29 m/s
```
