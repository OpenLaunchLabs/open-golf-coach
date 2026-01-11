"""
OpenGolfCoach - Calculate derived golf shot values.

This module provides a Python interface to the Rust-based golf ball
physics engine for calculating trajectory, spin components, and
shot classification.

Example:
    >>> import opengolfcoach
    >>> import json
    >>> shot = {
    ...     "ball_speed_meters_per_second": 70.0,
    ...     "vertical_launch_angle_degrees": 12.5,
    ...     "total_spin_rpm": 2800.0,
    ...     "spin_axis_degrees": 15.0
    ... }
    >>> result_json = opengolfcoach.calculate_derived_values(json.dumps(shot))
    >>> result = json.loads(result_json)
    >>> print(result["open_golf_coach"]["carry_distance_meters"])
"""

from .opengolfcoach import calculate_derived_values

__version__ = "0.1.0"
__all__ = ["calculate_derived_values", "__version__"]
