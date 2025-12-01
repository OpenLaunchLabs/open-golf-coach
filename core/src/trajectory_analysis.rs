use crate::trajectory::Trajectory;
use crate::vector::Vector3;
use std::f64::consts::PI;

const GRAVITY: f64 = 9.81; // m/s²
const FAIRWAY_FRICTION_COEFF: f64 = 0.18; // rolling resistance for a typical fairway
const ROLL_EFFICIENCY: f64 = 0.85; // accounts for bounce/energy lost to deformation
const ROLL_SCALING_COEFF: f64 = 0.15;

/// Get landing position from trajectory
/// Returns the final position in the trajectory (where ball lands)
pub fn get_landing_position(trajectory: &Trajectory) -> Vector3 {
    trajectory
        .points
        .last()
        .map(|p| p.position())
        .unwrap_or(Vector3::new(f64::NAN, f64::NAN, f64::NAN))
}

/// Get landing velocity from trajectory
/// Returns the final velocity in the trajectory (velocity at landing)
pub fn get_landing_velocity(trajectory: &Trajectory) -> Vector3 {
    trajectory
        .points
        .last()
        .map(|p| p.velocity())
        .unwrap_or(Vector3::new(f64::NAN, f64::NAN, f64::NAN))
}

/// Get hang time from trajectory
/// Returns the total flight time in seconds
pub fn get_hang_time(trajectory: &Trajectory) -> f64 {
    trajectory.points.last().map(|p| p.t).unwrap_or(f64::NAN)
}

/// Get apex (highest point) from trajectory
/// Returns the position of the highest point in the trajectory
pub fn get_apex_position(trajectory: &Trajectory) -> Vector3 {
    trajectory
        .points
        .iter()
        .max_by(|a, b| a.z.partial_cmp(&b.z).unwrap())
        .map(|p| p.position())
        .unwrap_or(Vector3::new(f64::NAN, f64::NAN, f64::NAN))
}

/// Get time to apex from trajectory
/// Returns the time at which the ball reaches its highest point
pub fn get_time_to_apex(trajectory: &Trajectory) -> f64 {
    trajectory
        .points
        .iter()
        .max_by(|a, b| a.z.partial_cmp(&b.z).unwrap())
        .map(|p| p.t)
        .unwrap_or(f64::NAN)
}

/// Get peak height from trajectory
/// Returns the maximum height (Z coordinate) reached during flight
pub fn get_peak_height(trajectory: &Trajectory) -> f64 {
    trajectory
        .points
        .iter()
        .map(|p| p.z)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(f64::NAN)
}

/// Get descent angle from trajectory
/// Returns the angle between landing velocity and horizontal plane in degrees
pub fn get_descent_angle(trajectory: &Trajectory) -> f64 {
    let landing_vel = get_landing_velocity(trajectory);
    let horizontal_speed = (landing_vel.x.powi(2) + landing_vel.y.powi(2)).sqrt();
    let descent_angle_rad = (-landing_vel.z).atan2(horizontal_speed);
    descent_angle_rad * 180.0 / PI
}

/// Get carry distance from trajectory
/// Returns the total distance traveled (magnitude of landing position)
pub fn get_carry_distance(trajectory: &Trajectory) -> f64 {
    let landing_pos = get_landing_position(trajectory);
    landing_pos.magnitude()
}

/// Get offline distance from trajectory
/// Returns the lateral (Y-axis) distance from centerline
pub fn get_offline_distance(trajectory: &Trajectory) -> f64 {
    let landing_pos = get_landing_position(trajectory);
    landing_pos.y
}

/// Estimate run-out after landing and return total distance (carry + roll)
pub fn get_total_distance(trajectory: &Trajectory) -> f64 {
    let carry = get_carry_distance(trajectory);
    let landing_pos = get_landing_position(trajectory);
    let landing_vel = get_landing_velocity(trajectory);

    // Horizontal speed drives roll potential.
    let horizontal_speed = (landing_vel.x.powi(2) + landing_vel.y.powi(2)).sqrt();
    if horizontal_speed <= 0.1 {
        return carry;
    }

    // Shallow descent angles roll more; steep wedge shots stop quickly.
    let descent_angle = get_descent_angle(trajectory).clamp(0.0, 90.0);
    let descent_factor = ((90.0 - descent_angle) / 90.0).powf(1.4);

    // Constant-deceleration model due to rolling resistance: d = v^2 / (2 * μ * g)
    let base_roll = horizontal_speed.powi(2) / (2.0 * FAIRWAY_FRICTION_COEFF * GRAVITY);
    let mut roll_distance = base_roll * descent_factor * ROLL_EFFICIENCY * ROLL_SCALING_COEFF;
    roll_distance = roll_distance.max(0.0);

    // Roll follows the down-range heading inferred from horizontal landing velocity,
    // or fall back to carry direction if velocity is ill-defined.
    let mut heading = Vector3::new(landing_vel.x, landing_vel.y, 0.0);
    if heading.magnitude() <= 0.01 {
        heading = Vector3::new(landing_pos.x, landing_pos.y, 0.0);
    }
    let heading = heading.normalize();

    let roll_vector = Vector3::new(heading.x * roll_distance, heading.y * roll_distance, 0.0);
    let total_vector = landing_pos.add(&roll_vector);
    total_vector.magnitude()
}
