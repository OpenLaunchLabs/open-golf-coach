# OpenGolfCoach - Unity Integration

Unity integration for the OpenGolfCoach library.

## Installation

### 1. Build the Rust Core Library

```bash
cd ../../core
cargo build --release
```

### 2. Set Up Unity Project

Create the following structure in your Unity project:

```
Assets/
  Plugins/
    OpenGolfCoach/
      OpenGolfCoach.cs
    x86_64/               # 64-bit libraries
      opengolfcoach.dll   # Windows
      libopengolfcoach.so # Linux
      libopengolfcoach.dylib # macOS
```

### 3. Copy Files

1. Copy `OpenGolfCoach.cs` to `Assets/Plugins/OpenGolfCoach/`
2. Copy the compiled Rust library to the appropriate platform folder:
   - Windows: `opengolfcoach.dll` → `Assets/Plugins/x86_64/`
   - macOS: `libopengolfcoach.dylib` → `Assets/Plugins/x86_64/`
   - Linux: `libopengolfcoach.so` → `Assets/Plugins/x86_64/`

### 4. Configure Plugin Import Settings

For each native library:

1. Select the library in Unity's Project window
2. In the Inspector, configure:
   - **Select platforms**: Check the appropriate platform (Windows, macOS, Linux)
   - **CPU**: x86_64
   - **Load on startup**: Checked

## Usage

### Basic Usage

```csharp
using OpenGolfCoach;
using UnityEngine;

public class GolfExample : MonoBehaviour
{
    void Start()
    {
        // Create shot data
        var shot = new GolfShotData
        {
            ball_speed_meters_per_second = 70.0f,
            vertical_launch_angle_degrees = 12.5f,
            horizontal_launch_angle_degrees = -2.0f,
            total_spin_rpm = 2800.0f,
            spin_axis_degrees = 15.0f
        };

        // Calculate derived values
        var result = GolfCalculator.CalculateDerivedValues(shot);

        // Use the results
        Debug.Log($"Carry: {result.carry_distance_meters:F2} meters");
        Debug.Log($"Offline: {result.offline_distance_meters:F2} meters");
        Debug.Log($"Backspin: {result.backspin_rpm:F1} RPM");
        Debug.Log($"Sidespin: {result.sidespin_rpm:F1} RPM");
    }
}
```

### Using Builder Pattern

```csharp
using OpenGolfCoach;

// Create shot with total spin
var shot = GolfShotBuilder.Create(
    ballSpeed: 70.0f,
    verticalLaunch: 12.5f,
    horizontalLaunch: -2.0f,
    totalSpin: 2800.0f,
    spinAxis: 15.0f
);

var result = GolfCalculator.CalculateDerivedValues(shot);

// Or create with spin components
var shot2 = GolfShotBuilder.CreateWithSpinComponents(
    ballSpeed: 65.0f,
    verticalLaunch: 14.0f,
    horizontalLaunch: 1.5f,
    backspin: 3500.0f,
    sidespin: -800.0f
);
```

### Safe Calculation with Error Handling

```csharp
using OpenGolfCoach;

var shot = GolfShotBuilder.Create(70.0f, 12.5f);

if (GolfCalculator.TryCalculateDerivedValues(shot, out var result))
{
    // Success - use result
    Debug.Log($"Carry: {result.carry_distance_meters} meters");
}
else
{
    // Failed - error already logged to console
    Debug.LogWarning("Calculation failed, using default values");
}
```

## Example MonoBehaviour

See `examples/unity/GolfShotExample.cs` for a complete MonoBehaviour component.

## Troubleshooting

### DllNotFoundException

If you get `DllNotFoundException: Unable to load DLL 'opengolfcoach'`:

1. Verify the native library is in the correct Plugins folder
2. Check the import settings for the library in Unity Inspector
3. Ensure the platform is selected for your build target
4. Try restarting Unity Editor

### Platform-Specific Issues

**Windows:**
- Ensure Visual C++ Redistributable is installed
- Library must be named `opengolfcoach.dll`

**macOS:**
- You may need to remove quarantine: `xattr -d com.apple.quarantine libopengolfcoach.dylib`
- Library must be named `libopengolfcoach.dylib`

**Linux:**
- Library must be named `libopengolfcoach.so`
- Ensure execute permissions: `chmod +x libopengolfcoach.so`

### JSON Serialization Issues

Unity's JsonUtility requires:
- Public fields (not properties)
- Exact field name matching (case-sensitive)
- `[Serializable]` attribute on the class

The provided `GolfShotData` class is already properly configured.
