/**
 * OpenGolfCoach - Node.js Example
 *
 * This example demonstrates how to use the OpenGolfCoach library
 * to calculate derived golf shot values.
 */

const { calculateDerivedValues } = require('../../bindings/nodejs/index.js');

// Example 1: Basic shot with total spin and spin axis
console.log('Example 1: Calculate carry, offline, backspin, and sidespin');
console.log('=' .repeat(60));

const shot1 = {
  ball_speed_meters_per_second: 70.0,
  vertical_launch_angle_degrees: 12.5,
  horizontal_launch_angle_degrees: -2.0,
  total_spin_rpm: 2800.0,
  spin_axis_degrees: 15.0
};

console.log('Input:');
console.log(JSON.stringify(shot1, null, 2));

const result1 = calculateDerivedValues(shot1);

console.log('\nOutput:');
console.log(JSON.stringify(result1, null, 2));

console.log('\nDerived values:');
console.log(`  Carry distance: ${result1.open_golf_coach.carry_distance_meters.toFixed(2)} meters`);
console.log(`  Offline distance: ${result1.open_golf_coach.offline_distance_meters.toFixed(2)} meters`);
console.log(`  Backspin: ${result1.open_golf_coach.backspin_rpm.toFixed(1)} RPM`);
console.log(`  Sidespin: ${result1.open_golf_coach.sidespin_rpm.toFixed(1)} RPM`);

// Example 2: Shot with backspin and sidespin (calculate total spin and axis)
console.log('\n\nExample 2: Calculate total spin and spin axis from components');
console.log('='.repeat(60));

const shot2 = {
  ball_speed_meters_per_second: 65.0,
  vertical_launch_angle_degrees: 14.0,
  horizontal_launch_angle_degrees: 1.5,
  backspin_rpm: 3500.0,
  sidespin_rpm: -800.0  // Negative = slice/fade
};

console.log('Input:');
console.log(JSON.stringify(shot2, null, 2));

const result2 = calculateDerivedValues(shot2);

console.log('\nOutput:');
console.log(JSON.stringify(result2, null, 2));

console.log('\nDerived values:');
console.log(`  Total spin: ${result2.open_golf_coach.total_spin_rpm.toFixed(1)} RPM`);
console.log(`  Spin axis: ${result2.open_golf_coach.spin_axis_degrees.toFixed(2)} degrees`);
console.log(`  Carry distance: ${result2.open_golf_coach.carry_distance_meters.toFixed(2)} meters`);
console.log(`  Offline distance: ${result2.open_golf_coach.offline_distance_meters.toFixed(2)} meters`);

// Example 3: Minimal input (just ball speed and launch angle)
console.log('\n\nExample 3: Minimal input');
console.log('='.repeat(60));

const shot3 = {
  ball_speed_meters_per_second: 75.0,
  vertical_launch_angle_degrees: 11.0
};

console.log('Input:');
console.log(JSON.stringify(shot3, null, 2));

const result3 = calculateDerivedValues(shot3);

console.log('\nOutput:');
console.log(JSON.stringify(result3, null, 2));

console.log('\nDerived values:');
console.log(`  Carry distance: ${result3.open_golf_coach.carry_distance_meters.toFixed(2)} meters`);
console.log(`  Offline distance: ${result3.open_golf_coach.offline_distance_meters.toFixed(2)} meters`);

// Example 4: Opt in to the simulated trajectory for animation
console.log('\n\nExample 4: Trajectory output for animation (60 Hz)');
console.log('='.repeat(60));

const shot4 = {
  ball_speed_meters_per_second: 70.0,
  vertical_launch_angle_degrees: 12.5,
  horizontal_launch_angle_degrees: -2.0,
  total_spin_rpm: 2800.0,
  spin_axis_degrees: 15.0,
  trajectory_enabled: true,
  trajectory_output_framerate_hz: 60
};

const result4 = calculateDerivedValues(shot4);
const trajectory = result4.open_golf_coach.trajectory;

console.log(`  sample_rate_hz: ${trajectory.sample_rate_hz}`);
console.log(`  point count:    ${trajectory.points.length}`);
console.log('  first 3 points:');
for (const p of trajectory.points.slice(0, 3)) {
  console.log(`    t=${p.t.toFixed(4)}  x=${p.x.toFixed(3)}  y=${p.y.toFixed(3)}  z=${p.z.toFixed(3)}`);
}
const last = trajectory.points[trajectory.points.length - 1];
console.log(`  landing:`);
console.log(`    t=${last.t.toFixed(4)}  x=${last.x.toFixed(3)}  y=${last.y.toFixed(3)}  z=${last.z.toFixed(3)}`);

// To consume in Three.js / glTF / WebGL native frame, swizzle each point to
// (p.y, p.z, -p.x). For Unity, swizzle to (p.y, p.z, p.x). Unreal can use
// the points as-is (matching coordinate frame).
