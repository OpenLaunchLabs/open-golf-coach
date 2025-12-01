/// Physics constants for clubhead speed estimation
const BALL_MASS: f64 = 0.04593; // kg (golf ball)
const CLUBHEAD_MASS: f64 = 0.200; // kg (~200g, typical driver head)
const DRIVER_COR_LIMIT: f64 = 0.83; // USGA/R&A limit for coefficient of restitution
const MIN_EFFECTIVE_COR: f64 = 0.52; // Represents a glancing or highly inefficient strike

#[derive(Clone, Copy)]
struct ImpactBand {
    max_ball_speed_mps: f64,
    base_cor: f64,
    optimal_launch_deg: f64,
    launch_tolerance_deg: f64,
    optimal_spin_rpm: f64,
    spin_tolerance_rpm: f64,
    face_influence_ratio: f64,
    spin_axis_gain: f64,
}

const IMPACT_BANDS: [ImpactBand; 4] = [
    // Wedges and short chips
    ImpactBand {
        max_ball_speed_mps: 40.0, // ~90 mph
        base_cor: 0.55,
        optimal_launch_deg: 28.0,
        launch_tolerance_deg: 15.0,
        optimal_spin_rpm: 9000.0,
        spin_tolerance_rpm: 4000.0,
        face_influence_ratio: 0.65,
        spin_axis_gain: 1.7,
    },
    // Short and mid irons
    ImpactBand {
        max_ball_speed_mps: 50.0, // ~112 mph
        base_cor: 0.66,
        optimal_launch_deg: 20.0,
        launch_tolerance_deg: 12.0,
        optimal_spin_rpm: 7000.0,
        spin_tolerance_rpm: 2500.0,
        face_influence_ratio: 0.72,
        spin_axis_gain: 2.1,
    },
    // Long irons and hybrids
    ImpactBand {
        max_ball_speed_mps: 60.0, // ~134 mph
        base_cor: 0.72,
        optimal_launch_deg: 16.0,
        launch_tolerance_deg: 10.0,
        optimal_spin_rpm: 5000.0,
        spin_tolerance_rpm: 2000.0,
        face_influence_ratio: 0.78,
        spin_axis_gain: 2.4,
    },
    // Fairway woods and drivers
    ImpactBand {
        max_ball_speed_mps: f64::INFINITY,
        base_cor: DRIVER_COR_LIMIT,
        optimal_launch_deg: 12.0,
        launch_tolerance_deg: 8.0,
        optimal_spin_rpm: 2500.0,
        spin_tolerance_rpm: 1500.0,
        face_influence_ratio: 0.85,
        spin_axis_gain: 2.8,
    },
];

fn band_for_ball_speed(ball_speed_mps: f64) -> ImpactBand {
    for band in IMPACT_BANDS {
        if ball_speed_mps <= band.max_ball_speed_mps {
            return band;
        }
    }
    IMPACT_BANDS[IMPACT_BANDS.len() - 1]
}

/// Estimated face/path relationship for a given shot
pub struct ClubFacePathEstimates {
    pub club_path_degrees: f64,
    pub club_face_to_target_degrees: f64,
    pub club_face_to_path_degrees: f64,
}

/// Estimate club path/face parameters using simplified D-plane assumptions.
pub fn estimate_club_face_path(
    ball_speed_mps: f64,
    horizontal_launch_angle_deg: f64,
    spin_axis_degrees: f64,
) -> ClubFacePathEstimates {
    let band = band_for_ball_speed(ball_speed_mps.max(5.0));
    let face_to_path = spin_axis_degrees / band.spin_axis_gain;

    let club_path = horizontal_launch_angle_deg - band.face_influence_ratio * face_to_path;
    let club_face = club_path + face_to_path;

    ClubFacePathEstimates {
        club_path_degrees: club_path,
        club_face_to_target_degrees: club_face,
        club_face_to_path_degrees: face_to_path,
    }
}

/// Calculate smash factor (ball speed / clubhead speed ratio)
///
/// Smash factor is a measure of energy transfer efficiency from club to ball.
/// Higher values indicate more efficient strikes.
/// - Driver: typically 1.45-1.50 for good strikes
/// - Irons: typically 1.35-1.40
///
/// # Arguments
/// * `ball_speed_mps` - Ball speed in meters per second
/// * `clubhead_speed_mps` - Clubhead speed in meters per second
///
/// # Returns
/// Smash factor (unitless ratio)
pub fn get_smash_factor(ball_speed_mps: f64, clubhead_speed_mps: f64) -> f64 {
    if clubhead_speed_mps == 0.0 {
        return f64::NAN;
    }
    ball_speed_mps / clubhead_speed_mps
}

/// Estimate clubhead speed from ball launch conditions
///
/// Uses a collision-based model capped by the USGA COR limit to estimate the
/// clubhead speed that would produce the observed ball speed. Takes into account:
/// - Mass ratio between clubhead and ball
/// - Piecewise effective COR derived from typical club categories (wedge → driver)
/// - Spin-rate dependent adjustments within each speed band
/// - Launch-angle penalties representing spin-loft inefficiencies
///
/// # Arguments
/// * `ball_speed_mps` - Ball speed in meters per second
/// * `vertical_launch_angle_deg` - Vertical launch angle in degrees
/// * `total_spin_rpm` - Total spin rate in RPM
///
/// # Returns
/// Estimated clubhead speed in meters per second
pub fn estimate_clubhead_speed(
    ball_speed_mps: f64,
    vertical_launch_angle_deg: f64,
    total_spin_rpm: f64,
) -> f64 {
    // Clamp user inputs to reasonable on-course ranges to avoid runaway penalties.
    let launch_angle = vertical_launch_angle_deg.clamp(-5.0, 70.0);
    let spin_rpm = total_spin_rpm.max(0.0);
    let band = band_for_ball_speed(ball_speed_mps.max(5.0));

    // Launch penalty relative to the band-specific optimal.
    let launch_deviation = (launch_angle - band.optimal_launch_deg).abs();
    let normalized_launch = (launch_deviation / band.launch_tolerance_deg).min(3.0);
    let launch_penalty = normalized_launch.powf(1.25) * 0.06;

    // Spin penalty inside the band (covers both over- and under-spin scenarios).
    let spin_tolerance = band.spin_tolerance_rpm.max(1.0);
    let normalized_spin = if spin_rpm >= band.optimal_spin_rpm {
        ((spin_rpm - band.optimal_spin_rpm) / spin_tolerance).min(3.0)
    } else {
        ((band.optimal_spin_rpm - spin_rpm) / (spin_tolerance * 1.5)).min(3.0)
    };
    let spin_penalty = normalized_spin.powf(1.15) * 0.08;

    // Extra penalty for extreme knuckleballs where almost no spin is generated.
    let knuckle_penalty = if spin_rpm < 1200.0 {
        ((1200.0 - spin_rpm) / 1200.0).powf(1.3) * 0.05
    } else {
        0.0
    };

    let mut effective_cor = band.base_cor - launch_penalty - spin_penalty - knuckle_penalty;
    effective_cor = effective_cor.clamp(MIN_EFFECTIVE_COR, DRIVER_COR_LIMIT);

    // Convert the effective COR into a smash factor via a simple collision model.
    let mass_ratio = BALL_MASS / CLUBHEAD_MASS;
    let smash_factor = (1.0 + effective_cor) / (1.0 + mass_ratio);

    ball_speed_mps / smash_factor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clubhead_speed_estimation_driver() {
        // Typical driver shot: 160 mph ball speed (~71.5 m/s)
        // Collision-based model should estimate clubhead speed around 105-112 mph (~47-50 m/s)
        let ball_speed = 71.5;
        let launch_angle = 11.5;
        let spin = 2500.0;

        let club_speed = estimate_clubhead_speed(ball_speed, launch_angle, spin);

        // Should be in realistic range for driver (105-112 mph = 47-50 m/s)
        assert!(
            club_speed > 45.0 && club_speed < 50.0,
            "Clubhead speed {} m/s ({} mph) should be reasonable for driver",
            club_speed,
            club_speed * 2.23694
        );

        // Ball speed should be higher than clubhead speed
        assert!(
            ball_speed > club_speed,
            "Ball speed should exceed clubhead speed"
        );
    }

    #[test]
    fn test_clubhead_speed_estimation_iron() {
        // Typical 7-iron: 120 mph ball speed (~53.6 m/s)
        // Model estimates clubhead speed around 85-95 mph (~38-42 m/s)
        let ball_speed = 53.6;
        let launch_angle = 16.0;
        let spin = 7000.0;

        let club_speed = estimate_clubhead_speed(ball_speed, launch_angle, spin);

        // Should be in realistic range for 7-iron (85-95 mph = 38-42 m/s)
        assert!(
            club_speed > 36.0 && club_speed < 43.0,
            "Clubhead speed {} m/s ({} mph) should be reasonable for iron",
            club_speed,
            club_speed * 2.23694
        );
    }

    #[test]
    fn test_clubhead_speed_estimation_wedge() {
        // Typical pitching wedge: 90 mph ball speed (~40 m/s)
        // Expect smash factor near 1.25 resulting in 72-78 mph club speed (~32-35 m/s)
        let ball_speed = 40.0;
        let launch_angle = 32.0;
        let spin = 9000.0;

        let club_speed = estimate_clubhead_speed(ball_speed, launch_angle, spin);

        assert!(
            club_speed > 30.0 && club_speed < 35.5,
            "Clubhead speed {} m/s ({} mph) should be reasonable for wedge",
            club_speed,
            club_speed * 2.23694
        );
    }

    #[test]
    fn test_spin_effect_on_clubhead_estimate() {
        // Deviating from the expected spin (too low or too high) should
        // reduce smash factor and therefore increase the required club speed.
        let ball_speed = 60.0;
        let launch_angle = 12.0;

        let optimal_spin_club_speed = estimate_clubhead_speed(ball_speed, launch_angle, 5000.0);
        let low_spin_club_speed = estimate_clubhead_speed(ball_speed, launch_angle, 2000.0);
        let high_spin_club_speed = estimate_clubhead_speed(ball_speed, launch_angle, 8000.0);

        assert!(
            low_spin_club_speed > optimal_spin_club_speed,
            "Low spin should require higher club speed than optimal spin: {} vs {}",
            low_spin_club_speed,
            optimal_spin_club_speed
        );
        assert!(
            high_spin_club_speed > optimal_spin_club_speed,
            "High spin should require higher club speed than optimal spin: {} vs {}",
            high_spin_club_speed,
            optimal_spin_club_speed
        );
    }

    #[test]
    fn test_face_path_estimation_driver_cut() {
        let estimates = estimate_club_face_path(70.0, -2.0, 15.0);
        assert!(
            estimates.club_face_to_path_degrees > 4.0 && estimates.club_face_to_path_degrees < 8.0
        );
        assert!(
            estimates.club_face_to_target_degrees > -2.0
                && estimates.club_face_to_target_degrees < 1.0
        );
        assert!(estimates.club_path_degrees < -4.5);
    }

    #[test]
    fn test_face_path_estimation_wedge() {
        let estimates = estimate_club_face_path(35.0, 3.0, -6.0);
        assert!(estimates.club_face_to_path_degrees < -2.0);
        assert!(
            estimates.club_face_to_target_degrees > 1.0
                && estimates.club_face_to_target_degrees < 5.0
        );
        assert!(estimates.club_path_degrees > 2.0);
    }

    #[test]
    fn test_smash_factor_driver() {
        // Typical driver: 160 mph ball / 107 mph club = 1.50 smash factor
        let ball_speed = 71.5; // 160 mph
        let club_speed = 47.8; // 107 mph
        let smash = get_smash_factor(ball_speed, club_speed);

        assert!((smash - 1.50).abs() < 0.01, "Smash factor should be ~1.50");
    }

    #[test]
    fn test_smash_factor_iron() {
        // Typical 7-iron: 120 mph ball / 87 mph club = 1.38 smash factor
        let ball_speed = 53.6; // 120 mph
        let club_speed = 38.9; // 87 mph
        let smash = get_smash_factor(ball_speed, club_speed);

        assert!((smash - 1.38).abs() < 0.01, "Smash factor should be ~1.38");
    }

    #[test]
    fn test_smash_factor_zero_clubhead_speed() {
        let smash = get_smash_factor(70.0, 0.0);
        assert!(
            smash.is_nan(),
            "Smash factor should be NaN for zero clubhead speed"
        );
    }
}
