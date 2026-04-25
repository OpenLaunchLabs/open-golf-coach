# API Reference

Complete API reference for OpenGolfCoach library across all language bindings.

## Data Structure

All language bindings use the same JSON-compatible data structure with these fields:

### Input Fields

These fields can be provided as input to the calculation:

| Field | Type | Unit | Description | Required |
|-------|------|------|-------------|----------|
| `ball_speed_meters_per_second` | float | m/s | Ball speed | Yes* |
| `vertical_launch_angle_degrees` | float | degrees | Vertical launch angle | Yes* |
| `horizontal_launch_angle_degrees` | float | degrees | Horizontal launch angle (0 = straight, negative = left for RH golfer) | No |
| `total_spin_rpm` | float | RPM | Total spin rate | No** |
| `spin_axis_degrees` | float | degrees | Spin axis (0 = pure backspin, positive = hook spin) | No** |
| `backspin_rpm` | float | RPM | Backspin component | No** |
| `sidespin_rpm` | float | RPM | Sidespin component (positive = hook/right for RH golfer) | No** |
| `ball_speed_mph` | float | mph | Optional ball speed in mph. Converted to m/s if metric speed missing | No |
| `club_speed_mph` | float | mph | Optional club speed in mph. Converted to m/s if metric speed missing | No |
| `carry_distance_yards` (`carry_yards`) | float | yards | Optional carry distance in yards | No |
| `total_distance_yards` (`total_yards`) | float | yards | Optional total distance in yards | No |
| `offline_distance_yards` (`offline_yards`) | float | yards | Optional offline distance in yards | No |
| `peak_height_yards` | float | yards | Optional peak height in yards | No |
| `landing_position_yards` | Vector3 | yards | Landing position vector in yards | No |
| `landing_velocity_mph` | Vector3 | mph | Landing velocity vector in mph | No |
| `us_customary_units` | object | mph/yards | Optional US customary inputs; converted to metric automatically | No |

*Required for distance calculations
**Provide either (total_spin + spin_axis) OR (backspin + sidespin)

### Output Fields

These fields are calculated and added by the library. **Hand-dependent
outputs** are wrapped as `{ right_handed, left_handed }` objects so consumers
can pick the perspective matching the player without re-running the
calculation. Hand-independent ball-flight and physics values remain plain
scalars.

#### Hand-independent (plain scalars)

| Field | Type | Unit | Description |
|-------|------|------|-------------|
| `carry_distance_meters` | float | meters | Carry distance (where ball lands) |
| `total_distance_meters` | float | meters | Carry plus estimated roll-out on a typical fairway |
| `offline_distance_meters` | float | meters | Lateral deviation (negative = left of target line; ball flight is the same regardless of handedness) |
| `backspin_rpm` | float | RPM | Backspin component (calculated if not provided) |
| `sidespin_rpm` | float | RPM | Sidespin component (calculated if not provided) |
| `total_spin_rpm` | float | RPM | Total spin rate (calculated if not provided) |
| `spin_axis_degrees` | float | degrees | Spin axis angle (calculated if not provided) |
| `club_speed_meters_per_second` | float | m/s | Estimated clubhead speed |
| `smash_factor` | float | ratio | Ball speed divided by club speed |
| `us_customary_units` | object | varies | Convenience conversions (see below) |

`horizontal_launch_angle_degrees`, `spin_axis_degrees`, and `sidespin_rpm`
follow the input's sign convention (positive = right of target line and are NOT wrapped — they
describe objective physics.

#### Hand-dependent (`{ right_handed, left_handed }`)

These six fields each carry both perspectives. Both are always populated.

| Field | Inner type | Description |
|-------|------------|-------------|
| `club_path_degrees` | `{right_handed: float, left_handed: float}` | Estimated club path relative to target line, in the player's swing frame |
| `club_face_to_target_degrees` | `{right_handed: float, left_handed: float}` | Clubface orientation relative to target, in the player's swing frame |
| `club_face_to_path_degrees` | `{right_handed: float, left_handed: float}` | Clubface minus path (face-to-path), in the player's swing frame |
| `shot_name` | `{right_handed: string, left_handed: string}` | Classification label chosen from the shot database |
| `shot_rank` | `{right_handed: string, left_handed: string}` | Gamified rank (S+, S, A, …) |
| `shot_color_rgb` | `{right_handed: string, left_handed: string}` | Recommended UI color for the shot |

#### `us_customary_units`

When metric values are available, OpenGolfCoach automatically includes their US customary counterparts under `open_golf_coach.us_customary_units`:

| Field | Type | Unit | Description |
|-------|------|------|-------------|
| `ball_speed_mph` | float | mph | Ball speed converted from m/s |
| `club_speed_mph` | float | mph | Clubhead speed converted from m/s |
| `carry_distance_yards` | float | yards | Carry distance converted from meters |
| `total_distance_yards` | float | yards | Total distance converted from meters |
| `offline_distance_yards` | float | yards | Offline distance converted from meters |
| `landing_position_yards` | Vector3 | yards | Landing position coordinates converted from meters |
| `landing_velocity_mph` | Vector3 | mph | Landing velocity components converted from m/s |
| `peak_height_yards` | float | yards | Peak height converted from meters |

The same structure can be supplied in the input. Any provided mph/yard values are converted to metric prior to
calculation, and the output still reports the authoritative metric values while regenerating the
`us_customary_units` block for convenience.
