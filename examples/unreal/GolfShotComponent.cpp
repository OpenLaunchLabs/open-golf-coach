// Copyright Open Launch Labs. Licensed under Apache 2.0.

#include "GolfShotComponent.h"

UGolfShotComponent::UGolfShotComponent()
{
    PrimaryComponentTick.bCanEverTick = false;

    // Example 1: Basic shot with total spin and spin axis
    ExampleShot1.BallSpeedMeterPerSecond = 70.0f;
    ExampleShot1.VerticalLaunchAngleDegrees = 12.5f;
    ExampleShot1.HorizontalLaunchAngleDegrees = -2.0f;
    ExampleShot1.TotalSpinRPM = 2800.0f;
    ExampleShot1.SpinAxisDegrees = 15.0f;

    // Example 2: Shot with backspin and sidespin components
    ExampleShot2.BallSpeedMeterPerSecond = 65.0f;
    ExampleShot2.VerticalLaunchAngleDegrees = 14.0f;
    ExampleShot2.HorizontalLaunchAngleDegrees = 1.5f;
    ExampleShot2.BackspinRPM = 3500.0f;
    ExampleShot2.SidespinRPM = -800.0f; // Negative = slice/fade
}

void UGolfShotComponent::BeginPlay()
{
    Super::BeginPlay();

    // Calculate and log example shots on begin play
    UE_LOG(LogTemp, Log, TEXT("=== OpenGolfCoach Example Calculations ==="));

    // Example 1
    FGolfShotData Result1;
    if (UOpenGolfCoachLibrary::CalculateDerivedValues(ExampleShot1, Result1))
    {
        UE_LOG(LogTemp, Log, TEXT("\nExample 1: Calculate carry, offline, and spin components"));
        UE_LOG(LogTemp, Log, TEXT("  Input:"));
        UE_LOG(LogTemp, Log, TEXT("    Ball Speed: %.1f m/s"), ExampleShot1.BallSpeedMeterPerSecond);
        UE_LOG(LogTemp, Log, TEXT("    V Launch: %.1f degrees"), ExampleShot1.VerticalLaunchAngleDegrees);
        UE_LOG(LogTemp, Log, TEXT("    H Launch: %.1f degrees"), ExampleShot1.HorizontalLaunchAngleDegrees);
        UE_LOG(LogTemp, Log, TEXT("    Total Spin: %.0f RPM"), ExampleShot1.TotalSpinRPM);
        UE_LOG(LogTemp, Log, TEXT("    Spin Axis: %.1f degrees"), ExampleShot1.SpinAxisDegrees);
        UE_LOG(LogTemp, Log, TEXT("  Output:"));
        UE_LOG(LogTemp, Log, TEXT("    Carry: %.2f meters"), Result1.CarryDistanceMeters);
        UE_LOG(LogTemp, Log, TEXT("    Offline: %.2f meters"), Result1.OfflineDistanceMeters);
        UE_LOG(LogTemp, Log, TEXT("    Backspin: %.1f RPM"), Result1.BackspinRPM);
        UE_LOG(LogTemp, Log, TEXT("    Sidespin: %.1f RPM"), Result1.SidespinRPM);
    }
    else
    {
        UE_LOG(LogTemp, Error, TEXT("Example 1 calculation failed"));
    }

    // Example 2
    FGolfShotData Result2;
    if (UOpenGolfCoachLibrary::CalculateDerivedValues(ExampleShot2, Result2))
    {
        UE_LOG(LogTemp, Log, TEXT("\nExample 2: Calculate total spin and spin axis from components"));
        UE_LOG(LogTemp, Log, TEXT("  Input:"));
        UE_LOG(LogTemp, Log, TEXT("    Ball Speed: %.1f m/s"), ExampleShot2.BallSpeedMeterPerSecond);
        UE_LOG(LogTemp, Log, TEXT("    V Launch: %.1f degrees"), ExampleShot2.VerticalLaunchAngleDegrees);
        UE_LOG(LogTemp, Log, TEXT("    H Launch: %.1f degrees"), ExampleShot2.HorizontalLaunchAngleDegrees);
        UE_LOG(LogTemp, Log, TEXT("    Backspin: %.0f RPM"), ExampleShot2.BackspinRPM);
        UE_LOG(LogTemp, Log, TEXT("    Sidespin: %.0f RPM"), ExampleShot2.SidespinRPM);
        UE_LOG(LogTemp, Log, TEXT("  Output:"));
        UE_LOG(LogTemp, Log, TEXT("    Total Spin: %.1f RPM"), Result2.TotalSpinRPM);
        UE_LOG(LogTemp, Log, TEXT("    Spin Axis: %.2f degrees"), Result2.SpinAxisDegrees);
        UE_LOG(LogTemp, Log, TEXT("    Carry: %.2f meters"), Result2.CarryDistanceMeters);
        UE_LOG(LogTemp, Log, TEXT("    Offline: %.2f meters"), Result2.OfflineDistanceMeters);
    }
    else
    {
        UE_LOG(LogTemp, Error, TEXT("Example 2 calculation failed"));
    }
}

void UGolfShotComponent::CalculateSampleShot()
{
    FGolfShotData InputShot;
    InputShot.BallSpeedMeterPerSecond = 75.0f;
    InputShot.VerticalLaunchAngleDegrees = 11.0f;
    InputShot.HorizontalLaunchAngleDegrees = 0.0f;
    InputShot.TotalSpinRPM = 3000.0f;
    InputShot.SpinAxisDegrees = 0.0f;

    FGolfShotData Result;
    if (UOpenGolfCoachLibrary::CalculateDerivedValues(InputShot, Result))
    {
        UE_LOG(LogTemp, Log, TEXT("\nSample Shot Calculation:"));
        UE_LOG(LogTemp, Log, TEXT("  Carry: %.2f meters"), Result.CarryDistanceMeters);
        UE_LOG(LogTemp, Log, TEXT("  Offline: %.2f meters"), Result.OfflineDistanceMeters);
    }
}

void UGolfShotComponent::CalculateCustomShot(
    float BallSpeed,
    float VerticalLaunch,
    float HorizontalLaunch,
    float TotalSpin,
    float SpinAxis,
    FGolfShotData& OutResult)
{
    FGolfShotData InputShot;
    InputShot.BallSpeedMeterPerSecond = BallSpeed;
    InputShot.VerticalLaunchAngleDegrees = VerticalLaunch;
    InputShot.HorizontalLaunchAngleDegrees = HorizontalLaunch;
    InputShot.TotalSpinRPM = TotalSpin;
    InputShot.SpinAxisDegrees = SpinAxis;

    if (UOpenGolfCoachLibrary::CalculateDerivedValues(InputShot, OutResult))
    {
        UE_LOG(LogTemp, Log, TEXT("Custom shot calculated successfully"));
    }
    else
    {
        UE_LOG(LogTemp, Error, TEXT("Custom shot calculation failed"));
    }
}
