"""
OpenGolfCoach - Python Example

This example demonstrates how to use the OpenGolfCoach library
to calculate derived golf shot values.
"""

import opengolfcoach
import json

def print_section(title):
    """Print a section header"""
    print(f"\n{title}")
    print("=" * 60)

def print_shot_data(label, data):
    """Pretty print shot data"""
    print(f"\n{label}:")
    print(json.dumps(data, indent=2, sort_keys=True))

# Example 1: Basic shot with total spin and spin axis
print_section("Example 1: Calculate carry, offline, backspin, and sidespin")

shot1 = {
    "ball_speed_meters_per_second": 70.0,
    "vertical_launch_angle_degrees": 12.5,
    "horizontal_launch_angle_degrees": -2.0,
    "total_spin_rpm": 2800.0,
    "spin_axis_degrees": 15.0
}

print_shot_data("Input", shot1)

result1_json = opengolfcoach.calculate_derived_values(json.dumps(shot1))
result1 = json.loads(result1_json)

print_shot_data("Output", result1)

# Example 2: Shot with backspin and sidespin (calculate total spin and axis)
print_section("Example 2: Calculate total spin and spin axis from components")

shot2 = {
    "ball_speed_meters_per_second": 65.0,
    "vertical_launch_angle_degrees": 14.0,
    "horizontal_launch_angle_degrees": 1.5,
    "backspin_rpm": 3500.0,
    "sidespin_rpm": -800.0  # Negative = slice/fade
}

print_shot_data("Input", shot2)

result2_json = opengolfcoach.calculate_derived_values(json.dumps(shot2))
result2 = json.loads(result2_json)

print_shot_data("Output", result2)

# Example 3: Minimal input (just ball speed and launch angle)
print_section("Example 3: Minimal input")

shot3 = {
    "ball_speed_meters_per_second": 75.0,
    "vertical_launch_angle_degrees": 11.0
}

print_shot_data("Input", shot3)

result3_json = opengolfcoach.calculate_derived_values(json.dumps(shot3))
result3 = json.loads(result3_json)

print_shot_data("Output", result3)

# Example 4: Processing multiple shots
print_section("Example 4: Batch processing multiple shots")

shots = [
    {"ball_speed_meters_per_second": 60.0, "vertical_launch_angle_degrees": 15.0},
    {"ball_speed_meters_per_second": 70.0, "vertical_launch_angle_degrees": 12.0},
    {"ball_speed_meters_per_second": 80.0, "vertical_launch_angle_degrees": 10.0},
]

print("\nProcessing 3 shots...")
results = []
for i, shot in enumerate(shots, 1):
    result_json = opengolfcoach.calculate_derived_values(json.dumps(shot))
    result = json.loads(result_json)
    results.append(result)
    print(f"  Shot {i}: {result['ball_speed_meters_per_second']:.0f} m/s, "
          f"{result['vertical_launch_angle_degrees']:.0f}° -> "
          f"{result['open_golf_coach']['carry_distance_meters']:.1f}m carry")
