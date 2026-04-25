use crate::trajectory::{Trajectory, TrajectoryPoint};
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

/// Down-sample a native-rate `Trajectory` to `target_hz` by linearly
/// interpolating each emitted point between the bracketing native steps.
///
/// The first emitted point is at `t = 0`, subsequent points fall on
/// `k / target_hz` for `k = 1, 2, …` up to the last native time, and the
/// final native point (the landing frame) is always preserved so the landing
/// isn't aliased away. If `target_hz` is non-finite, non-positive, or at
/// least the native rate (500 Hz), the native points are returned unchanged.
pub fn down_sample_trajectory(
    trajectory: &Trajectory,
    target_hz: f64,
) -> Vec<TrajectoryPoint> {
    const NATIVE_RATE_HZ: f64 = 500.0;

    let native = &trajectory.points;
    if native.is_empty() {
        return Vec::new();
    }
    if !target_hz.is_finite() || target_hz <= 0.0 || target_hz >= NATIVE_RATE_HZ {
        return native.clone();
    }

    let dt = 1.0 / target_hz;
    let last_t = native.last().unwrap().t;
    let mut out: Vec<TrajectoryPoint> = Vec::with_capacity(((last_t * target_hz) as usize) + 2);
    let mut cursor = 0usize;

    let mut k = 0u64;
    loop {
        let sample_t = (k as f64) * dt;
        if sample_t > last_t {
            break;
        }
        // Advance cursor so that native[cursor].t <= sample_t < native[cursor+1].t,
        // or cursor == native.len() - 1 at the end.
        while cursor + 1 < native.len() && native[cursor + 1].t <= sample_t {
            cursor += 1;
        }
        if cursor + 1 >= native.len() {
            out.push(native[cursor]);
        } else {
            let a = &native[cursor];
            let b = &native[cursor + 1];
            let span = b.t - a.t;
            let frac = if span > 0.0 {
                ((sample_t - a.t) / span).clamp(0.0, 1.0)
            } else {
                0.0
            };
            out.push(lerp_point(a, b, frac, sample_t));
        }
        k += 1;
    }

    // Always preserve the final native point (the landing frame). If the last
    // sampled time fell short of `last_t`, append it; if it landed exactly on
    // the last sample (rare but possible), replace it with the canonical
    // landing point so x/y/z/v* match the integrator's final state.
    let last_native = *native.last().unwrap();
    match out.last() {
        Some(p) if (p.t - last_native.t).abs() < 1e-9 => {
            *out.last_mut().unwrap() = last_native;
        }
        _ => out.push(last_native),
    }

    out
}

fn lerp_point(a: &TrajectoryPoint, b: &TrajectoryPoint, frac: f64, t: f64) -> TrajectoryPoint {
    let lerp = |x: f64, y: f64| x + (y - x) * frac;
    TrajectoryPoint {
        x: lerp(a.x, b.x),
        y: lerp(a.y, b.y),
        z: lerp(a.z, b.z),
        vx: lerp(a.vx, b.vx),
        vy: lerp(a.vy, b.vy),
        vz: lerp(a.vz, b.vz),
        t,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn point(t: f64, x: f64, z: f64) -> TrajectoryPoint {
        TrajectoryPoint {
            x,
            y: 0.0,
            z,
            vx: 1.0,
            vy: 0.0,
            vz: -1.0,
            t,
        }
    }

    #[test]
    fn down_sample_interpolates_between_native_steps() {
        // Native trajectory at 500 Hz (dt=0.002), with x linear in t for a
        // simple invariant to check.
        let mut points = Vec::new();
        for k in 0..=500 {
            let t = k as f64 * 0.002;
            points.push(point(t, t * 100.0, 1.0 - t));
        }
        let trajectory = Trajectory { points };

        let down = down_sample_trajectory(&trajectory, 60.0);
        // Roughly hang_time * 60 + landing point
        assert!(down.len() >= 60);
        // First sample at t=0
        assert!((down[0].t - 0.0).abs() < 1e-12);
        // Interior samples spaced 1/60
        for w in down.windows(2).take(down.len() - 2) {
            assert!((w[1].t - w[0].t - 1.0 / 60.0).abs() < 1e-9);
        }
        // Linear x = 100 * t check on an interior sample
        let mid = &down[10];
        assert!((mid.x - mid.t * 100.0).abs() < 1e-9);
        // Final point preserves the integrator's last frame exactly.
        let last_native = trajectory.points.last().unwrap();
        let last_out = down.last().unwrap();
        assert_eq!(last_out.t, last_native.t);
        assert_eq!(last_out.x, last_native.x);
    }

    #[test]
    fn down_sample_returns_native_when_rate_at_or_above_native() {
        let mut points = Vec::new();
        for k in 0..=10 {
            let t = k as f64 * 0.002;
            points.push(point(t, t, 0.0));
        }
        let trajectory = Trajectory {
            points: points.clone(),
        };
        for rate in [500.0_f64, 1000.0_f64, f64::INFINITY] {
            let out = down_sample_trajectory(&trajectory, rate);
            assert_eq!(out.len(), points.len(), "rate {} should pass through", rate);
        }
    }

    #[test]
    fn down_sample_handles_non_positive_rate_as_passthrough() {
        let trajectory = Trajectory {
            points: vec![point(0.0, 0.0, 0.0), point(0.002, 1.0, 0.0)],
        };
        for rate in [0.0_f64, -10.0_f64, f64::NAN] {
            let out = down_sample_trajectory(&trajectory, rate);
            assert_eq!(out.len(), 2, "rate {} should pass through", rate);
        }
    }

    #[test]
    fn down_sample_empty_trajectory_returns_empty() {
        let trajectory = Trajectory { points: vec![] };
        assert!(down_sample_trajectory(&trajectory, 60.0).is_empty());
    }
}
