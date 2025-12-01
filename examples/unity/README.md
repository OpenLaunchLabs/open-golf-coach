# Unity Example

This example demonstrates how to use OpenGolfCoach in Unity.

## Setup

1. Follow the installation instructions in `bindings/unity/README.md`
2. Copy `GolfShotExample.cs` to your Unity project's `Assets/Scripts/` folder
3. Ensure the native library is properly installed in `Assets/Plugins/x86_64/`

## Using the Example Component

### Option 1: Add to GameObject

1. Create an empty GameObject in your scene
2. Add the `GolfShotExample` component
3. Enter Play mode - calculations will run automatically
4. Check the Console for output

### Option 2: Use Context Menu

1. Add the component to a GameObject
2. Right-click the component in the Inspector
3. Select one of the context menu options:
   - "Calculate Example 1"
   - "Calculate Example 2"
   - "Calculate Example 3"
   - "Batch Process Shots"

## Expected Output

When you run the examples, you should see output like:

```
=== OpenGolfCoach Unity Examples ===

--- Example 1: Calculate from Total Spin ---
Input: 70 m/s, 12.5°V, -2°H, 2800 RPM, 15° axis
  Carry: 185.42 meters
  Offline: -6.21 meters
  Backspin: 2704.7 RPM
  Sidespin: 724.8 RPM

--- Example 2: Calculate from Spin Components ---
Input: 65 m/s, 14°V, 1.5°H, 3500 backspin, -800 sidespin
  Total Spin: 3590.6 RPM
  Spin Axis: -12.89 degrees
  Carry: 178.23 meters
  Offline: 3.45 meters

--- Example 3: Minimal Input ---
Input: 75 m/s, 11°
  Carry: 198.67 meters
  Offline: 0.00 meters
```

## Integration Ideas

### Golf Ball Physics

```csharp
public class GolfBall : MonoBehaviour
{
    public void Hit(float ballSpeed, float vLaunch, float hLaunch)
    {
        var shot = GolfShotBuilder.Create(ballSpeed, vLaunch, hLaunch);
        var result = GolfCalculator.CalculateDerivedValues(shot);

        // Use result to apply physics
        Vector3 targetPos = CalculateTargetPosition(result);
        StartCoroutine(AnimateTrajectory(targetPos));
    }
}
```

### Launch Monitor UI

```csharp
public class LaunchMonitorUI : MonoBehaviour
{
    public void OnShotDetected(GolfShotData rawData)
    {
        if (GolfCalculator.TryCalculateDerivedValues(rawData, out var result))
        {
            carryText.text = $"{result.carry_distance_meters:F1}m";
            offlineText.text = $"{result.offline_distance_meters:F1}m";
            backspinText.text = $"{result.backspin_rpm:F0} RPM";
            sidespinText.text = $"{result.sidespin_rpm:F0} RPM";
        }
    }
}
```

### Practice Range

```csharp
public class PracticeRange : MonoBehaviour
{
    public void SimulateShot()
    {
        // Random shot parameters
        var shot = GolfShotBuilder.Create(
            Random.Range(60f, 80f),
            Random.Range(10f, 15f),
            Random.Range(-5f, 5f),
            Random.Range(2000f, 4000f),
            Random.Range(-20f, 20f)
        );

        var result = GolfCalculator.CalculateDerivedValues(shot);
        SpawnBallWithTrajectory(result);
    }
}
```

## Customization

Feel free to modify the example component to:
- Add visual trajectory rendering
- Connect to a launch monitor API
- Create a ball flight simulator
- Build a golf training application
- Develop shot analysis tools

## Performance Notes

- Calculations are very fast (< 1ms typically)
- Safe to call every frame if needed
- Consider caching results if parameters don't change
- Use `TryCalculateDerivedValues` for safe error handling
