# Contributing to OpenGolfCoach

Thank you for your interest in contributing to OpenGolfCoach! This document provides guidelines and instructions for contributing.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/open-golf-coach.git`
3. Create a branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Run tests: `cargo test`
6. Commit and push
7. Open a Pull Request

## Development Setup

### Prerequisites

- Rust 1.70+
- wasm-pack (for WebAssembly builds)
- CMake 3.15+ (for C++ examples)
- Python 3.8+ and maturin (for Python bindings)
- Node.js 16+ (for JavaScript examples)

### Build All Components

```bash
# Core library
cd core
cargo build --release
cargo test

# WebAssembly
wasm-pack build --target nodejs

# Python bindings
cd ../bindings/python
maturin develop

# C++ example
cd ../examples/cpp
mkdir build && cd build
cmake .. && make
```

## Code Style

### Rust

Follow standard Rust conventions:
- Run `cargo fmt` before committing
- Run `cargo clippy` and fix warnings
- Add doc comments for public APIs
- Include unit tests for new functionality

### JavaScript/TypeScript

- Use ES6+ features
- Include JSDoc comments
- Follow existing code style

### Python

- Follow PEP 8
- Use type hints where appropriate
- Include docstrings

### C++

- Use C++11 or later
- Follow existing naming conventions
- Include header documentation

### C#

- Follow Microsoft C# coding conventions
- Use XML documentation comments
- Test in Unity when possible

## Adding New Features

### Core Library Changes

1. Add functionality to `core/src/lib.rs`
2. Add unit tests
3. Update documentation
4. Ensure WebAssembly compatibility
5. Update C FFI if needed

### Adding Calculations

When adding new golf calculations:

1. **Add to GolfShot struct**:
```rust
pub struct GolfShot {
    // ...existing fields...

    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_field: Option<f64>,
}
```

2. **Implement calculation**:
```rust
pub fn calculate_new_value(params: f64) -> f64 {
    // Your calculation here
}
```

3. **Add to calculate_derived_values**:
```rust
impl GolfShot {
    pub fn calculate_derived_values(&mut self) {
        // ...existing code...

        if self.new_field.is_none() {
            if let Some(input) = self.some_input {
                self.new_field = Some(calculate_new_value(input));
            }
        }
    }
}
```

4. **Add tests**:
```rust
#[test]
fn test_new_calculation() {
    let result = calculate_new_value(test_input);
    assert!((result - expected).abs() < 0.01);
}
```

5. **Update all language bindings** to expose new fields

### Documentation

- Update README.md with new features
- Add examples if applicable
- Update BUILDING.md if build process changes
- Include references for physics formulas

## Testing

### Rust Tests

```bash
cd core
cargo test
cargo test --release
```

### Integration Tests

Test each language binding:

```bash
# Node.js
cd examples/nodejs
node example.js

# Python
cd examples/python
python example.py

# C++
cd examples/cpp/build
./golf_example
```

### Manual Testing

- Test in Unity Editor
- Test in Unreal Engine
- Verify WebAssembly in browser

## Physics and Formulas

When implementing golf physics:

1. **Cite sources**: Include references to research papers or physics texts
2. **Use SI units internally**: Meters, radians, m/s
3. **Convert at boundaries**: Convert to/from user-friendly units (degrees, RPM) in the API
4. **Add comments**: Explain the physics behind calculations
5. **Validate**: Compare results with known values or other tools

### Example

```rust
/// Calculate Magnus force on golf ball
///
/// Based on "The Physics of Golf" by Jorgensen (1999)
///
/// # Arguments
/// * `velocity` - Ball velocity in m/s
/// * `spin_rate` - Spin rate in rad/s (converted from RPM at API boundary)
///
/// # Returns
/// Magnus force coefficient
fn calculate_magnus_force(velocity: f64, spin_rate: f64) -> f64 {
    // Implementation with physics comments
}
```

## Pull Request Guidelines

### Before Submitting

- [ ] Code compiles without warnings
- [ ] All tests pass
- [ ] Code is formatted (`cargo fmt`)
- [ ] Clippy is happy (`cargo clippy`)
- [ ] Documentation is updated
- [ ] Examples work
- [ ] Commit messages are clear

### PR Description

Include:
- What: What does this PR do?
- Why: Why is this change needed?
- How: How does it work?
- Testing: How was it tested?
- Breaking changes: Does this break existing APIs?

### Example PR Template

```markdown
## Description
Adds calculation for apex height and flight time.

## Motivation
Users requested the ability to know when the ball reaches peak height.

## Changes
- Added `apex_height_meters` field to `GolfShot`
- Added `flight_time_seconds` field to `GolfShot`
- Implemented trajectory calculation using kinematic equations
- Updated all language bindings
- Added tests and examples

## Testing
- Unit tests pass
- Tested in Node.js, Python, and C++
- Verified against known trajectory data

## Breaking Changes
None - only adds new optional fields.
```

## Improving Physics Model

The current physics model is simplified. Contributions to improve accuracy are welcome!

### Areas for Improvement

1. **Aerodynamics**:
   - More accurate drag coefficient model
   - Reynolds number effects
   - Dimple effects

2. **Spin**:
   - Spin decay over flight
   - Spin loft effects
   - Gear effect

3. **Environmental**:
   - Wind effects
   - Altitude/air density
   - Temperature effects

4. **Ground Interaction**:
   - Roll distance
   - Bounce
   - Lie angle effects

### Adding Physics Improvements

1. Create PR discussing the improvement
2. Include references to research/data
3. Implement in feature branch
4. Include validation data
5. Add tests comparing to known results
6. Document assumptions and limitations

## Language Binding Contributions

### Adding a New Language

1. Create `bindings/LANGUAGE/` directory
2. Implement wrapper around FFI or WASM
3. Create `examples/LANGUAGE/` with working example
4. Add README with build instructions
5. Update main README.md

### Improving Existing Bindings

- Better error handling
- More idiomatic APIs
- Performance optimizations
- Better documentation

## Questions?

- Join us in the Developer section of our [Discord server](https://discord.gg/openlaunch)
- Review documentation

## Code of Conduct

- Be respectful and constructive
- Welcome newcomers
- Focus on what's best for the project
- Accept constructive criticism gracefully

## License

By submitting a pull request, you agree to license your contribution under the Apache 2.0 License.
