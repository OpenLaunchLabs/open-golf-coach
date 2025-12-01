// Copyright Open Launch Labs. Licensed under Apache 2.0.
// Example MonoBehaviour showing how to use OpenGolfCoach in Unity

using UnityEngine;
using OpenGolfCoach;

/// <summary>
/// Example component that demonstrates golf shot calculations in Unity
/// </summary>
public class GolfShotExample : MonoBehaviour
{
    [Header("Example 1: Calculate from Total Spin")]
    [SerializeField] private float example1BallSpeed = 70.0f;
    [SerializeField] private float example1VerticalLaunch = 12.5f;
    [SerializeField] private float example1HorizontalLaunch = -2.0f;
    [SerializeField] private float example1TotalSpin = 2800.0f;
    [SerializeField] private float example1SpinAxis = 15.0f;

    [Header("Example 2: Calculate from Spin Components")]
    [SerializeField] private float example2BallSpeed = 65.0f;
    [SerializeField] private float example2VerticalLaunch = 14.0f;
    [SerializeField] private float example2HorizontalLaunch = 1.5f;
    [SerializeField] private float example2Backspin = 3500.0f;
    [SerializeField] private float example2Sidespin = -800.0f;

    [Header("Results")]
    [SerializeField] private GolfShotData result1;
    [SerializeField] private GolfShotData result2;

    void Start()
    {
        Debug.Log("=== OpenGolfCoach Unity Examples ===");
        CalculateExample1();
        CalculateExample2();
        CalculateExample3();
    }

    /// <summary>
    /// Example 1: Calculate carry, offline, backspin, and sidespin from total spin
    /// </summary>
    [ContextMenu("Calculate Example 1")]
    public void CalculateExample1()
    {
        Debug.Log("\n--- Example 1: Calculate from Total Spin ---");

        var shot = GolfShotBuilder.Create(
            example1BallSpeed,
            example1VerticalLaunch,
            example1HorizontalLaunch,
            example1TotalSpin,
            example1SpinAxis
        );

        Debug.Log($"Input: {example1BallSpeed} m/s, {example1VerticalLaunch}°V, " +
                  $"{example1HorizontalLaunch}°H, {example1TotalSpin} RPM, {example1SpinAxis}° axis");

        if (GolfCalculator.TryCalculateDerivedValues(shot, out result1))
        {
            Debug.Log($"  Carry: {result1.carry_distance_meters:F2} meters");
            Debug.Log($"  Offline: {result1.offline_distance_meters:F2} meters");
            Debug.Log($"  Backspin: {result1.backspin_rpm:F1} RPM");
            Debug.Log($"  Sidespin: {result1.sidespin_rpm:F1} RPM");
        }
    }

    /// <summary>
    /// Example 2: Calculate total spin and spin axis from components
    /// </summary>
    [ContextMenu("Calculate Example 2")]
    public void CalculateExample2()
    {
        Debug.Log("\n--- Example 2: Calculate from Spin Components ---");

        var shot = GolfShotBuilder.CreateWithSpinComponents(
            example2BallSpeed,
            example2VerticalLaunch,
            example2HorizontalLaunch,
            example2Backspin,
            example2Sidespin
        );

        Debug.Log($"Input: {example2BallSpeed} m/s, {example2VerticalLaunch}°V, " +
                  $"{example2HorizontalLaunch}°H, {example2Backspin} backspin, {example2Sidespin} sidespin");

        if (GolfCalculator.TryCalculateDerivedValues(shot, out result2))
        {
            Debug.Log($"  Total Spin: {result2.total_spin_rpm:F1} RPM");
            Debug.Log($"  Spin Axis: {result2.spin_axis_degrees:F2} degrees");
            Debug.Log($"  Carry: {result2.carry_distance_meters:F2} meters");
            Debug.Log($"  Offline: {result2.offline_distance_meters:F2} meters");
        }
    }

    /// <summary>
    /// Example 3: Minimal input
    /// </summary>
    [ContextMenu("Calculate Example 3")]
    public void CalculateExample3()
    {
        Debug.Log("\n--- Example 3: Minimal Input ---");

        var shot = new GolfShotData
        {
            ball_speed_meters_per_second = 75.0f,
            vertical_launch_angle_degrees = 11.0f
        };

        Debug.Log($"Input: {shot.ball_speed_meters_per_second} m/s, {shot.vertical_launch_angle_degrees}°");

        try
        {
            var result = GolfCalculator.CalculateDerivedValues(shot);
            Debug.Log($"  Carry: {result.carry_distance_meters:F2} meters");
            Debug.Log($"  Offline: {result.offline_distance_meters:F2} meters");
        }
        catch (System.Exception e)
        {
            Debug.LogError($"Calculation failed: {e.Message}");
        }
    }

    /// <summary>
    /// Example of using calculated values to visualize trajectory
    /// </summary>
    public void VisualizeTrajectory(GolfShotData shotData)
    {
        if (!GolfCalculator.TryCalculateDerivedValues(shotData, out var result))
        {
            return;
        }

        // Use the results to create a visual trajectory
        Vector3 startPosition = transform.position;
        Vector3 endPosition = new Vector3(
            startPosition.x + result.carry_distance_meters,
            startPosition.y,
            startPosition.z + result.offline_distance_meters
        );

        Debug.DrawLine(startPosition, endPosition, Color.green, 5.0f);
        Debug.Log($"Trajectory: {startPosition} -> {endPosition}");
    }

    /// <summary>
    /// Example of batch processing multiple shots
    /// </summary>
    [ContextMenu("Batch Process Shots")]
    public void BatchProcessShots()
    {
        Debug.Log("\n--- Batch Processing ---");

        var shots = new[]
        {
            GolfShotBuilder.Create(60.0f, 15.0f),
            GolfShotBuilder.Create(70.0f, 12.0f),
            GolfShotBuilder.Create(80.0f, 10.0f),
        };

        for (int i = 0; i < shots.Length; i++)
        {
            if (GolfCalculator.TryCalculateDerivedValues(shots[i], out var result))
            {
                Debug.Log($"Shot {i + 1}: {result.ball_speed_meters_per_second:F0} m/s, " +
                          $"{result.vertical_launch_angle_degrees:F0}° -> " +
                          $"{result.carry_distance_meters:F1}m carry");
            }
        }
    }
}
