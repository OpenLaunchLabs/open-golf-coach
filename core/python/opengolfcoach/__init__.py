"""
OpenGolfCoach - Python bindings for golf ball trajectory simulation

This module provides a Python interface to the Rust-based golf ball physics engine.
"""

import json
from . import _opengolfcoach


def calculate_derived_values(input_data):
    """
    Calculate derived golf shot values from input parameters.

    Args:
        input_data: Either a dict or a JSON string containing shot parameters.
                   Expected keys:
                   - ball_speed_meters_per_second (optional)
                   - vertical_launch_angle_degrees (optional)
                   - horizontal_launch_angle_degrees (optional)
                   - total_spin_rpm (optional)
                   - spin_axis_degrees (optional)
                   - backspin_rpm (optional)
                   - sidespin_rpm (optional)

    Returns:
        dict: Input data with added 'open_golf_coach' section containing:
              - backspin_rpm
              - sidespin_rpm
              - total_spin_rpm
              - spin_axis_degrees
              - landing_position
              - landing_velocity
              - carry_distance_meters
              - offline_distance_meters
              - descent_angle_degrees
              - hang_time_seconds

    Example:
        >>> result = calculate_derived_values({
        ...     "ball_speed_meters_per_second": 70.0,
        ...     "vertical_launch_angle_degrees": 12.0,
        ...     "total_spin_rpm": 2800.0,
        ...     "spin_axis_degrees": 15.0
        ... })
        >>> print(result['open_golf_coach']['carry_distance_meters'])
    """
    # Convert dict to JSON string if needed
    if isinstance(input_data, dict):
        json_input = json.dumps(input_data)
    else:
        json_input = input_data

    # Call the FFI function
    ffi = _opengolfcoach.ffi
    lib = _opengolfcoach.lib

    buffer_size = 100000  # 100KB should be plenty for trajectory data
    output_buffer = ffi.new('char[]', buffer_size)
    input_cstr = ffi.new('char[]', json_input.encode('utf-8'))

    result_code = lib.calculate_derived_values_ffi(input_cstr, output_buffer, buffer_size)

    if result_code != 0:
        error_messages = {
            -1: "Null pointer error",
            -2: "Invalid UTF-8 in input",
            -3: "JSON parse error or invalid input format",
            -4: "Serialization error",
            -5: "C string conversion error",
            -6: "Output buffer too small",
        }
        error_msg = error_messages.get(result_code, f"Unknown error code: {result_code}")
        raise RuntimeError(f"OpenGolfCoach FFI error: {error_msg}")

    output_json = ffi.string(output_buffer).decode('utf-8')
    return json.loads(output_json)


__all__ = ['calculate_derived_values']
