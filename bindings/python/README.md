# OpenGolfCoach

[![PyPI version](https://badge.fury.io/py/opengolfcoach.svg)](https://badge.fury.io/py/opengolfcoach)
[![Python](https://img.shields.io/pypi/pyversions/opengolfcoach.svg)](https://pypi.org/project/opengolfcoach/)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

High-performance Python library for calculating derived golf shot values. Built with Rust for maximum performance.

## Features

- **Trajectory Calculation**: Calculate carry distance, total distance, offline deviation, hang time, and peak height
- **Spin Analysis**: Convert between total spin/axis and backspin/sidespin components
- **Shot Classification**: Automatic shot naming (Straight, Draw, Fade, etc.) with quality ranking
- **Unit Conversion**: Automatic US customary unit conversions (mph, yards)
- **Club Analytics**: Estimate clubhead speed, smash factor, and club path
- **Cross-Platform**: Native binaries for Linux, macOS, and Windows

## Installation

```bash
pip install opengolfcoach
```

## Quick Start

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
ogc = result["open_golf_coach"]
print(f"Carry: {ogc['carry_distance_meters']:.1f} m ({ogc['us_customary_units']['carry_distance_yards']:.1f} yds)")
print(f"Offline: {ogc['offline_distance_meters']:.1f} m")
print(f"Shot: {ogc['shot_name']} (Rank: {ogc['shot_rank']})")
```

## Input Fields

| Field | Type | Description |
|-------|------|-------------|
| `ball_speed_meters_per_second` | float | Ball speed in m/s |
| `vertical_launch_angle_degrees` | float | Vertical launch angle |
| `horizontal_launch_angle_degrees` | float | Horizontal launch (0 = straight) |
| `total_spin_rpm` | float | Total spin rate |
| `spin_axis_degrees` | float | Spin axis angle |
| `backspin_rpm` | float | Backspin component (alternative to total_spin) |
| `sidespin_rpm` | float | Sidespin component (alternative to spin_axis) |

You can also provide input in US customary units:

```python
shot = {
    "vertical_launch_angle_degrees": 12.0,
    "total_spin_rpm": 2300.0,
    "spin_axis_degrees": 0.0,
    "us_customary_units": {
        "ball_speed_mph": 150.0
    }
}
```

## Output Fields

All derived values are added under the `open_golf_coach` key:

| Field | Description |
|-------|-------------|
| `carry_distance_meters` | Calculated carry distance |
| `total_distance_meters` | Carry plus estimated roll |
| `offline_distance_meters` | Lateral deviation |
| `hang_time_seconds` | Time in air |
| `peak_height_meters` | Maximum height |
| `descent_angle_degrees` | Landing angle |
| `backspin_rpm` | Backspin component |
| `sidespin_rpm` | Sidespin component |
| `total_spin_rpm` | Total spin (if backspin/sidespin provided) |
| `spin_axis_degrees` | Spin axis (if backspin/sidespin provided) |
| `club_speed_meters_per_second` | Estimated clubhead speed |
| `smash_factor` | Ball speed / club speed ratio |
| `shot_name` | Classification (e.g., "Straight", "Draw", "Fade") |
| `shot_rank` | Quality ranking (S+, S, A, B, C, D, E) |
| `shot_color_rgb` | Color code for visualization |
| `landing_position` | 3D landing coordinates {x, y, z} |
| `landing_velocity` | 3D landing velocity {x, y, z} |
| `us_customary_units` | All values in mph/yards |

## Building from Source

```bash
# Install maturin
pip install maturin

# Build and install in development mode
cd bindings/python
maturin develop --release

# Run tests
pip install pytest
pytest tests/ -v
```

## Building Wheels

```bash
# Build wheel for current platform
maturin build --release

# Build wheels for multiple platforms (requires Docker)
maturin build --release --manylinux auto
```

## License

Apache 2.0 License - see [LICENSE](../../LICENSE) for details.

## Links

- [GitHub Repository](https://github.com/OpenLaunchLabs/open-golf-coach)
- [Documentation](https://github.com/OpenLaunchLabs/open-golf-coach#readme)
- [Bug Tracker](https://github.com/OpenLaunchLabs/open-golf-coach/issues)
- [Changelog](https://github.com/OpenLaunchLabs/open-golf-coach/blob/main/CHANGELOG.md)
