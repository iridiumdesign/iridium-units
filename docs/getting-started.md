# Getting Started with iridium-units

iridium-units is a Rust library for units of measure with runtime dimensional analysis, designed for astrophysics calculations.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
iridium-units = "0.1"
```

## Basic Usage

### Creating Quantities

Quantities are created by multiplying a number by a unit reference:

```rust
use iridium_units::prelude::*;

// Basic quantities
let distance = 100.0 * M;        // 100 meters
let time = 9.58 * S;             // 9.58 seconds
let mass = 70.0 * KG;            // 70 kilograms

// Derived quantities through arithmetic
let speed = &distance / &time;     // ~10.4 m/s
let energy = 0.5 * &mass * &speed * &speed;  // kinetic energy
```

### Unit Conversion

Convert between compatible units using `.to()`:

```rust
use iridium_units::prelude::*;

let distance = 5.0 * KM;
let in_meters = distance.to(M)?;      // 5000 m
let in_miles = distance.to(MILE)?;    // ~3.1 miles

// Get just the numeric value
let meters_value = distance.to_value(&M)?;  // 5000.0
```

### Dimensional Analysis

The library automatically tracks dimensions and prevents incompatible operations:

```rust
use iridium_units::prelude::*;

let distance = 100.0 * M;
let time = 10.0 * S;

// This works - creates velocity
let velocity = &distance / &time;

// This fails at runtime - can't add meters to seconds
let result = &distance + &time;  // Returns Err(IncompatibleDimensions)
```

### Parsing Units from Strings

Parse unit strings with flexible syntax:

```rust
use iridium_units::prelude::*;

// Simple units
let meter = parse_unit("m")?;
let kilometer = parse_unit("km")?;

// Composite units
let velocity = parse_unit("m/s")?;
let acceleration = parse_unit("m/s^2")?;
let force = parse_unit("kg*m/s^2")?;

// Unicode support
let area = parse_unit("m²")?;
let micro = parse_unit("µm")?;

// Natural language
let speed = parse_unit("km per hour")?;

// LaTeX notation
let energy = parse_unit("kg m^{2} s^{-2}")?;

// Astrophysical subscripts
let solar_mass = parse_unit("M_sun")?;

// Parse quantities (value + unit)
let distance = parse_quantity("100 km")?;
let wavelength = parse_quantity("500 nm")?;
```

## Unit Systems

iridium-units provides several unit systems:

### SI Units (`systems::si`)

```rust
use iridium_units::systems::si::*;

// Base units
let length = 1.0 * M;      // meter
let time = 1.0 * S;        // second
let mass = 1.0 * KG;       // kilogram
let current = 1.0 * A;     // ampere
let temp = 1.0 * K;        // kelvin

// Derived units
let force = 1.0 * N;       // newton
let energy = 1.0 * J;      // joule
let power = 1.0 * W;       // watt
let frequency = 1.0 * HZ;  // hertz
let pressure = 1.0 * PA;   // pascal

// Prefixed units
let km = 1.0 * KM;         // kilometer
let nm = 1.0 * NM;         // nanometer
let GHz = 1.0 * GHZ;       // gigahertz
```

### Astrophysical Units (`systems::astrophysical`)

```rust
use iridium_units::systems::astrophysical::*;

// Distance
let d1 = 1.0 * PARSEC;       // parsec
let d2 = 1.0 * AU;           // astronomical unit
let d3 = 1.0 * LIGHT_YEAR;   // light year

// Solar/planetary
let mass = 1.0 * SOLAR_MASS;
let radius = 1.0 * SOLAR_RADIUS;
let lum = 1.0 * SOLAR_LUMINOSITY;

// Spectroscopy
let flux = 1.0 * JANSKY;     // Jansky (flux density)
let wave = 1.0 * ANGSTROM;   // Angstrom
```

### CGS Units (`systems::cgs`)

```rust
use iridium_units::systems::cgs::*;

let energy = 1.0 * ERG;      // erg
let force = 1.0 * DYNE;      // dyne
let field = 1.0 * GAUSS;     // gauss
```

### Logarithmic Units (`systems::logarithmic`)

```rust
use iridium_units::systems::logarithmic::*;

let star_mag = 5.0 * MAG;           // magnitude
let signal = 10.0 * DB;             // decibel
let order = 2.0 * DEX;              // dex (order of magnitude)
```

## Equivalencies

Equivalencies enable conversions between different physical dimensions when there's a physical relationship. See [equivalencies.md](equivalencies.md) for full documentation.

```rust
use iridium_units::prelude::*;
use iridium_units::equivalencies::spectral;

// Convert wavelength to frequency
let wavelength = 500.0 * NM;
let frequency = wavelength.to_equiv(&HZ, spectral())?;
```

## Physical Constants

Access CODATA 2018 physical constants:

```rust
use iridium_units::constants::*;

let c = SPEED_OF_LIGHT;           // 299792458 m/s
let h = PLANCK_CONSTANT;          // 6.62607015e-34 J·s
let k = BOLTZMANN_CONSTANT;       // 1.380649e-23 J/K
let G = GRAVITATIONAL_CONSTANT;   // 6.67430e-11 m³/(kg·s²)
```

## Error Handling

All fallible operations return `UnitResult<T>`:

```rust
use iridium_units::prelude::*;
use iridium_units::error::UnitError;

let result = (1.0 * M) + (1.0 * S);
match result {
    Ok(sum) => println!("Sum: {}", sum),
    Err(UnitError::IncompatibleDimensions { lhs, rhs }) => {
        println!("Cannot add {} and {}", lhs, rhs);
    }
    Err(e) => println!("Error: {}", e),
}
```

## Performance Tips

For processing large datasets, use the batch conversion API:

```rust
use iridium_units::prelude::*;
use iridium_units::quantity::{batch_convert, conversion_factor};

// Convert 10,000 values at once (~80x faster than individual conversion)
let values_km: Vec<f64> = (0..10000).map(|i| i as f64).collect();
let values_m = batch_convert(&values_km, &KM, &M)?;

// Or get the factor for manual/SIMD operations
let factor = conversion_factor(&KM, &M)?;
```

## Next Steps

- [Equivalencies Guide](equivalencies.md) - Converting between different physical domains
- [Unit Systems Reference](unit-systems.md) - Complete unit reference
- [Advanced Topics](advanced.md) - Custom units, registries, and more
