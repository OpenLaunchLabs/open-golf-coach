"""Type stubs for opengolfcoach Python bindings."""

from typing import TypedDict


class Vector3(TypedDict):
    """3D vector for position/velocity."""

    x: float
    y: float
    z: float


class USCustomaryUnits(TypedDict, total=False):
    """US customary unit conversions."""

    ball_speed_mph: float
    club_speed_mph: float
    carry_distance_yards: float
    total_distance_yards: float
    offline_distance_yards: float
    landing_position_yards: Vector3
    landing_velocity_mph: Vector3
    peak_height_yards: float
    optimal_maximum_distance_yards: float


class DerivedValues(TypedDict, total=False):
    """Derived values calculated by OpenGolfCoach."""

    backspin_rpm: float
    sidespin_rpm: float
    total_spin_rpm: float
    spin_axis_degrees: float
    landing_position: Vector3
    landing_velocity: Vector3
    carry_distance_meters: float
    total_distance_meters: float
    offline_distance_meters: float
    descent_angle_degrees: float
    hang_time_seconds: float
    peak_height_meters: float
    club_speed_meters_per_second: float
    smash_factor: float
    optimal_maximum_distance_meters: float
    distance_efficiency_percent: float
    club_path_degrees: float
    club_face_to_target_degrees: float
    club_face_to_path_degrees: float
    shot_name: str
    shot_rank: str
    shot_color_rgb: str
    us_customary_units: USCustomaryUnits
    pressure_pascals: float
    elevation_meters: float
    temperature_kelvin: float
    humidity_percent: float


def calculate_derived_values(json_input: str) -> str:
    """
    Calculate derived golf shot values from a JSON string.

    The library adds an "open_golf_coach" object with all derived values,
    leaving the original input JSON unchanged.

    Args:
        json_input: JSON string containing golf shot parameters.
            Expected fields:
            - ball_speed_meters_per_second (float, optional)
            - vertical_launch_angle_degrees (float, optional)
            - horizontal_launch_angle_degrees (float, optional)
            - total_spin_rpm (float, optional)
            - spin_axis_degrees (float, optional)
            - backspin_rpm (float, optional)
            - sidespin_rpm (float, optional)
            - us_customary_units (dict, optional) - for mph/yards input

    Returns:
        JSON string with original values plus "open_golf_coach" section
        containing all derived values.

    Raises:
        ValueError: If JSON parsing fails or input format is invalid.

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
    ...


__all__: list[str]
__version__: str
