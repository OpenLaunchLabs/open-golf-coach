use pyo3::prelude::*;

/// Calculate derived golf shot values from a JSON string
///
/// The library adds an "open_golf_coach" object with all derived values,
/// leaving the original input JSON unchanged.
///
/// Args:
///     json_input (str): JSON string containing golf shot parameters
///
/// Returns:
///     str: JSON string with original values plus "open_golf_coach" section
///
/// Example:
///     >>> import opengolfcoach
///     >>> import json
///     >>> shot = {
///     ...     "ball_speed_meters_per_second": 70.0,
///     ...     "vertical_launch_angle_degrees": 12.5,
///     ...     "total_spin_rpm": 2800.0,
///     ...     "spin_axis_degrees": 15.0
///     ... }
///     >>> result_json = opengolfcoach.calculate_derived_values(json.dumps(shot))
///     >>> result = json.loads(result_json)
///     >>> print(result["open_golf_coach"]["carry_distance_meters"])
#[pyfunction]
fn calculate_derived_values(json_input: &str) -> PyResult<String> {
    // Use the WASM/FFI function from the core library
    match ::opengolfcoach::calculate_derived_values(json_input) {
        Ok(result) => Ok(result),
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            format!("Calculation error: {:?}", e)
        )),
    }
}

/// OpenGolfCoach - Calculate derived golf shot values
///
/// This module provides functions to calculate derived golf metrics such as:
/// - Carry and offline distance  
/// - Backspin and sidespin from total spin and spin axis
/// - Total spin and spin axis from backspin and sidespin
/// - Landing position and velocity vectors
/// - Descent angle
#[pymodule]
fn opengolfcoach(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate_derived_values, m)?)?;
    Ok(())
}
