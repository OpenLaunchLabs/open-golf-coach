// Copyright Open Launch Labs. Licensed under Apache 2.0.

using System;
using System.Runtime.InteropServices;
using System.Text;
using UnityEngine;

namespace OpenGolfCoach
{
    /// <summary>
    /// Golf shot data structure
    /// </summary>
    [Serializable]
    public class GolfShotData
    {
        public float ball_speed_meters_per_second;
        public float vertical_launch_angle_degrees;
        public float horizontal_launch_angle_degrees;
        public float total_spin_rpm;
        public float spin_axis_degrees;
        public float ball_speed_mph;
        public float carry_distance_meters;
        public float total_distance_meters;
        public float offline_distance_meters;
        public float carry_distance_yards;
        public float total_distance_yards;
        public float offline_distance_yards;
        public float backspin_rpm;
        public float sidespin_rpm;
        public float club_speed_meters_per_second;
        public float club_speed_mph;
        public float smash_factor;
        public float club_path_degrees;
        public float club_face_to_target_degrees;
        public float club_face_to_path_degrees;
        public Vector3 landing_position_yards;
        public Vector3 landing_velocity_mph;
        public float peak_height_yards;
        public string shot_name;
        public string shot_rank;
        public string shot_color_rgb;
        public USCustomaryUnits us_customary_units;

        public GolfShotData()
        {
            ball_speed_meters_per_second = 0f;
            vertical_launch_angle_degrees = 0f;
            horizontal_launch_angle_degrees = 0f;
            total_spin_rpm = 0f;
            spin_axis_degrees = 0f;
            ball_speed_mph = 0f;
            carry_distance_meters = 0f;
            total_distance_meters = 0f;
            offline_distance_meters = 0f;
            carry_distance_yards = 0f;
            total_distance_yards = 0f;
            offline_distance_yards = 0f;
            backspin_rpm = 0f;
            sidespin_rpm = 0f;
            club_speed_meters_per_second = 0f;
            club_speed_mph = 0f;
            smash_factor = 0f;
            club_path_degrees = 0f;
            club_face_to_target_degrees = 0f;
            club_face_to_path_degrees = 0f;
            landing_position_yards = Vector3.zero;
            landing_velocity_mph = Vector3.zero;
            peak_height_yards = 0f;
            shot_name = string.Empty;
            shot_rank = string.Empty;
            shot_color_rgb = string.Empty;
            us_customary_units = new USCustomaryUnits();
        }
    }

    [Serializable]
    public class USCustomaryUnits
    {
        public float ball_speed_mph;
        public float club_speed_mph;
        public float carry_distance_yards;
        public float total_distance_yards;
        public float offline_distance_yards;
        public Vector3 landing_position_yards;
        public Vector3 landing_velocity_mph;
        public float peak_height_yards;

        public USCustomaryUnits()
        {
            ball_speed_mph = 0f;
            club_speed_mph = 0f;
            carry_distance_yards = 0f;
            total_distance_yards = 0f;
            offline_distance_yards = 0f;
            landing_position_yards = Vector3.zero;
            landing_velocity_mph = Vector3.zero;
            peak_height_yards = 0f;
        }
    }

    /// <summary>
    /// Main library interface for golf shot calculations
    /// </summary>
    public static class GolfCalculator
    {
        private const string LibraryName = "opengolfcoach";

        [DllImport(LibraryName, EntryPoint = "calculate_derived_values_ffi", CallingConvention = CallingConvention.Cdecl)]
        private static extern int CalculateDerivedValuesFFI(
            [MarshalAs(UnmanagedType.LPStr)] string jsonInput,
            StringBuilder outputBuffer,
            int bufferSize
        );

        /// <summary>
        /// Calculate derived golf shot values
        /// </summary>
        /// <param name="shotData">Input golf shot data</param>
        /// <returns>Golf shot data with derived values calculated</returns>
        /// <exception cref="InvalidOperationException">Thrown when calculation fails</exception>
        public static GolfShotData CalculateDerivedValues(GolfShotData shotData)
        {
            // Convert to JSON
            string jsonInput = JsonUtility.ToJson(shotData);

            // Call native function
            StringBuilder output = new StringBuilder(8192);
            int result = CalculateDerivedValuesFFI(jsonInput, output, output.Capacity);

            if (result != 0)
            {
                string errorMessage = result switch
                {
                    -1 => "Null pointer error",
                    -2 => "Input string is not valid UTF-8",
                    -3 => "JSON parsing failed",
                    -4 => "JSON serialization failed",
                    -5 => "Output string conversion failed",
                    -6 => "Output buffer too small",
                    _ => "Unknown error"
                };

                throw new InvalidOperationException($"Golf calculation failed: {errorMessage} (code: {result})");
            }

            // Parse output JSON
            string jsonOutput = output.ToString();
            return JsonUtility.FromJson<GolfShotData>(jsonOutput);
        }

        /// <summary>
        /// Calculate derived values with error handling that logs to Unity console
        /// </summary>
        /// <param name="shotData">Input golf shot data</param>
        /// <param name="result">Output golf shot data (null if failed)</param>
        /// <returns>True if calculation succeeded, false otherwise</returns>
        public static bool TryCalculateDerivedValues(GolfShotData shotData, out GolfShotData result)
        {
            try
            {
                result = CalculateDerivedValues(shotData);
                return true;
            }
            catch (Exception e)
            {
                Debug.LogError($"OpenGolfCoach calculation failed: {e.Message}");
                result = null;
                return false;
            }
        }
    }

    /// <summary>
    /// Helper class for creating golf shot data
    /// </summary>
    public static class GolfShotBuilder
    {
        public static GolfShotData Create(
            float ballSpeed,
            float verticalLaunch,
            float horizontalLaunch = 0f,
            float totalSpin = 0f,
            float spinAxis = 0f)
        {
            return new GolfShotData
            {
                ball_speed_meters_per_second = ballSpeed,
                vertical_launch_angle_degrees = verticalLaunch,
                horizontal_launch_angle_degrees = horizontalLaunch,
                total_spin_rpm = totalSpin,
                spin_axis_degrees = spinAxis
            };
        }

        public static GolfShotData CreateWithSpinComponents(
            float ballSpeed,
            float verticalLaunch,
            float horizontalLaunch,
            float backspin,
            float sidespin)
        {
            return new GolfShotData
            {
                ball_speed_meters_per_second = ballSpeed,
                vertical_launch_angle_degrees = verticalLaunch,
                horizontal_launch_angle_degrees = horizontalLaunch,
                backspin_rpm = backspin,
                sidespin_rpm = sidespin
            };
        }
    }
}
