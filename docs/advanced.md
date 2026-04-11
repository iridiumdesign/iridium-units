# Advanced Topics

## Custom Unit Registries

Create custom registries for domain-specific units:

```rust
use iridium_units::parsing::UnitRegistry;
use iridium_units::prelude::*;

// Create a custom registry
let mut registry = UnitRegistry::with_builtins();

// Add custom units
let my_unit = Unit::Base(BaseUnit::new(
    "my_unit", "mu", &["myunit"],
    Dimension::LENGTH,
    1.5  // 1 my_unit = 1.5 meters
));
registry.register(&["my_unit", "mu"], my_unit);

// Parse with custom registry
let q = registry.parse_quantity("10 mu")?;  // 10 my_units = 15 m
```

### Builder Pattern

```rust
let registry = UnitRegistry::with_builtins()
    .with_unit(&["custom_length"], length_unit)
    .with_unit(&["custom_time"], time_unit)
    .with_unit(&["cl"], length_unit);
```

### Merging Registries

```rust
let mut main_registry = UnitRegistry::with_builtins();
let specialized_registry = create_specialized_registry();
main_registry.merge(&specialized_registry);
```

### Example: Furlongs per Fortnight

The classic obscure velocity unit, useful for proving your unit library actually works:

```rust
use iridium_units::prelude::*;
use iridium_units::parsing::UnitRegistry;

// Create custom units
let furlong = Unit::Base(BaseUnit::new(
    "furlong", "fur", &["furlongs"],
    Dimension::LENGTH,
    201.168  // 1 furlong = 201.168 meters (1/8 mile)
));

let fortnight = Unit::Base(BaseUnit::new(
    "fortnight", "ftn", &["fortnights"],
    Dimension::TIME,
    1_209_600.0  // 14 days in seconds
));

// Register them
let mut registry = UnitRegistry::with_builtins();
registry.register(&["furlong", "fur", "furlongs"], furlong.clone());
registry.register(&["fortnight", "ftn", "fortnights"], fortnight.clone());

// Now convert the speed of light to furlongs per fortnight
let c = 299_792_458.0 * M / S;
let fur_per_ftn = &furlong / &fortnight;
let c_obscure = c.to(&fur_per_ftn)?;
println!("{}", c_obscure);  // ~1.803e12 fur/ftn

// Or parse directly
let speed = registry.parse_quantity("100 fur/ftn")?;
let in_mph = speed.to(&(MILE / H))?;
println!("{}", in_mph);  // ~0.000372 mph (a very slow speed)
```

---

## Dimensional Analysis

### The Dimension Type

Dimensions are represented with 11 base components using rational exponents:

```rust
use iridium_units::dimension::{Dimension, Rational16};

// Velocity: L T⁻¹
let velocity_dim = Dimension::LENGTH
    .mul(&Dimension::TIME.pow(Rational16::new(-1, 1)));

// Energy: M L² T⁻²
let energy_dim = Dimension::MASS
    .mul(&Dimension::LENGTH.pow(Rational16::new(2, 1)))
    .mul(&Dimension::TIME.pow(Rational16::new(-2, 1)));

// Check equality
assert_eq!(energy_dim, J.dimension());
```

### Base Dimensions

| Dimension | Field |
|-----------|-------|
| Length | `length` |
| Time | `time` |
| Mass | `mass` |
| Current | `current` |
| Temperature | `temperature` |
| Angle | `angle` |
| Solid Angle | `solid_angle` |
| Luminous Intensity | `luminous_intensity` |
| Magnitude | `magnitude` |
| Amount | `amount` |
| Photon | `photon` |

### Rational Exponents

Unlike many unit libraries that use floats, iridium-units uses `Rational16` for exact fractional exponents:

```rust
use iridium_units::dimension::Rational16;

// Create rationals
let half = Rational16::new(1, 2);
let third = Rational16::new(1, 3);

// Square root has exponent 1/2
let length = 4.0 * M;
let sqrt_length = length.sqrt();  // 2 m^(1/2)

// Cube root would have exponent 1/3
```

---

## Batch Operations

For high-performance processing of large datasets:

### Basic Batch Conversion

```rust
use iridium_units::quantity::{batch_convert, batch_convert_into, conversion_factor};

// Convert a vector
let km_values: Vec<f64> = (0..10000).map(|i| i as f64).collect();
let m_values = batch_convert(&km_values, &KM, &M)?;

// Zero-allocation conversion into existing buffer
let mut output = vec![0.0; 10000];
batch_convert_into(&km_values, &KM, &M, &mut output)?;

// Get factor for manual SIMD operations
let factor = conversion_factor(&KM, &M)?;
```

### Performance Comparison

| Method | 10,000 values |
|--------|---------------|
| Individual `.to()` | ~130 µs |
| `batch_convert()` | ~1.8 µs |
| Manual factor | ~1.8 µs |

The batch API is ~70-80× faster because it computes the conversion factor once.

---

## Physical Constants

Access CODATA 2018 constants:

```rust
use iridium_units::constants::*;

// Fundamental constants
SPEED_OF_LIGHT        // 299792458 m/s
PLANCK_CONSTANT       // 6.62607015e-34 J·s
BOLTZMANN_CONSTANT    // 1.380649e-23 J/K
GRAVITATIONAL_CONSTANT // 6.67430e-11 m³/(kg·s²)
ELEMENTARY_CHARGE     // 1.602176634e-19 C
AVOGADRO_CONSTANT     // 6.02214076e23 mol⁻¹

// Derived constants
STEFAN_BOLTZMANN      // 5.670374419e-8 W/(m²·K⁴)
WIEN_DISPLACEMENT     // 2.897771955e-3 m·K
RYDBERG_CONSTANT      // 1.097373156816e7 m⁻¹

// Particle masses
ELECTRON_MASS         // 9.1093837015e-31 kg
PROTON_MASS           // 1.67262192369e-27 kg
NEUTRON_MASS          // 1.67492749804e-27 kg

// Astronomical
SOLAR_MASS_CONST      // 1.98840987e30 kg (GM☉/G)
PARSEC_CONST          // 3.0856775814913673e16 m
```

---

## Error Handling

### Error Types

```rust
use iridium_units::error::UnitError;

match result {
    Err(UnitError::DimensionMismatch { from, to }) => {
        // Converting between incompatible dimensions
    }
    Err(UnitError::IncompatibleDimensions { lhs, rhs }) => {
        // Adding/subtracting incompatible dimensions
    }
    Err(UnitError::UnknownUnit { name, suggestions }) => {
        // Unknown unit in parsing, with suggestions
    }
    Err(UnitError::ParseError(msg)) => {
        // General parsing error
    }
    Err(UnitError::NoEquivalency { from, to }) => {
        // No equivalency found for conversion
    }
    Err(UnitError::NotDimensionless) => {
        // Tried to get scalar from non-dimensionless quantity
    }
    Err(UnitError::LogarithmicError(msg)) => {
        // Invalid logarithmic operation
    }
    Err(UnitError::DimensionOverflow) => {
        // Overflow in dimension exponent calculation
    }
    Err(UnitError::BatchError(msg)) => {
        // Error in batch operation
    }
    Ok(value) => { /* success */ }
}
```

### Error Suggestions

Unknown unit errors include suggestions:

```rust
let result = parse_unit("metrs");
// Error: unknown unit 'metrs', did you mean 'meters'?
```

---

## String Parsing Features

### Unicode Support

```rust
parse_unit("m²")?;     // Superscript ²
parse_unit("µm")?;     // Greek mu
parse_unit("Ω")?;      // Greek Omega
parse_unit("Å")?;      // Angstrom symbol
parse_unit("°")?;      // Degree symbol
parse_unit("m⁻¹")?;    // Superscript negative
```

### LaTeX Notation

```rust
parse_unit("m^{2}")?;           // Braced exponents
parse_unit(r"kg \cdot m")?;     // \cdot for multiplication
parse_unit(r"\mu m")?;          // \mu for micro
```

### Natural Language

```rust
parse_unit("km per hour")?;     // "per" for division
parse_unit("meters per second squared")?;
```

### Astrophysical Subscripts

```rust
parse_unit("M_sun")?;    // Solar mass
parse_unit("R_jup")?;    // Jupiter radius
parse_unit("L_sol")?;    // Solar luminosity
```

### Parentheses

```rust
parse_unit("(kg m)/s^2")?;
parse_unit("m/(s^2)")?;
parse_unit("(m/s)^2")?;
```

---

## Implementing Equivalencies

Create custom equivalencies for domain-specific conversions:

```rust
use iridium_units::equivalencies::{Equivalency, Converter};
use iridium_units::unit::Unit;

fn custom_equivalency(context: f64) -> Equivalency {
    Equivalency::new("custom", move |from, to| {
        // Check if this equivalency applies
        let from_dim = from.dimension();
        let to_dim = to.dimension();

        if /* dimensions match criteria */ {
            Some(Converter::new(
                move |x| {
                    // Forward conversion
                    // Return Err(String) for invalid inputs
                    Ok(x * context)
                },
                move |x| {
                    // Backward conversion
                    Ok(x / context)
                },
            ))
        } else {
            None
        }
    })
}
```

### Infallible Converters

For conversions that can't fail:

```rust
Converter::new_infallible(
    |x| x * 2.0,    // Forward
    |x| x / 2.0,    // Backward
)
```

---

## Thread Safety

All unit types are `Send + Sync`, making them safe to use across threads:

```rust
use std::thread;
use iridium_units::prelude::*;

let handles: Vec<_> = (0..4).map(|i| {
    thread::spawn(move || {
        let distance = (i as f64) * KM;
        distance.to(M).unwrap()
    })
}).collect();

for handle in handles {
    let result = handle.join().unwrap();
    println!("{}", result);
}
```

---
