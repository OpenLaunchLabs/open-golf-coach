# Unreal Engine Example

This example demonstrates how to use OpenGolfCoach in Unreal Engine 5.x.

## Setup

1. Follow the installation instructions in `bindings/unreal/README.md`
2. Copy the example files to your Unreal project:
   - `GolfShotComponent.h` → `Source/YourProject/Public/`
   - `GolfShotComponent.cpp` → `Source/YourProject/Private/`

## Usage in C++

### Option 1: Use the Component

```cpp
// Add component to an actor
UGolfShotComponent* GolfComponent = CreateDefaultSubobject<UGolfShotComponent>(TEXT("GolfComponent"));

// Calculate a sample shot
GolfComponent->CalculateSampleShot();

// Calculate a custom shot
FGolfShotData Result;
GolfComponent->CalculateCustomShot(70.0f, 12.5f, -2.0f, 2800.0f, 15.0f, Result);
```

### Option 2: Use the Library Directly

```cpp
#include "OpenGolfCoach.h"

void MyFunction()
{
    FGolfShotData InputShot;
    InputShot.BallSpeedMeterPerSecond = 70.0f;
    InputShot.VerticalLaunchAngleDegrees = 12.5f;
    InputShot.TotalSpinRPM = 2800.0f;
    InputShot.SpinAxisDegrees = 15.0f;

    FGolfShotData OutputShot;
    if (UOpenGolfCoachLibrary::CalculateDerivedValues(InputShot, OutputShot))
    {
        // Use the results
        float Carry = OutputShot.CarryDistanceMeters;
        float Offline = OutputShot.OfflineDistanceMeters;
    }
}
```

## Usage in Blueprints

1. Add a **Golf Shot Component** to any actor
2. In the Event Graph:
   - Create a **Golf Shot Data** variable
   - Set the desired values (Ball Speed, Launch Angles, etc.)
   - Call **Calculate Derived Values**
   - Access the output values

### Example Blueprint Flow

```
Event BeginPlay
  → Make Golf Shot Data (Ball Speed: 70, V Launch: 12.5, etc.)
  → Calculate Derived Values
  → Print String (Carry Distance)
  → Print String (Offline Distance)
```

## Example Output

When you run a level with the GolfShotComponent attached, you'll see output like:

```
LogTemp: === OpenGolfCoach Example Calculations ===

LogTemp: Example 1: Calculate carry, offline, and spin components
LogTemp:   Input:
LogTemp:     Ball Speed: 70.0 m/s
LogTemp:     V Launch: 12.5 degrees
LogTemp:     H Launch: -2.0 degrees
LogTemp:     Total Spin: 2800 RPM
LogTemp:     Spin Axis: 15.0 degrees
LogTemp:   Output:
LogTemp:     Carry: 185.42 meters
LogTemp:     Offline: -6.21 meters
LogTemp:     Backspin: 2704.7 RPM
LogTemp:     Sidespin: 724.8 RPM
```

## Integration Tips

- Use the component for actors that need golf calculations (golf balls, practice ranges)
- Call the library directly for one-off calculations
- Expose custom Blueprint functions for your specific use cases
- Cache the library reference to avoid repeated lookups
