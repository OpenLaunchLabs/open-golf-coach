# Python Example

This example demonstrates how to use OpenGolfCoach in a Python application.

## Setup

First, build and install the Python bindings:

```bash
cd ../../bindings/python
pip install maturin
maturin develop --release
```

## Running the Example

```bash
python example.py
```

## Integration in Your Project

Once the package is published to PyPI, you can install it with:

```bash
pip install opengolfcoach
```

Then use it in your code:

```python
import opengolfcoach
import json

shot = {
    "ball_speed_meters_per_second": 70.0,
    "vertical_launch_angle_degrees": 12.5,
    "total_spin_rpm": 2800.0,
    "spin_axis_degrees": 15.0
}

result_json = opengolfcoach.calculate_derived_values(json.dumps(shot))
result = json.loads(result_json)

print(f"Carry: {result['carry_distance_meters']:.2f} meters")
```

## Type Hints

For better IDE support, you can create type stubs:

```python
from typing import TypedDict, Optional

class GolfShot(TypedDict, total=False):
    ball_speed_meters_per_second: float
    vertical_launch_angle_degrees: float
    horizontal_launch_angle_degrees: float
    total_spin_rpm: float
    spin_axis_degrees: float
    carry_distance_meters: float
    offline_distance_meters: float
    backspin_rpm: float
    sidespin_rpm: float
```
