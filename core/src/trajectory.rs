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

/// Dynamic viscosity of air at given temperature using Sutherland's law [Pa·s]
fn dynamic_viscosity_of_air(temp_c: f64) -> f64 {
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

/// Calculate lift coefficient from spin parameter S = ω·R/V.
///
/// For S <= 0.25 uses the Lyu et al. 2018 quadratic fit (averaged across 13
/// production balls).
/// Bearman & Harvey (1976) and subsequent studies show CL continues to increase
/// with spin at supercritical Re. Create a linear extension beyond 0.25,
/// giving C1 continuity and a physically reasonable slope beyond Lyu.
/// * Empirical Source (S <= 0.25): Lyu et al. 2018 (13-ball average quadratic fit)
/// * Empirical Source (S > 0.25 linear slope matching): Bearman & Harvey (1976)
fn lift_coefficient(spin_param: f64) -> f64 {
    const S_TRANSITION: f64 = 0.25;
    const CL_TRANSITION: f64 = 0.2941; // 1.99 * 0.25 - 3.255 * 0.25^2
    const SLOPE_TRANSITION: f64 = 0.3625; // d/dS(1.99*S - 3.255*S^2) at S = 0.25

    if spin_param <= S_TRANSITION {
        1.99 * spin_param - 3.255 * spin_param * spin_param
    } else {
        CL_TRANSITION + SLOPE_TRANSITION * (spin_param - S_TRANSITION)
    }
}

/// Drag coefficient modeled as a generalized smooth sigmoid function.
///
/// This generalized model uses a standard logistic (sigmoid) curve to transition
/// seamlessly through the drag crisis between the subcritical and supercritical regimes,
/// which is widely accepted in CFD (e.g., Smits & Smith, 1994).
///
/// The typical values that are largely ball dependent are the subcritical and supercritcal
/// drag coefficients.  A sigmoid is a reasonable, smooth transition between these two values.
///
/// The base drag coefficient is then augmented by a smooth, monotonically increasing
/// spin modifier.
fn drag_coefficient(re: f64, spin_param: f64) -> f64 {
    // Sigmoid parameters for the drag crisis
    // * Empirical Source (CD bounds): Bearman & Harvey (1976) & Smits & Smith (1994)
    //   Both observe subcritical CD ~0.5 and supercritical CD ~0.21 - 0.25 for dimpled spheres.
    const CD_SUBCRIT: f64 = 0.5;      // Low speed drag coeff
    const CD_SUPERCRIT: f64 = 0.225;    // High speed drag coeff
    const RE_CRIT: f64 = 80_000.0;     // Center of the drag crisis
    const K: f64 = 0.0002;            // Steepness of the transition

    // 1. Calculate stationary (non-spinning) drag using the sigmoid function
    let stationary_drag = CD_SUPERCRIT + (CD_SUBCRIT - CD_SUPERCRIT) / (1.0 + f64::exp(K * (re - RE_CRIT)));

    // 2. Smooth spin modifier
    // Smits & Smith and empirical data typically model the spin penalty continuously.
    // The Magnus wake separation penalty plateaus at high spin (S > 0.4).
    // An inverted exponential captures the initial quadratic behavior (slope ~0.45)
    // while capping the maximum penalty at +0.18 to prevent unbounded drag on wedges.
    let spin_modifier = 0.18 * (1.0 - f64::exp(-3.0 * spin_param * spin_param));

    stationary_drag + spin_modifier
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

    // Aerodynamic spin decay constant (per meter of travel) based on the
    // aerodynamic skin-friction theory and empirical wind tunnel matching.
    // * Empirical Source: Smits and Smith (1994) experimentally measured a decay
    //   constant of c = 0.00002 for a slazenger ball, which resolves to an
    //   aerodynamic scalar of K = 0.00094 m^-1.  Modern balls are estimated to decay slighty faster.
    const K_SPIN_DECAY: f64 = 0.001;

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
    let kinematic_viscosity = dynamic_viscosity_of_air(temperature_c) / air_density;

    while (position.z >= 0.0) && iteration < MAX_ITERATIONS {
        // Get current ball speed
        let current_speed = velocity.magnitude();

        // Calculate aerodynamic coefficients
        let re = current_speed * BALL_DIAMETER / kinematic_viscosity;
        let spin_param = total_spin * (BALL_DIAMETER / 2.0) / current_speed;
        let lift_coeff = lift_coefficient(spin_param);
        let drag_coeff = drag_coefficient(re, spin_param);

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

        // Apply spin decay (Aerodynamic decay proportional to velocity)
        total_spin *= (-K_SPIN_DECAY * current_speed * DELTA_TIME).exp();

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
