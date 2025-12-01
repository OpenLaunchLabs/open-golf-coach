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

  /** Estimated club path relative to target line */
  club_path_degrees?: number;

  /** Estimated face orientation relative to target */
  club_face_to_target_degrees?: number;

  /** Estimated face-to-path relationship */
  club_face_to_path_degrees?: number;

  /** Classified shot label */
  shot_name?: string;

  /** Classification rank (S+, S, A, etc.) */
  shot_rank?: string;

  /** Recommended display color for the shot */
  shot_color_rgb?: string;

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
}

/**
 * Calculate derived golf shot values
 *
 * @param shotData - Golf shot parameters
 * @returns Shot data with added derived values
 */
export function calculateDerivedValues(shotData: GolfShot): GolfShot;
