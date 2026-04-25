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


class HandedString(TypedDict):
    """A string value carrying both right- and left-handed perspectives."""

    right_handed: str
    left_handed: str


class HandedFloat(TypedDict):
    """A float value carrying both right- and left-handed perspectives."""

    right_handed: float
    left_handed: float


class TrajectoryPoint(TypedDict):
    """One sampled point along the simulated ball trajectory.

    Coordinates: +X forward (target line), +Y right, +Z up. Matches Unreal
    directly. For Unity, swizzle to (p['y'], p['z'], p['x']). For Three.js,
    swizzle to (p['y'], p['z'], -p['x']).
    """

    t: float  # seconds since start of flight
    x: float  # forward position, meters
    y: float  # right position, meters
    z: float  # up position, meters
    vx: float  # forward velocity, m/s
    vy: float  # right velocity, m/s
    vz: float  # up velocity, m/s


class Trajectory(TypedDict):
    """Sampled ball trajectory.

    Emitted under ``open_golf_coach.trajectory`` when the caller passes
    ``trajectory_enabled=True``. The internal simulation runs at 500 Hz;
    ``sample_rate_hz`` is the (effective, post-clamp) emission rate, and
    points are linearly interpolated between native integrator steps.
    """

    sample_rate_hz: float
    points: list[TrajectoryPoint]


class DerivedValues(TypedDict, total=False):
    """Derived values calculated by OpenGolfCoach.

    Hand-dependent fields (shot_name, shot_rank, shot_color_rgb,
    club_path_degrees, club_face_to_target_degrees, club_face_to_path_degrees)
    are wrapped so each call returns both perspectives — pick the side that
    matches the player.
    """

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
    club_path_degrees: HandedFloat
    club_face_to_target_degrees: HandedFloat
    club_face_to_path_degrees: HandedFloat
    shot_name: HandedString
    shot_rank: HandedString
    shot_color_rgb: HandedString
    us_customary_units: USCustomaryUnits
    pressure_pascals: float
    elevation_meters: float
    temperature_kelvin: float
    humidity_percent: float
    trajectory: Trajectory


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
            - trajectory_enabled (bool, optional) - opt in to receiving the
              simulated ball trajectory under ``open_golf_coach.trajectory``
            - trajectory_output_framerate_hz (float, optional) - down-sample
              rate for the emitted trajectory, clamped to (0, 500]; defaults
              to the native 500 Hz simulation rate

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
