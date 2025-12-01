# OpenGolfCoach - Python Bindings

Python bindings for the OpenGolfCoach library.

## Installation

### From Source

```bash
# Install maturin
pip install maturin

# Build and install in development mode
maturin develop --release
```

## Usage

```python
import opengolfcoach
import json

# Create golf shot data
shot = {
    "ball_speed_meters_per_second": 70.0,
    "vertical_launch_angle_degrees": 12.5,
    "horizontal_launch_angle_degrees": -2.0,
    "total_spin_rpm": 2800.0,
    "spin_axis_degrees": 15.0
}

# Calculate derived values
result_json = opengolfcoach.calculate_derived_values(json.dumps(shot))
result = json.loads(result_json)

# Access derived values
print(f"Carry: {result['carry_distance_meters']:.2f} meters")
print(f"Offline: {result['offline_distance_meters']:.2f} meters")
print(f"Backspin: {result['backspin_rpm']:.1f} RPM")
print(f"Sidespin: {result['sidespin_rpm']:.1f} RPM")
```

## Building Wheels

```bash
# Build wheel for current platform
maturin build --release

# Build wheels for multiple platforms (requires Docker)
maturin build --release --manylinux 2014
```
