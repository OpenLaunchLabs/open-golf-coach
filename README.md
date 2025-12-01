# OpenGolfCoach - Derived Golf Values Library

[![CI Build](https://github.com/OpenLaunchLabs/open-golf-coach/actions/workflows/ci.yml/badge.svg)](https://github.com/OpenLaunchLabs/open-golf-coach/actions/workflows/ci.yml)

A high-performance, cross-platform library for calculating derived golf shot values. Built with Rust and WebAssembly for maximum performance and compatibility.

## Features

Calculate derived golf metrics from basic shot data:

- **Carry, Total & Offline Distance**: Calculate where the ball lands and estimate roll-out for typical fairways
- **Spin Components**: Convert between total spin/spin axis and backspin/sidespin
- **Shot Classification**: Deterministic classification to provide a suggested shot name/rank/color for UI highlights, skills training.
- **US Customary Units**: Automatic mph/yard conversions for common launch-monitor metrics
- **Cross-Platform**: Works in Node.js, Python, C++, Unreal Engine, and Unity

## Supported Platforms

- 🟢 **Node.js / JavaScript** - WebAssembly bindings
- 🐍 **Python** - Native bindings via PyO3
- ⚙️ **C++** - Native library
- 🎮 **Unreal Engine** - C++ integration
- 🎯 **Unity** - C# bindings

## Input Format

The library accepts JSON with standardized field names:

```json
{
  "ball_speed_meters_per_second": 70.0,
  "vertical_launch_angle_degrees": 12.5,
  "horizontal_launch_angle_degrees": -2.0,
  "total_spin_rpm": 2800.0,
  "spin_axis_degrees": 15.0
}
```

You can also supply US customary values if that is what your launch monitor reports. Add a `us_customary_units`
object (either at the root or inside `open_golf_coach`) and OpenGolfCoach will convert everything to metric
before running physics, then regenerate the `us_customary_units` block from the calculated metric values:

```json
{
  "vertical_launch_angle_degrees": 12.5,
  "total_spin_rpm": 2800.0,
  "spin_axis_degrees": 15.0,
  "us_customary_units": {
    "ball_speed_mph": 156.6,
    "carry_distance_yards": 202.9
  }
}
```

## Output Format

The library adds derived values to the input:

```json
{
  "ball_speed_meters_per_second": 70.0,
  "vertical_launch_angle_degrees": 12.5,
  "horizontal_launch_angle_degrees": -2.0,
  "total_spin_rpm": 2800.0,
  "spin_axis_degrees": 15.0,
  "carry_distance_meters": 185.4,
  "total_distance_meters": 201.7,
  "offline_distance_meters": -6.2,
  "backspin_rpm": 2700.5,
  "sidespin_rpm": 724.8,
  "club_speed_meters_per_second": 47.4,
  "smash_factor": 1.48,
  "club_path_degrees": -4.6,
  "club_face_to_target_degrees": -0.4,
  "club_face_to_path_degrees": 4.2,
  "shot_name": "Straight",
  "shot_rank": "S",
  "shot_color_rgb": "0x00B3FF",
  "us_customary_units": {
    "ball_speed_mph": 156.6,
    "club_speed_mph": 106.1,
    "carry_distance_yards": 202.9,
    "total_distance_yards": 220.6,
    "offline_distance_yards": -6.8
  }
}
```

### Shot Classification

Shots are classified using deterministic rules based on horizontal launch angle (HLA) and spin axis:

**Direction** (based on HLA):
- Pull: HLA < -3°
- Straight: -3° ≤ HLA ≤ 3°
- Push: HLA > 3°

**Shape** (based on spin axis):
- Hook: spin_axis < -12°
- Draw: -12° ≤ spin_axis < -3°
- None: -3° ≤ spin_axis ≤ 3°
- Fade: 3° < spin_axis ≤ 12°
- Slice: spin_axis > 12°

**Special cases** (checked first):
- Putt: VLA < 0.1° and ball_speed < 15 m/s
- Worm Burner: VLA < 5° and ball_speed > 20 m/s
- Shanks: |HLA| > 12° and VLA > 12°
- Duck Hook / Banana Slice: extreme spin axis with specific speed/VLA
- Baby Push Draw / Baby Pull Fade: opposite signs with small magnitudes

Classification adds three fields to the output:
- `shot_name` – human friendly label (e.g., `Straight`, `Push Slice`)
- `shot_rank` – gamified ranking (S+, S, A, B, C, D, E)
- `shot_color_rgb` – hex color for UI visualization

Rank colors can be customized in `shot_classification/rank_colors.toml`.

## Building from Source

### Prerequisites

- [Rust](https://rustup.rs/) (1.70+)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/) (for WebAssembly builds)
- Node.js 16+ (for JavaScript examples)
- Python 3.8+ (for Python bindings)
- CMake 3.15+ (for C++ builds)

### Build the Core Library

```bash
# Build WebAssembly module
cd core
wasm-pack build --target web

# Build native Rust library
cargo build --release

# Build Python bindings
cd ../bindings/python
pip install maturin
maturin develop --release

# Build C++ library
cd ../bindings/cpp
mkdir build && cd build
cmake ..
make
```

## Quick Start

### Node.js / JavaScript

```bash
cd examples/nodejs
npm install
node example.js
```

### Python

```bash
cd examples/python
pip install -r requirements.txt
python example.py
```

### C++

```bash
cd examples/cpp
mkdir build && cd build
cmake ..
make
./golf_example
```

### Unity

1. Copy `bindings/unity/OpenGolfCoach.cs` to your Unity project's Assets folder
2. Copy the compiled library to your Plugins folder
3. See `examples/unity/GolfShotExample.cs` for usage

### Unreal Engine

1. Copy `bindings/unreal/` to your project's `Source/ThirdParty/` folder
2. See `examples/unreal/GolfShotComponent.h` for usage

## API Reference

### Core Functions

#### `calculate_derived_values(json_input: string) -> string`

Main function that processes a golf shot JSON and returns it with added derived values.

**Input Fields:**
- `ball_speed_meters_per_second` (required): Ball speed in m/s
- `vertical_launch_angle_degrees` (required): Vertical launch angle
- `horizontal_launch_angle_degrees` (optional): Horizontal launch angle (0 = straight)
- `total_spin_rpm` (optional): Total spin rate
- `spin_axis_degrees` (optional): Spin axis angle

**Added Fields:**
- `carry_distance_meters`: Calculated carry distance
- `offline_distance_meters`: Lateral deviation (negative = left)
- `backspin_rpm`: Backspin component (from total_spin + spin_axis)
- `sidespin_rpm`: Sidespin component (from total_spin + spin_axis)
- `total_spin_rpm`: Total spin (from backspin + sidespin if not provided)
- `spin_axis_degrees`: Spin axis (from backspin + sidespin if not provided)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

Apache 2.0 License - see LICENSE file for details

## Acknowledgments

Built for the golf analytics community to provide accurate, fast, and accessible ball flight calculations.
