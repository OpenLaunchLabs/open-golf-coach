# Node.js Example

This example demonstrates how to use OpenGolfCoach in a Node.js application.

## Setup

First, build the WebAssembly module:

```bash
cd ../../core
wasm-pack build --target nodejs
```

This will create a `pkg` directory with the compiled WebAssembly module.

## Running the Example

```bash
node example.js
```

## Integration in Your Project

1. Install the package (once published):

```bash
npm install opengolfcoach
```

2. Use in your code:

```javascript
const { calculateDerivedValues } = require('opengolfcoach');

const shot = {
  ball_speed_meters_per_second: 70.0,
  vertical_launch_angle_degrees: 12.5,
  total_spin_rpm: 2800.0,
  spin_axis_degrees: 15.0
};

const result = calculateDerivedValues(shot);
console.log(result.carry_distance_meters);
```

## TypeScript Support

The package includes TypeScript definitions:

```typescript
import { calculateDerivedValues, GolfShot } from 'opengolfcoach';

const shot: GolfShot = {
  ball_speed_meters_per_second: 70.0,
  vertical_launch_angle_degrees: 12.5,
  total_spin_rpm: 2800.0,
  spin_axis_degrees: 15.0
};

const result = calculateDerivedValues(shot);
```
