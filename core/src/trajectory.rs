use crate::Vector3;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// A single point in the golf ball trajectory
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TrajectoryPoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
    pub t: f64, // Time since start of flight in seconds
}

impl TrajectoryPoint {
    fn new(position: Vector3, velocity: Vector3, time: f64) -> Self {
        TrajectoryPoint {
            x: position.x,
            y: position.y,
            z: position.z,
            vx: velocity.x,
            vy: velocity.y,
            vz: velocity.z,
            t: time,
        }
    }

    /// Get position as a Vector3
    pub fn position(&self) -> Vector3 {
        Vector3::new(self.x, self.y, self.z)
    }

    /// Get velocity as a Vector3
    pub fn velocity(&self) -> Vector3 {
        Vector3::new(self.vx, self.vy, self.vz)
    }
}

/// Ball trajectory data - sequence of trajectory points
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trajectory {
    pub points: Vec<TrajectoryPoint>,
}

impl Trajectory {
    fn new() -> Self {
        Trajectory { points: Vec::new() }
    }
}

/// Physics constants and ball properties
const DELTA_TIME: f64 = 1.0 / 500.0; // 500 Hz update rate
const BALL_DIAMETER: f64 = 0.04267; // meters
const BALL_MASS: f64 = 0.04593; // kg
const GRAVITY: f64 = 9.81; // m/s²

/// Kinematic viscosity of air at given temperature
fn kinematic_viscosity_of_air(temp_c: f64) -> f64 {
    // Sutherland's law approximation
    let t = temp_c + 273.15;
    1.458e-6 * t.powf(1.5) / (t + 110.4)
}

/// Returns approximate static air pressure [hPa] given elevation [m].
fn pressure_hpa_at_elevation(elevation_m: f64) -> f64 {
    // Constants for troposphere (up to ~11 km)
    let p0 = 1013.25; // sea-level standard pressure (hPa)
    let t0 = 288.15; // sea-level standard temperature (K)
    let g = 9.80665; // gravity (m/s²)
    let l = 0.0065; // temperature lapse rate (K/m)
    let r = 8.3144598; // universal gas constant (J/mol/K)
    let m = 0.0289644; // molar mass of Earth's air (kg/mol)

    p0 * (1.0 - (l * elevation_m) / t0).powf((g * m) / (r * l))
}

/// Air density as a function of elevation
/// Based on barometric formula
fn air_density_humid(p_hpa: f64, temp_c: f64, rel_humidity: f64) -> f64 {
    let t_k = temp_c + 273.15;

    // saturation vapor pressure (Tetens formula)
    let e_s = 6.112 * f64::exp((17.67 * temp_c) / (temp_c + 243.5));
    let e = rel_humidity * e_s; // hPa
    let p_dry = (p_hpa - e) * 100.0;
    let p_vapor = e * 100.0;

    // gas constants
    let r_dry = 287.058;
    let r_vapor = 461.495;

    (p_dry / (r_dry * t_k)) + (p_vapor / (r_vapor * t_k))
}

/// Calculate lift coefficient based on spin number
/// Reference: https://www.seas.upenn.edu/~meam211/slides/aero.pdf
/// and https://www.mdpi.com/2504-3900/2/6/238/pdf
fn lift_coefficient(omega: f64, speed: f64) -> f64 {
    let spin_number = omega * BALL_DIAMETER / (2.0 * speed);

    if spin_number > 0.306153 {
        (0.33 - 0.23) * (spin_number - 0.20) / (0.40 - 0.20) + 0.23
    } else {
        -3.25 * spin_number.powi(2) + 1.99 * spin_number
    }
}

/// Calculate drag coefficient based on Reynolds number and spin
/// Reference: https://www.seas.upenn.edu/~meam211/slides/aero.pdf
/// and https://www.mdpi.com/2504-3900/2/6/238/pdf
fn drag_coefficient(omega: f64, speed: f64, temp_c: f64) -> f64 {
    let spin_number = omega * BALL_DIAMETER / (2.0 * speed);
    let reynolds = speed * BALL_DIAMETER / kinematic_viscosity_of_air(temp_c);

    // Calculate spin modifier from spin number
    let mut spin_modifier = -0.255;
    if spin_number < 0.15 {
        spin_modifier += (0.28 - 0.255) * (spin_number - 0.00) / (0.15 - 0.00) + 0.255;
    } else if spin_number < 0.25 {
        spin_modifier += (0.33 - 0.28) * (spin_number - 0.15) / (0.25 - 0.15) + 0.28;
    } else if spin_number <= 0.35 {
        spin_modifier += (0.355 - 0.33) * (spin_number - 0.25) / (0.35 - 0.25) + 0.33;
    } else {
        spin_modifier += (0.38 - 0.355) * (spin_number - 0.35) / (0.45 - 0.35) + 0.355;
    }

    // Calculate drag based on Reynolds number
    if reynolds < 38000.0 {
        0.50 + spin_modifier
    } else if reynolds < 45000.0 {
        (0.35 - 0.48) * (reynolds - 38000.0) / (45000.0 - 38000.0) + 0.48 + spin_modifier
    } else if reynolds < 50000.0 {
        (0.30 - 0.35) * (reynolds - 45000.0) / (50000.0 - 45000.0) + 0.35 + spin_modifier
    } else if reynolds < 60000.0 {
        (0.24 + 0.8 * spin_modifier - 0.30 + spin_modifier) * (reynolds - 50000.0)
            / (60000.0 - 50000.0)
            + 0.30
            + spin_modifier
    } else if reynolds < 240000.0 {
        (0.26 - 0.24 + 0.8 * spin_modifier) * (reynolds - 60000.0) / (240000.0 - 60000.0)
            + 0.24
            + 0.8 * spin_modifier
    } else if reynolds <= 4000000.0 {
        (0.30 - 0.26) * (reynolds - 240000.0) / (4000000.0 - 240000.0) + 0.26
    } else {
        (0.30 - 0.26) * (reynolds - 240000.0) / (4000000.0 - 240000.0) + 0.26
    }
}

/// Calculate full ball trajectory using numerical integration
///
/// Coordinate system: Unreal LEFT HANDED
/// - X is forward toward target
/// - Y is right (positive = right) positive sidespin and spin axis is right (fade/slice)
/// - Z is up
///
/// # Arguments
/// * `ball_speed_mps` - Ball speed in meters per second
/// * `v_launch_deg` - Vertical launch angle in degrees
/// * `h_launch_deg` - Horizontal launch angle in degrees (0 = straight, negative = left)
/// * `backspin_rpm` - Backspin rate in RPM
/// * `sidespin_rpm` - Sidespin rate in RPM (positive = hook/right)
/// * `elevation_m` - Altitude in meters above sea level
/// * `temperature_k` - Temperature in Kelvin
/// * `humidity_percent` - Relative humidity as percentage (0-100)
/// * `pressure_pa` - Atmospheric pressure in Pascals (optional, calculated from elevation if None)
///
/// # Returns
/// Trajectory containing sequences of positions and velocities
pub fn calculate_trajectory(
    ball_speed_mps: f64,
    v_launch_deg: f64,
    h_launch_deg: f64,
    backspin_rpm: f64,
    sidespin_rpm: f64,
    elevation_m: f64,
    temperature_k: f64,
    humidity_percent: f64,
    pressure_pa: Option<f64>,
) -> Trajectory {
    // Convert launch angles to radians
    let v_launch_rad = v_launch_deg * PI / 180.0;
    let h_launch_rad = h_launch_deg * PI / 180.0;

    // Convert RPM to rad/s: RPM * (2π / 60) = RPM * 0.10472
    let backspin_rad_s = backspin_rpm * 0.10472;
    let sidespin_rad_s = sidespin_rpm * 0.10472;
    let total_spin_rad_s = (backspin_rad_s.powi(2) + sidespin_rad_s.powi(2)).sqrt();
    let spin_axis = sidespin_rad_s.atan2(backspin_rad_s);

    // Create trajectory to store results
    let mut trajectory = Trajectory::new();

    // Initial position (on ground at origin)
    let mut position = Vector3::new(0.0, 0.0, 0.0);

    // Initial velocity in Unreal coordinates
    // X component is forward
    // Y component is right (positive = right) positive sidespin and spin axis is right (fade/slice)
    // Z component is up
    let v_horizontal = ball_speed_mps * v_launch_rad.cos();
    let mut velocity = Vector3::new(
        v_horizontal * h_launch_rad.cos(),   // X: backward/forward
        v_horizontal * h_launch_rad.sin(),   // Y: left/right
        ball_speed_mps * v_launch_rad.sin(), // Z: down/up
    );

    // Store initial state
    let mut time = 0.0;
    trajectory
        .points
        .push(TrajectoryPoint::new(position, velocity, time));

    let mut total_spin = total_spin_rad_s;

    // Spin decay rate
    // Fraction per second, exponential decay
    const SPIN_DECAY_RATE: f64 = 0.04; // 4% per second

    // Simulate until ball hits ground (z <= 0) or is falling (vz < 0)
    // Need at least one iteration to start
    let mut iteration = 0;
    const MAX_ITERATIONS: i32 = 10000; // Safety limit

    // Calculate weather and other constants
    // Use provided pressure or calculate from elevation
    let p_hpa = if let Some(pressure_pascals) = pressure_pa {
        pressure_pascals / 100.0 // Convert Pa to hPa
    } else {
        pressure_hpa_at_elevation(elevation_m)
    };

    // Convert temperature from Kelvin to Celsius for air density calculation
    let temperature_c = temperature_k - 273.15;

    // Convert humidity from percentage to fraction (0-1)
    let humidity_fraction = humidity_percent / 100.0;

    let air_density = air_density_humid(p_hpa, temperature_c, humidity_fraction);
    let cross_sectional_area = PI * (BALL_DIAMETER / 2.0).powi(2);

    while (position.z >= 0.0) && iteration < MAX_ITERATIONS {
        // Get current ball speed
        let current_speed = velocity.magnitude();

        // Calculate aerodynamic coefficients
        let lift_coeff = lift_coefficient(total_spin, current_speed);
        let drag_coeff = drag_coefficient(total_spin, current_speed, temperature_c);

        // Calculate forces
        let dynamic_pressure = 0.5 * air_density * cross_sectional_area * current_speed.powi(2);
        let drag_force_mag = dynamic_pressure * drag_coeff;

        // Force components using spin axis and current direction
        // Lift acts perpendicular to velocity, in the plane defined by spin axis
        // Drag acts opposite to velocity

        // Scale drag force to velocity components and negate
        // This is already in world coordinates
        let vector_drag_force = Vector3::new(
            -drag_force_mag * (velocity.x / current_speed),
            -drag_force_mag * (velocity.y / current_speed),
            -drag_force_mag * (velocity.z / current_speed),
        );

        // Calculate lift
        // Unit vectors
        let v_hat = velocity.normalize();

        // Define spin components in ball body frame
        let spin_axis_vec = Vector3::new(
            0.0,
            -spin_axis.cos(), // Backspin is in -Y direction
            spin_axis.sin(),  // Sidespin is in +Z direction
        )
        .normalize();

        // Lift direction = cross(spin_axis, v_hat)
        let lift_dir = spin_axis_vec.cross(&v_hat).normalize();

        // Lift force magnitude
        let lift_force_mag =
            0.5 * air_density * current_speed.powi(2) * cross_sectional_area * lift_coeff;

        // Lift vector
        let vector_lift_force = Vector3::new(
            lift_dir.x * lift_force_mag,
            lift_dir.y * lift_force_mag,
            lift_dir.z * lift_force_mag,
        );

        let total_force = vector_drag_force.add(&vector_lift_force);

        // Calculate acceleration (F = ma)
        let accel = Vector3::new(
            total_force.x / BALL_MASS,
            total_force.y / BALL_MASS,
            total_force.z / BALL_MASS - GRAVITY, // Include gravity
        );

        // Update velocity (v = u + at)
        let new_velocity = Vector3::new(
            velocity.x + accel.x * DELTA_TIME,
            velocity.y + accel.y * DELTA_TIME,
            velocity.z + accel.z * DELTA_TIME,
        );

        // Update position using average velocity over timestep
        // This can result in more stable numerical integration
        let avg_velocity = Vector3::new(
            (velocity.x + new_velocity.x) / 2.0,
            (velocity.y + new_velocity.y) / 2.0,
            (velocity.z + new_velocity.z) / 2.0,
        );

        position.x += avg_velocity.x * DELTA_TIME;
        position.y += avg_velocity.y * DELTA_TIME;
        position.z += avg_velocity.z * DELTA_TIME;

        velocity = new_velocity;

        // Apply spin decay
        total_spin *= (-SPIN_DECAY_RATE * DELTA_TIME).exp();

        // Update time
        time += DELTA_TIME;

        // Store state at each timestep
        trajectory
            .points
            .push(TrajectoryPoint::new(position, velocity, time));

        iteration += 1;
    }

    if iteration >= MAX_ITERATIONS {
        // Safety fallback: clear trajectory and add NaN values
        trajectory.points.clear();
        trajectory.points.push(TrajectoryPoint::new(
            Vector3::new(f64::NAN, f64::NAN, f64::NAN),
            Vector3::new(f64::NAN, f64::NAN, f64::NAN),
            0.0,
        ));
    }

    trajectory
}
