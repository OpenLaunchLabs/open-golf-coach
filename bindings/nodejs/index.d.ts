export interface Vector3Like {
  x: number;
  y: number;
  z: number;
}

export interface USCustomaryUnits {
  ball_speed_mph?: number;
  club_speed_mph?: number;
  carry_distance_yards?: number;
  total_distance_yards?: number;
  offline_distance_yards?: number;
  landing_position_yards?: Vector3Like;
  landing_velocity_mph?: Vector3Like;
  peak_height_yards?: number;
}

/**
 * Wrapper carrying both right-handed and left-handed perspectives of a value.
 * Hand-dependent outputs (shot names, club path/face) are emitted as `Handed<T>`
 * so consumers pick the perspective matching the player without re-running
 * the calculation.
 */
export interface Handed<T> {
  right_handed: T;
  left_handed: T;
}

/**
 * One sampled point along the simulated ball trajectory.
 *
 * Coordinates: +X forward (target line), +Y right, +Z up. Matches Unreal
 * directly. For Unity, swizzle to `(p.y, p.z, p.x)`. For Three.js / glTF,
 * swizzle to `(p.y, p.z, -p.x)`.
 */
export interface TrajectoryPoint {
  /** Time since start of flight, in seconds */
  t: number;
  /** Forward position (m) */
  x: number;
  /** Right position (m) */
  y: number;
  /** Up position (m) */
  z: number;
  /** Forward velocity (m/s) */
  vx: number;
  /** Right velocity (m/s) */
  vy: number;
  /** Up velocity (m/s) */
  vz: number;
}

/**
 * Sampled ball trajectory, emitted under `open_golf_coach.trajectory` when
 * `trajectory_enabled` is `true`. The internal simulation runs at 500 Hz;
 * `sample_rate_hz` is the (effective, post-clamp) emission rate, and points
 * are linearly interpolated between native integrator steps.
 */
export interface Trajectory {
  sample_rate_hz: number;
  points: TrajectoryPoint[];
}

/**
 * Golf shot data structure
 */
export interface GolfShot {
  /** Ball speed in meters per second */
  ball_speed_meters_per_second?: number;

  /** Ball speed in miles per hour */
  ball_speed_mph?: number;

  /** Vertical launch angle in degrees */
  vertical_launch_angle_degrees?: number;

  /** Horizontal launch angle in degrees (0 = straight, negative = left) */
  horizontal_launch_angle_degrees?: number;

  /** Total spin rate in RPM */
  total_spin_rpm?: number;

  /** Spin axis angle in degrees (0 = pure backspin, positive = hook spin) */
  spin_axis_degrees?: number;

  /** Calculated carry distance in meters */
  carry_distance_meters?: number;

  /** Carry plus estimated roll on a standard fairway (meters) */
  total_distance_meters?: number;

  /** Lateral deviation in meters (negative = left) */
  offline_distance_meters?: number;

  /** Backspin component in RPM */
  backspin_rpm?: number;

  /** Sidespin component in RPM */
  sidespin_rpm?: number;

  /** Estimated clubhead speed in m/s */
  club_speed_meters_per_second?: number;

  /** Estimated clubhead speed in mph */
  club_speed_mph?: number;

  /** Smash factor (ball speed / club speed) */
  smash_factor?: number;

  /** Estimated club path relative to target line, per handedness */
  club_path_degrees?: Handed<number>;

  /** Estimated face orientation relative to target, per handedness */
  club_face_to_target_degrees?: Handed<number>;

  /** Estimated face-to-path relationship, per handedness */
  club_face_to_path_degrees?: Handed<number>;

  /** Classified shot label, per handedness */
  shot_name?: Handed<string>;

  /** Classification rank (S+, S, A, etc.), per handedness */
  shot_rank?: Handed<string>;

  /** Recommended display color for the shot, per handedness */
  shot_color_rgb?: Handed<string>;

  /** Carry distance in yards (converted to meters) */
  carry_distance_yards?: number;

  /** Total distance in yards (converted to meters) */
  total_distance_yards?: number;

  /** Offline distance in yards (converted to meters) */
  offline_distance_yards?: number;

  /** Landing position expressed in yards */
  landing_position_yards?: Vector3Like;

  /** Landing velocity expressed in mph */
  landing_velocity_mph?: Vector3Like;

  /** Peak height in yards */
  peak_height_yards?: number;

  /** Convenience US customary conversions */
  us_customary_units?: USCustomaryUnits;

  /**
   * Opt in to receiving the simulated ball trajectory under
   * `open_golf_coach.trajectory`. Default `false`.
   */
  trajectory_enabled?: boolean;

  /**
   * Down-sample rate for the emitted trajectory, in Hz. Clamped to
   * (0, 500]. Defaults to 500 (native simulation rate) when
   * `trajectory_enabled` is `true` and this is omitted or non-positive.
   */
  trajectory_output_framerate_hz?: number;

  /**
   * Sampled ball trajectory. Present only when `trajectory_enabled` was
   * set to `true` on input.
   */
  trajectory?: Trajectory;
}

/**
 * Calculate derived golf shot values
 *
 * @param shotData - Golf shot parameters
 * @returns Shot data with added derived values
 */
export function calculateDerivedValues(shotData: GolfShot): GolfShot;
