// Core modules
mod clubhead_data;
mod shot_classifier;
mod trajectory;
mod trajectory_analysis;
mod unit_conversions;
mod vector;

// Language bindings (WASM, C FFI, etc.)
pub mod bindings;

// Re-export bindings at crate root for compatibility
pub use bindings::{calculate_derived_values, calculate_derived_values_ffi};

// Re-export public Rust API types
pub use clubhead_data::{
    estimate_club_face_path, estimate_clubhead_speed, get_smash_factor, ClubFacePathEstimates,
};
pub use trajectory::{calculate_trajectory, Trajectory, TrajectoryPoint};
pub use trajectory_analysis::{
    get_apex_position, get_carry_distance, get_descent_angle, get_hang_time, get_landing_position,
    get_landing_velocity, get_offline_distance, get_peak_height, get_time_to_apex,
    get_total_distance,
};
pub use vector::Vector3;

use serde::{Deserialize, Serialize};
use shot_classifier::classify_shot;
use std::f64::consts::PI;
use unit_conversions::{
    meters_per_second_to_mph, meters_to_yards, mph_to_meters_per_second, vector_meters_to_yards,
    vector_mph_to_mps, vector_mps_to_mph, vector_yards_to_meters, yards_to_meters,
};

// Only used in tests, but needed for bindings module
#[cfg(test)]
use serde_json::Value;

/// Derived values calculated by OpenGolfCoach
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DerivedValues {
    // Spin conversions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backspin_rpm: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sidespin_rpm: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_spin_rpm: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub spin_axis_degrees: Option<f64>,

    // Trajectory results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landing_position: Option<Vector3>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub landing_velocity: Option<Vector3>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub carry_distance_meters: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_distance_meters: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline_distance_meters: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub descent_angle_degrees: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hang_time_seconds: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub peak_height_meters: Option<f64>,

    // Clubhead estimates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub club_speed_meters_per_second: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub smash_factor: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimal_maximum_distance_meters: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_efficiency_percent: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub club_path_degrees: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub club_face_to_target_degrees: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub club_face_to_path_degrees: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shot_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shot_rank: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shot_color_rgb: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_customary_units: Option<USCustomaryValues>,

    // Environmental conditions used (only if not provided by user)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pressure_pascals: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub elevation_meters: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature_kelvin: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub humidity_percent: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct USCustomaryValues {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ball_speed_mph: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub club_speed_mph: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub carry_distance_yards: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_distance_yards: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline_distance_yards: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub landing_position_yards: Option<Vector3>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub landing_velocity_mph: Option<Vector3>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub peak_height_yards: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimal_maximum_distance_yards: Option<f64>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct InputUSCustomaryUnits {
    #[serde(default)]
    pub ball_speed_mph: Option<f64>,

    #[serde(default)]
    pub club_speed_mph: Option<f64>,

    #[serde(default)]
    pub carry_distance_yards: Option<f64>,

    #[serde(default)]
    pub total_distance_yards: Option<f64>,

    #[serde(default)]
    pub offline_distance_yards: Option<f64>,

    #[serde(default)]
    pub landing_position_yards: Option<Vector3>,

    #[serde(default)]
    pub landing_velocity_mph: Option<Vector3>,

    #[serde(default)]
    pub peak_height_yards: Option<f64>,
}

impl DerivedValues {
    fn new() -> Self {
        DerivedValues {
            backspin_rpm: None,
            sidespin_rpm: None,
            total_spin_rpm: None,
            spin_axis_degrees: None,
            landing_position: None,
            landing_velocity: None,
            carry_distance_meters: None,
            total_distance_meters: None,
            offline_distance_meters: None,
            descent_angle_degrees: None,
            hang_time_seconds: None,
            peak_height_meters: None,
            club_speed_meters_per_second: None,
            smash_factor: None,
            optimal_maximum_distance_meters: None,
            distance_efficiency_percent: None,
            club_path_degrees: None,
            club_face_to_target_degrees: None,
            club_face_to_path_degrees: None,
            shot_name: None,
            shot_rank: None,
            shot_color_rgb: None,
            us_customary_units: None,
            pressure_pascals: None,
            elevation_meters: None,
            temperature_kelvin: None,
            humidity_percent: None,
        }
    }

    fn populate_us_customary_units(&mut self, ball_speed_mps: Option<f64>) {
        let mut units = self.us_customary_units.clone().unwrap_or_default();

        if units.ball_speed_mph.is_none() {
            if let Some(speed) = ball_speed_mps {
                units.ball_speed_mph = Some(meters_per_second_to_mph(speed));
            }
        }

        if units.club_speed_mph.is_none() {
            if let Some(club_speed) = self.club_speed_meters_per_second {
                units.club_speed_mph = Some(meters_per_second_to_mph(club_speed));
            }
        }

        if units.carry_distance_yards.is_none() {
            if let Some(carry) = self.carry_distance_meters {
                units.carry_distance_yards = Some(meters_to_yards(carry));
            }
        }

        if units.total_distance_yards.is_none() {
            if let Some(total) = self.total_distance_meters {
                units.total_distance_yards = Some(meters_to_yards(total));
            }
        }

        if units.offline_distance_yards.is_none() {
            if let Some(offline) = self.offline_distance_meters {
                units.offline_distance_yards = Some(meters_to_yards(offline));
            }
        }

        if units.landing_position_yards.is_none() {
            if let Some(position) = self.landing_position {
                units.landing_position_yards = Some(vector_meters_to_yards(&position));
            }
        }

        if units.landing_velocity_mph.is_none() {
            if let Some(velocity) = self.landing_velocity {
                units.landing_velocity_mph = Some(vector_mps_to_mph(&velocity));
            }
        }

        if units.peak_height_yards.is_none() {
            if let Some(height) = self.peak_height_meters {
                units.peak_height_yards = Some(meters_to_yards(height));
            }
        }

        if units.optimal_maximum_distance_yards.is_none() {
            if let Some(max_distance) = self.optimal_maximum_distance_meters {
                units.optimal_maximum_distance_yards = Some(meters_to_yards(max_distance));
            }
        }

        if units.has_values() {
            self.us_customary_units = Some(units);
        }
    }
}

impl USCustomaryValues {
    fn has_values(&self) -> bool {
        self.ball_speed_mph.is_some()
            || self.club_speed_mph.is_some()
            || self.carry_distance_yards.is_some()
            || self.total_distance_yards.is_some()
            || self.offline_distance_yards.is_some()
            || self.landing_position_yards.is_some()
            || self.landing_velocity_mph.is_some()
            || self.peak_height_yards.is_some()
    }
}

impl InputUSCustomaryUnits {
    fn has_values(&self) -> bool {
        self.ball_speed_mph.is_some()
            || self.club_speed_mph.is_some()
            || self.carry_distance_yards.is_some()
            || self.total_distance_yards.is_some()
            || self.offline_distance_yards.is_some()
            || self.landing_position_yards.is_some()
            || self.landing_velocity_mph.is_some()
            || self.peak_height_yards.is_some()
    }
}

fn apply_us_unit_inputs(derived: &mut DerivedValues, units: &InputUSCustomaryUnits) {
    if derived.carry_distance_meters.is_none() {
        if let Some(val) = units.carry_distance_yards {
            derived.carry_distance_meters = Some(yards_to_meters(val));
        }
    }
    if derived.total_distance_meters.is_none() {
        if let Some(val) = units.total_distance_yards {
            derived.total_distance_meters = Some(yards_to_meters(val));
        }
    }
    if derived.offline_distance_meters.is_none() {
        if let Some(val) = units.offline_distance_yards {
            derived.offline_distance_meters = Some(yards_to_meters(val));
        }
    }
    if derived.peak_height_meters.is_none() {
        if let Some(val) = units.peak_height_yards {
            derived.peak_height_meters = Some(yards_to_meters(val));
        }
    }
    if derived.landing_position.is_none() {
        if let Some(vec_yds) = units.landing_position_yards {
            derived.landing_position = Some(vector_yards_to_meters(&vec_yds));
        }
    }
    if derived.landing_velocity.is_none() {
        if let Some(vec_mph) = units.landing_velocity_mph {
            derived.landing_velocity = Some(vector_mph_to_mps(&vec_mph));
        }
    }
    if derived.club_speed_meters_per_second.is_none() {
        if let Some(club_mph) = units.club_speed_mph {
            derived.club_speed_meters_per_second = Some(mph_to_meters_per_second(club_mph));
        }
    }
}

/// Input data structure for reading values from JSON
#[derive(Debug, Clone, Deserialize)]
pub struct InputData {
    #[serde(default)]
    ball_speed_meters_per_second: Option<f64>,

    #[serde(default)]
    ball_speed_mph: Option<f64>,

    #[serde(default)]
    vertical_launch_angle_degrees: Option<f64>,

    #[serde(default)]
    horizontal_launch_angle_degrees: Option<f64>,

    #[serde(default)]
    total_spin_rpm: Option<f64>,

    #[serde(default)]
    spin_axis_degrees: Option<f64>,

    #[serde(default)]
    backspin_rpm: Option<f64>,

    #[serde(default)]
    sidespin_rpm: Option<f64>,

    #[serde(default)]
    club_speed_meters_per_second: Option<f64>,

    #[serde(default)]
    club_speed_mph: Option<f64>,

    #[serde(default, alias = "carry_yards")]
    carry_distance_yards: Option<f64>,

    #[serde(default, alias = "total_yards")]
    total_distance_yards: Option<f64>,

    #[serde(default, alias = "offline_yards")]
    offline_distance_yards: Option<f64>,

    #[serde(default)]
    peak_height_yards: Option<f64>,

    #[serde(default)]
    landing_position_yards: Option<Vector3>,

    #[serde(default)]
    landing_velocity_mph: Option<Vector3>,

    // Derived outputs that may come from external sources
    #[serde(default)]
    landing_position: Option<Vector3>,

    #[serde(default)]
    landing_velocity: Option<Vector3>,

    #[serde(default)]
    carry_distance_meters: Option<f64>,

    #[serde(default)]
    total_distance_meters: Option<f64>,

    #[serde(default)]
    offline_distance_meters: Option<f64>,

    #[serde(default)]
    descent_angle_degrees: Option<f64>,

    #[serde(default)]
    hang_time_seconds: Option<f64>,

    #[serde(default)]
    peak_height_meters: Option<f64>,

    #[serde(default)]
    smash_factor: Option<f64>,

    #[serde(default)]
    club_path_degrees: Option<f64>,

    #[serde(default)]
    club_face_to_target_degrees: Option<f64>,

    #[serde(default)]
    club_face_to_path_degrees: Option<f64>,

    #[serde(default)]
    shot_name: Option<String>,

    #[serde(default)]
    shot_rank: Option<String>,

    #[serde(default)]
    shot_color_rgb: Option<String>,

    // Environmental conditions
    #[serde(default)]
    pressure_pascals: Option<f64>,

    #[serde(default)]
    elevation_meters: Option<f64>,

    #[serde(default)]
    temperature_kelvin: Option<f64>,

    #[serde(default)]
    humidity_percent: Option<f64>,

    #[serde(default)]
    us_customary_units: Option<InputUSCustomaryUnits>,
}

/// Calculate all derived values from input data
pub fn calculate_derived_values_from_input(input: &InputData) -> DerivedValues {
    let mut derived = DerivedValues::new();

    macro_rules! copy_if_provided {
        ($field:ident) => {
            if let Some(value) = &input.$field {
                derived.$field = Some(value.clone());
            }
        };
    }

    copy_if_provided!(backspin_rpm);
    copy_if_provided!(sidespin_rpm);
    copy_if_provided!(total_spin_rpm);
    copy_if_provided!(spin_axis_degrees);
    copy_if_provided!(landing_position);
    copy_if_provided!(landing_velocity);
    copy_if_provided!(carry_distance_meters);
    copy_if_provided!(total_distance_meters);
    copy_if_provided!(offline_distance_meters);
    copy_if_provided!(descent_angle_degrees);
    copy_if_provided!(hang_time_seconds);
    copy_if_provided!(peak_height_meters);
    copy_if_provided!(club_speed_meters_per_second);
    copy_if_provided!(smash_factor);
    copy_if_provided!(club_path_degrees);
    copy_if_provided!(club_face_to_target_degrees);
    copy_if_provided!(club_face_to_path_degrees);
    copy_if_provided!(shot_name);
    copy_if_provided!(shot_rank);
    copy_if_provided!(shot_color_rgb);

    let root_us_units = InputUSCustomaryUnits {
        ball_speed_mph: input.ball_speed_mph,
        club_speed_mph: input.club_speed_mph,
        carry_distance_yards: input.carry_distance_yards,
        total_distance_yards: input.total_distance_yards,
        offline_distance_yards: input.offline_distance_yards,
        landing_position_yards: input.landing_position_yards,
        landing_velocity_mph: input.landing_velocity_mph,
        peak_height_yards: input.peak_height_yards,
    };
    if root_us_units.has_values() {
        apply_us_unit_inputs(&mut derived, &root_us_units);
    }

    if let Some(us_units) = &input.us_customary_units {
        apply_us_unit_inputs(&mut derived, us_units);
    }

    let mut ball_speed_mps = input.ball_speed_meters_per_second;
    if ball_speed_mps.is_none() {
        if let Some(speed_mph) = input.ball_speed_mph {
            ball_speed_mps = Some(mph_to_meters_per_second(speed_mph));
        }
    }
    if ball_speed_mps.is_none() {
        if let Some(us_units) = &input.us_customary_units {
            if let Some(speed_mph) = us_units.ball_speed_mph {
                ball_speed_mps = Some(mph_to_meters_per_second(speed_mph));
            }
        }
    }

    // Calculate spin components if we have total spin and spin axis, but not if already provided
    if derived.backspin_rpm.is_none() || derived.sidespin_rpm.is_none() {
        if let (Some(total_spin), Some(spin_axis)) = (input.total_spin_rpm, input.spin_axis_degrees)
        {
            let (backspin, sidespin) = calculate_spin_components(total_spin, spin_axis);
            if derived.backspin_rpm.is_none() {
                derived.backspin_rpm = Some(backspin);
            }
            if derived.sidespin_rpm.is_none() {
                derived.sidespin_rpm = Some(sidespin);
            }
        }
    }

    // Calculate total spin and spin axis if we have backspin and sidespin, but not if already provided
    if derived.total_spin_rpm.is_none() || derived.spin_axis_degrees.is_none() {
        if let (Some(backspin), Some(sidespin)) = (input.backspin_rpm, input.sidespin_rpm) {
            let (total_spin, spin_axis) = calculate_total_spin_and_axis(backspin, sidespin);
            if derived.total_spin_rpm.is_none() {
                derived.total_spin_rpm = Some(total_spin);
            }
            if derived.spin_axis_degrees.is_none() {
                derived.spin_axis_degrees = Some(spin_axis);
            }
        }
    }

    // Calculate trajectory-based values
    if let (Some(ball_speed), Some(v_angle)) = (ball_speed_mps, input.vertical_launch_angle_degrees)
    {
        let h_angle = input.horizontal_launch_angle_degrees.unwrap_or(0.0);

        // Use provided or derived spin values
        let backspin = input.backspin_rpm.or(derived.backspin_rpm).unwrap_or(0.0);
        let sidespin = input.sidespin_rpm.or(derived.sidespin_rpm).unwrap_or(0.0);

        // Environmental conditions: use provided values or defaults
        // Defaults from trajectory.rs: ELEVATION_M = 0.0, TEMPERATURE_C = 25.0, RELATIVE_HUMIDITY = 0.50
        const DEFAULT_ELEVATION_M: f64 = 0.0;
        const DEFAULT_TEMPERATURE_K: f64 = 298.15; // 25°C in Kelvin
        const DEFAULT_HUMIDITY_PERCENT: f64 = 50.0;

        let elevation_m = input.elevation_meters.unwrap_or(DEFAULT_ELEVATION_M);
        let temperature_k = input.temperature_kelvin.unwrap_or(DEFAULT_TEMPERATURE_K);
        let humidity_percent = input.humidity_percent.unwrap_or(DEFAULT_HUMIDITY_PERCENT);

        // If user didn't provide these values, we'll add them to derived output later
        let should_output_elevation = input.elevation_meters.is_none();
        let should_output_temperature = input.temperature_kelvin.is_none();
        let should_output_humidity = input.humidity_percent.is_none();
        let should_output_pressure = input.pressure_pascals.is_none();

        // Only calculate trajectory if we need any trajectory-derived values
        let needs_trajectory = derived.landing_position.is_none()
            || derived.landing_velocity.is_none()
            || derived.carry_distance_meters.is_none()
            || derived.total_distance_meters.is_none()
            || derived.offline_distance_meters.is_none()
            || derived.descent_angle_degrees.is_none()
            || derived.hang_time_seconds.is_none()
            || derived.peak_height_meters.is_none();

        if needs_trajectory {
            let trajectory = calculate_trajectory(
                ball_speed,
                v_angle,
                h_angle,
                backspin,
                sidespin,
                elevation_m,
                temperature_k,
                humidity_percent,
                input.pressure_pascals,
            );

            if derived.carry_distance_meters.is_none() {
                derived.carry_distance_meters = Some(get_carry_distance(&trajectory));
            }
            if derived.total_distance_meters.is_none() {
                derived.total_distance_meters = Some(get_total_distance(&trajectory));
            }
            if derived.offline_distance_meters.is_none() {
                derived.offline_distance_meters = Some(get_offline_distance(&trajectory));
            }
            if derived.descent_angle_degrees.is_none() {
                derived.descent_angle_degrees = Some(get_descent_angle(&trajectory));
            }
            if derived.hang_time_seconds.is_none() {
                derived.hang_time_seconds = Some(get_hang_time(&trajectory));
            }
            if derived.peak_height_meters.is_none() {
                derived.peak_height_meters = Some(get_peak_height(&trajectory));
            }

            if derived.landing_position.is_none() {
                derived.landing_position = Some(get_landing_position(&trajectory));
            }
            if derived.landing_velocity.is_none() {
                derived.landing_velocity = Some(get_landing_velocity(&trajectory));
            }
        }

        // Determine clubhead speed: use provided value (metric or converted) if available
        let club_speed = if let Some(measured_speed) = derived.club_speed_meters_per_second {
            measured_speed
        } else {
            // Estimate clubhead speed from ball launch conditions
            // Calculate total spin (use provided or derived values)
            let total_spin = input
                .total_spin_rpm
                .or(derived.total_spin_rpm)
                .unwrap_or_else(|| {
                    // Calculate from backspin/sidespin if available
                    let bs = backspin;
                    let ss = sidespin;
                    (bs.powi(2) + ss.powi(2)).sqrt()
                });

            estimate_clubhead_speed(ball_speed, v_angle, total_spin)
        };

        // Only set club_speed if not provided
        if derived.club_speed_meters_per_second.is_none() {
            derived.club_speed_meters_per_second = Some(club_speed);
        }

        // Calculate smash factor (ball speed / club speed ratio) only if not provided
        if input.smash_factor.is_none() {
            derived.smash_factor = Some(get_smash_factor(ball_speed, club_speed));
        }

        // Calculate distance efficiency (carry distance vs theoretical maximum)
        // Theoretical maximum is 2.4 yards per mph club speed, or 4.91 meters per m/s club speed
        if let Some(carry_meters) = derived.carry_distance_meters {
            let theoretical_max_meters = club_speed * 4.91;
            let efficiency = carry_meters / theoretical_max_meters * 100.0;
            derived.optimal_maximum_distance_meters = Some(theoretical_max_meters);
            derived.distance_efficiency_percent = Some(efficiency.round());
        }

        // Estimate club face/path relationship when we have horizontal launch data, but only if not provided
        let needs_face_path = derived.club_path_degrees.is_none()
            || derived.club_face_to_target_degrees.is_none()
            || derived.club_face_to_path_degrees.is_none();

        if needs_face_path {
            if let Some(spin_axis) = input.spin_axis_degrees.or(derived.spin_axis_degrees) {
                if input.horizontal_launch_angle_degrees.is_some() {
                    let estimates = estimate_club_face_path(ball_speed, h_angle, spin_axis);
                    if derived.club_path_degrees.is_none() {
                        derived.club_path_degrees = Some(estimates.club_path_degrees);
                    }
                    if derived.club_face_to_target_degrees.is_none() {
                        derived.club_face_to_target_degrees =
                            Some(estimates.club_face_to_target_degrees);
                    }
                    if derived.club_face_to_path_degrees.is_none() {
                        derived.club_face_to_path_degrees =
                            Some(estimates.club_face_to_path_degrees);
                    }
                }
            }
        }

        // Shot classification (vector similarity) if not already provided
        let needs_shot_classification = derived.shot_name.is_none()
            || derived.shot_rank.is_none()
            || derived.shot_color_rgb.is_none();

        if needs_shot_classification {
            let total_spin_for_classification =
                input.total_spin_rpm.or(derived.total_spin_rpm).or_else(|| {
                    derived
                        .backspin_rpm
                        .zip(derived.sidespin_rpm)
                        .map(|(bs, ss)| (bs.powi(2) + ss.powi(2)).sqrt())
                });
            let spin_axis_for_classification =
                input.spin_axis_degrees.or(derived.spin_axis_degrees);

            if let (Some(ball_speed), Some(total_spin), Some(spin_axis)) = (
                ball_speed_mps,
                total_spin_for_classification,
                spin_axis_for_classification,
            ) {
                if let Some(classification) =
                    classify_shot(ball_speed, v_angle, h_angle, total_spin, spin_axis)
                {
                    if derived.shot_name.is_none() {
                        derived.shot_name = Some(classification.shot_name);
                    }
                    if derived.shot_rank.is_none() {
                        derived.shot_rank = Some(classification.shot_rank);
                    }
                    if derived.shot_color_rgb.is_none() {
                        derived.shot_color_rgb = Some(classification.shot_color_rgb);
                    }
                }
            }
        }

        // Add environmental defaults to output if user didn't provide them
        if should_output_elevation {
            derived.elevation_meters = Some(elevation_m);
        }
        if should_output_temperature {
            derived.temperature_kelvin = Some(temperature_k);
        }
        if should_output_humidity {
            derived.humidity_percent = Some(humidity_percent);
        }
        if should_output_pressure {
            // Calculate pressure from elevation if not provided
            // Helper function in trajectory.rs converts elevation to hPa, then convert to Pa
            let pressure_hpa = 1013.25 * (1.0 - (0.0065 * elevation_m) / 288.15).powf(5.255);
            derived.pressure_pascals = Some(pressure_hpa * 100.0);
        }
    }

    derived.populate_us_customary_units(ball_speed_mps);

    derived
}

/// Convert total spin and spin axis to backspin and sidespin components
///
/// # Arguments
/// * `total_spin_rpm` - Total spin rate in RPM
/// * `spin_axis_degrees` - Spin axis angle in degrees (0 = pure backspin, positive = hook spin)
///
/// # Returns
/// (backspin_rpm, sidespin_rpm)
pub fn calculate_spin_components(total_spin_rpm: f64, spin_axis_degrees: f64) -> (f64, f64) {
    let spin_axis_rad = spin_axis_degrees * PI / 180.0;

    // Backspin is the cosine component
    let backspin = total_spin_rpm * spin_axis_rad.cos();

    // Sidespin is the sine component
    let sidespin = total_spin_rpm * spin_axis_rad.sin();

    (backspin, sidespin)
}

/// Convert backspin and sidespin to total spin and spin axis
///
/// # Arguments
/// * `backspin_rpm` - Backspin rate in RPM
/// * `sidespin_rpm` - Sidespin rate in RPM (positive = hook/right spin for RH golfer)
///
/// # Returns
/// (total_spin_rpm, spin_axis_degrees)
pub fn calculate_total_spin_and_axis(backspin_rpm: f64, sidespin_rpm: f64) -> (f64, f64) {
    // Total spin is the magnitude of the vector
    let total_spin = (backspin_rpm.powi(2) + sidespin_rpm.powi(2)).sqrt();

    // Spin axis is the angle of the vector
    let spin_axis_rad = sidespin_rpm.atan2(backspin_rpm);
    let spin_axis_degrees = spin_axis_rad * 180.0 / PI;

    (total_spin, spin_axis_degrees)
}

#[cfg(test)]
mod tests {
    use super::*;
    use unit_conversions::{mph_to_meters_per_second, yards_to_meters};

    #[test]
    fn test_spin_components() {
        let (backspin, sidespin) = calculate_spin_components(3000.0, 0.0);
        assert!((backspin - 3000.0).abs() < 0.1);
        assert!(sidespin.abs() < 0.1);

        let (backspin, sidespin) = calculate_spin_components(3000.0, 45.0);
        assert!((backspin - 2121.3).abs() < 1.0);
        assert!((sidespin - 2121.3).abs() < 1.0);
    }

    #[test]
    fn test_total_spin_and_axis() {
        let (total, axis) = calculate_total_spin_and_axis(3000.0, 0.0);
        assert!((total - 3000.0).abs() < 0.1);
        assert!(axis.abs() < 0.1);

        let (total, axis) = calculate_total_spin_and_axis(2121.3, 2121.3);
        assert!((total - 3000.0).abs() < 1.0);
        assert!((axis - 45.0).abs() < 0.1);
    }

    #[test]
    fn test_trajectory_calculation() {
        // Test driver shot: 120 mph (~53.6 m/s), 12° launch, 2300 RPM backspin
        let trajectory =
            calculate_trajectory(53.6, 12.0, 0.0, 2300.0, 0.0, 0.0, 298.15, 50.0, None);
        let landing_pos = get_landing_position(&trajectory);
        let _landing_vel = get_landing_velocity(&trajectory);

        // Trajectory should have multiple points
        assert!(
            trajectory.points.len() > 1,
            "Trajectory should have multiple points"
        );

        // Physics model produces realistic carry distances
        let carry = landing_pos.magnitude();
        assert!(
            carry > 100.0 && carry < 300.0,
            "Carry distance {} should be reasonable for driver",
            carry
        );

        // Should be near centerline (Y is left/right)
        assert!(
            landing_pos.y.abs() < 10.0,
            "Offline {} should be minimal with no sidespin",
            landing_pos.y
        );

        // Should have landed (Z is up/down)
        assert!(
            landing_pos.z <= 0.1,
            "Should have landed, z = {}",
            landing_pos.z
        );

        // Forward component should be positive (X is forward)
        assert!(landing_pos.x > 0.0, "Forward distance should be positive");
    }

    #[test]
    fn test_total_distance_includes_roll() {
        let trajectory =
            calculate_trajectory(70.0, 11.0, 0.0, 2200.0, 0.0, 0.0, 298.15, 50.0, None);
        let carry = get_carry_distance(&trajectory);
        let total = get_total_distance(&trajectory);
        assert!(
            total >= carry,
            "Total distance {} should be at least the carry {}",
            total,
            carry
        );
        assert!(
            total - carry < 25.0,
            "Roll {} should stay within realistic bounds",
            total - carry
        );
        assert!(
            total - carry > 1.0,
            "Roll {} should be non-trivial for a driver",
            total - carry
        );
    }

    #[test]
    fn test_total_distance_matches_sample_shot() {
        let (backspin, sidespin) = calculate_spin_components(2800.0, 15.0);
        let trajectory = calculate_trajectory(
            70.0, 12.5, -2.0, backspin, sidespin, 0.0, 298.15, 50.0, None,
        );
        let carry = get_carry_distance(&trajectory);
        let total = get_total_distance(&trajectory);
        let roll = total - carry;
        assert!(
            carry > 215.0 && carry < 230.0,
            "Carry {} should be realistic for this shot",
            carry
        );
        assert!(
            roll > 5.0 && roll < 15.0,
            "Roll {} should be near ~10 yards for this shot",
            roll
        );
    }

    #[test]
    fn test_trajectory_time() {
        // Test that time increases correctly
        let trajectory =
            calculate_trajectory(53.6, 12.0, 0.0, 2300.0, 0.0, 0.0, 298.15, 50.0, None);

        // First point should be at t=0
        assert_eq!(trajectory.points[0].t, 0.0, "Initial time should be 0");

        // Time should increase monotonically
        for i in 1..trajectory.points.len() {
            assert!(
                trajectory.points[i].t > trajectory.points[i - 1].t,
                "Time should increase monotonically"
            );
        }

        // Last point should have non-zero time (ball took some time to land)
        let final_time = trajectory.points.last().unwrap().t;
        assert!(final_time > 0.0, "Flight time should be positive");
        assert!(
            final_time < 30.0,
            "Flight time should be reasonable (< 30s)"
        );
    }

    #[test]
    fn test_json_output_structure() {
        let json_input = r#"{
            "ball_speed_meters_per_second": 70.0,
            "vertical_launch_angle_degrees": 12.0,
            "horizontal_launch_angle_degrees": -2.0,
            "total_spin_rpm": 2800.0,
            "spin_axis_degrees": 15.0
        }"#;

        let result = calculate_derived_values(json_input).unwrap();
        let output: Value = serde_json::from_str(&result).unwrap();

        // Original fields should be preserved
        assert_eq!(output["ball_speed_meters_per_second"], 70.0);
        assert_eq!(output["vertical_launch_angle_degrees"], 12.0);

        // Should have open_golf_coach section
        assert!(output.get("open_golf_coach").is_some());

        // Derived values should be in open_golf_coach
        let derived = &output["open_golf_coach"];
        assert!(derived.get("carry_distance_meters").is_some());
        assert!(derived.get("total_distance_meters").is_some());
        assert!(derived.get("backspin_rpm").is_some());
        assert!(derived.get("sidespin_rpm").is_some());
        assert!(derived.get("hang_time_seconds").is_some());
        assert!(derived.get("peak_height_meters").is_some());
        assert!(derived.get("club_path_degrees").is_some());
        assert!(derived.get("club_face_to_target_degrees").is_some());
        assert!(derived.get("club_face_to_path_degrees").is_some());
        assert!(derived.get("shot_name").is_some());
        assert!(derived.get("shot_rank").is_some());
        assert!(derived.get("shot_color_rgb").is_some());
        assert!(derived.get("us_customary_units").is_some());

        // Verify hang_time_seconds is reasonable
        let hang_time = derived["hang_time_seconds"].as_f64().unwrap();
        assert!(
            hang_time > 0.0 && hang_time < 30.0,
            "Hang time {} should be reasonable",
            hang_time
        );

        // Verify peak_height_meters is reasonable
        let peak_height = derived["peak_height_meters"].as_f64().unwrap();
        assert!(
            peak_height > 0.0 && peak_height < 100.0,
            "Peak height {} should be reasonable",
            peak_height
        );

        let us_units = &derived["us_customary_units"];
        assert!(us_units.get("ball_speed_mph").is_some());
        assert!(us_units.get("carry_distance_yards").is_some());
        assert!(us_units.get("club_speed_mph").is_some());
    }

    #[test]
    fn test_preserve_provided_derived_values() {
        let json_input = r#"{
            "ball_speed_meters_per_second": 70.0,
            "vertical_launch_angle_degrees": 12.0,
            "horizontal_launch_angle_degrees": -1.5,
            "total_spin_rpm": 2800.0,
            "spin_axis_degrees": 15.0,
            "open_golf_coach": {
                "carry_distance_meters": 150.0,
                "club_path_degrees": -3.5,
                "smash_factor": 1.42
            }
        }"#;

        let result = calculate_derived_values(json_input).unwrap();
        let output: Value = serde_json::from_str(&result).unwrap();

        let derived = &output["open_golf_coach"];
        assert!((derived["carry_distance_meters"].as_f64().unwrap() - 150.0).abs() < 1e-6);
        assert!((derived["club_path_degrees"].as_f64().unwrap() + 3.5).abs() < 1e-6);
        assert!((derived["smash_factor"].as_f64().unwrap() - 1.42).abs() < 1e-6);
    }

    #[test]
    fn test_us_customary_unit_values() {
        let json_input = r#"{
            "ball_speed_meters_per_second": 53.6,
            "vertical_launch_angle_degrees": 12.0,
            "horizontal_launch_angle_degrees": 0.0,
            "total_spin_rpm": 2300.0,
            "spin_axis_degrees": 0.0
        }"#;

        let result = calculate_derived_values(json_input).unwrap();
        let output: Value = serde_json::from_str(&result).unwrap();
        let derived = &output["open_golf_coach"];
        let us_units = &derived["us_customary_units"];

        let ball_speed_mph = us_units["ball_speed_mph"].as_f64().unwrap();
        assert!((ball_speed_mph - 120.0).abs() < 0.5);

        let carry_yards = us_units["carry_distance_yards"].as_f64().unwrap();
        let carry_meters = derived["carry_distance_meters"].as_f64().unwrap();
        assert!((carry_yards - carry_meters * 1.0936133).abs() < 1e-6);

        let club_speed_mph = us_units["club_speed_mph"].as_f64().unwrap();
        let club_speed_mps = derived["club_speed_meters_per_second"].as_f64().unwrap();
        assert!((club_speed_mph - club_speed_mps * 2.23693629).abs() < 1e-6);
    }

    #[test]
    fn test_shot_classification_matches_straight() {
        let json_input = r#"{
            "ball_speed_meters_per_second": 70.0,
            "vertical_launch_angle_degrees": 12.0,
            "horizontal_launch_angle_degrees": 0.0,
            "total_spin_rpm": 2500.0,
            "spin_axis_degrees": 0.0
        }"#;

        let result = calculate_derived_values(json_input).unwrap();
        let output: Value = serde_json::from_str(&result).unwrap();
        let derived = &output["open_golf_coach"];

        assert_eq!(derived["shot_name"], "Straight");
        assert_eq!(derived["shot_rank"], "B");
        assert_eq!(derived["shot_color_rgb"], "0x7CB342");
    }

    #[test]
    fn test_us_customary_input_conversion() {
        let json_input = r#"{
            "vertical_launch_angle_degrees": 12.0,
            "horizontal_launch_angle_degrees": 0.0,
            "total_spin_rpm": 2300.0,
            "spin_axis_degrees": 0.0,
            "us_customary_units": {
                "ball_speed_mph": 150.0,
                "carry_distance_yards": 240.0,
                "club_speed_mph": 100.0
            }
        }"#;

        let result = calculate_derived_values(json_input).unwrap();
        let output: Value = serde_json::from_str(&result).unwrap();
        let derived = &output["open_golf_coach"];

        let expected_carry_meters = yards_to_meters(240.0);
        assert!(
            (derived["carry_distance_meters"].as_f64().unwrap() - expected_carry_meters).abs()
                < 1e-6
        );

        let expected_club_speed = mph_to_meters_per_second(100.0);
        assert!(
            (derived["club_speed_meters_per_second"].as_f64().unwrap() - expected_club_speed).abs()
                < 1e-6
        );

        let us_units = &derived["us_customary_units"];
        assert!((us_units["carry_distance_yards"].as_f64().unwrap() - 240.0).abs() < 1e-6);
        assert!((us_units["ball_speed_mph"].as_f64().unwrap() - 150.0).abs() < 0.1);
    }
}
