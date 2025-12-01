/**
 * OpenGolfCoach - Node.js Bindings
 *
 * This module provides a simple interface to calculate derived golf shot values
 * from basic shot parameters.
 */

const wasm = require('./wasm/opengolfcoach.js');

/**
 * Calculate derived golf shot values
 *
 * @param {Object} shotData - Golf shot parameters
 * @param {number} shotData.ball_speed_meters_per_second - Ball speed in m/s
 * @param {number} shotData.vertical_launch_angle_degrees - Vertical launch angle
 * @param {number} [shotData.horizontal_launch_angle_degrees] - Horizontal launch angle (0 = straight)
 * @param {number} [shotData.total_spin_rpm] - Total spin rate
 * @param {number} [shotData.spin_axis_degrees] - Spin axis angle
 * @returns {Object} Shot data with added derived values
 *
 * @example
 * const result = calculateDerivedValues({
 *   ball_speed_meters_per_second: 70.0,
 *   vertical_launch_angle_degrees: 12.5,
 *   horizontal_launch_angle_degrees: -2.0,
 *   total_spin_rpm: 2800.0,
 *   spin_axis_degrees: 15.0
 * });
 *
 * console.log(result.carry_distance_meters);
 * console.log(result.offline_distance_meters);
 * console.log(result.backspin_rpm);
 * console.log(result.sidespin_rpm);
 */
function calculateDerivedValues(shotData) {
  const jsonInput = JSON.stringify(shotData);
  const jsonOutput = wasm.calculate_derived_values(jsonInput);
  return JSON.parse(jsonOutput);
}

module.exports = {
  calculateDerivedValues
};
