// Copyright Open Launch Labs. Licensed under Apache 2.0.

#include "OpenGolfCoach.h"
#include "HAL/PlatformProcess.h"
#include "Misc/Paths.h"
#include "Dom/JsonObject.h"
#include "Serialization/JsonReader.h"
#include "Serialization/JsonSerializer.h"

void* UOpenGolfCoachLibrary::LibraryHandle = nullptr;
UOpenGolfCoachLibrary::FCalculateDerivedValuesFn UOpenGolfCoachLibrary::CalculateFunction = nullptr;

bool UOpenGolfCoachLibrary::LoadLibrary()
{
    if (LibraryHandle != nullptr)
    {
        return true; // Already loaded
    }

    // Determine library name based on platform
    FString LibraryName;
#if PLATFORM_WINDOWS
    LibraryName = TEXT("opengolfcoach.dll");
#elif PLATFORM_MAC
    LibraryName = TEXT("libopengolfcoach.dylib");
#elif PLATFORM_LINUX
    LibraryName = TEXT("libopengolfcoach.so");
#else
    UE_LOG(LogTemp, Error, TEXT("OpenGolfCoach: Unsupported platform"));
    return false;
#endif

    // Try to load from plugin binaries directory
    FString LibraryPath = FPaths::Combine(FPaths::ProjectPluginsDir(), TEXT("OpenGolfCoach/Binaries"), FPlatformProcess::GetBinariesSubdirectory(), LibraryName);

    LibraryHandle = FPlatformProcess::GetDllHandle(*LibraryPath);

    if (LibraryHandle == nullptr)
    {
        UE_LOG(LogTemp, Error, TEXT("OpenGolfCoach: Failed to load library from %s"), *LibraryPath);
        return false;
    }

    // Get function pointer
    CalculateFunction = (FCalculateDerivedValuesFn)FPlatformProcess::GetDllExport(LibraryHandle, TEXT("calculate_derived_values_ffi"));

    if (CalculateFunction == nullptr)
    {
        UE_LOG(LogTemp, Error, TEXT("OpenGolfCoach: Failed to find calculate_derived_values_ffi function"));
        FPlatformProcess::FreeDllHandle(LibraryHandle);
        LibraryHandle = nullptr;
        return false;
    }

    UE_LOG(LogTemp, Log, TEXT("OpenGolfCoach: Library loaded successfully"));
    return true;
}

void UOpenGolfCoachLibrary::UnloadLibrary()
{
    if (LibraryHandle != nullptr)
    {
        FPlatformProcess::FreeDllHandle(LibraryHandle);
        LibraryHandle = nullptr;
        CalculateFunction = nullptr;
    }
}

bool UOpenGolfCoachLibrary::CalculateDerivedValues(const FGolfShotData& ShotData, FGolfShotData& OutShotData)
{
    if (!LoadLibrary())
    {
        return false;
    }

    // Convert struct to JSON
    TSharedPtr<FJsonObject> JsonObject = MakeShareable(new FJsonObject);

    if (ShotData.BallSpeedMetersPerSecond > 0.0f)
        JsonObject->SetNumberField(TEXT("ball_speed_meters_per_second"), ShotData.BallSpeedMetersPerSecond);

    if (ShotData.VerticalLaunchAngleDegrees != 0.0f)
        JsonObject->SetNumberField(TEXT("vertical_launch_angle_degrees"), ShotData.VerticalLaunchAngleDegrees);

    if (ShotData.HorizontalLaunchAngleDegrees != 0.0f)
        JsonObject->SetNumberField(TEXT("horizontal_launch_angle_degrees"), ShotData.HorizontalLaunchAngleDegrees);

    if (ShotData.TotalSpinRPM > 0.0f)
        JsonObject->SetNumberField(TEXT("total_spin_rpm"), ShotData.TotalSpinRPM);

    if (ShotData.SpinAxisDegrees != 0.0f)
        JsonObject->SetNumberField(TEXT("spin_axis_degrees"), ShotData.SpinAxisDegrees);

    if (ShotData.BackspinRPM > 0.0f)
        JsonObject->SetNumberField(TEXT("backspin_rpm"), ShotData.BackspinRPM);

    if (ShotData.SidespinRPM != 0.0f)
        JsonObject->SetNumberField(TEXT("sidespin_rpm"), ShotData.SidespinRPM);

    FString InputJson;
    TSharedRef<TJsonWriter<>> JsonWriter = TJsonWriterFactory<>::Create(&InputJson);
    FJsonSerializer::Serialize(JsonObject.ToSharedRef(), JsonWriter);

    // Call Rust function
    char OutputBuffer[8192];
    int Result = CalculateFunction(TCHAR_TO_UTF8(*InputJson), OutputBuffer, sizeof(OutputBuffer));

    if (Result != 0)
    {
        UE_LOG(LogTemp, Error, TEXT("OpenGolfCoach: Calculation failed with error code %d"), Result);
        return false;
    }

    // Parse output JSON
    FString OutputJson(UTF8_TO_TCHAR(OutputBuffer));
    TSharedPtr<FJsonObject> OutputObject;
    TSharedRef<TJsonReader<>> Reader = TJsonReaderFactory<>::Create(OutputJson);

    if (!FJsonSerializer::Deserialize(Reader, OutputObject) || !OutputObject.IsValid())
    {
        UE_LOG(LogTemp, Error, TEXT("OpenGolfCoach: Failed to parse output JSON"));
        return false;
    }

    // Copy input fields from root JSON to output struct
    OutShotData.BallSpeedMetersPerSecond = OutputObject->GetNumberField(TEXT("ball_speed_meters_per_second"));
    OutShotData.VerticalLaunchAngleDegrees = OutputObject->GetNumberField(TEXT("vertical_launch_angle_degrees"));

    if (OutputObject->HasField(TEXT("horizontal_launch_angle_degrees")))
        OutShotData.HorizontalLaunchAngleDegrees = OutputObject->GetNumberField(TEXT("horizontal_launch_angle_degrees"));

    if (OutputObject->HasField(TEXT("total_spin_rpm")))
        OutShotData.TotalSpinRPM = OutputObject->GetNumberField(TEXT("total_spin_rpm"));

    if (OutputObject->HasField(TEXT("spin_axis_degrees")))
        OutShotData.SpinAxisDegrees = OutputObject->GetNumberField(TEXT("spin_axis_degrees"));

    // Get derived values from the open_golf_coach nested object
    const TSharedPtr<FJsonObject>* DerivedObject;
    if (OutputObject->TryGetObjectField(TEXT("open_golf_coach"), DerivedObject))
    {
        if ((*DerivedObject)->HasField(TEXT("carry_distance_meters")))
            OutShotData.CarryDistanceMeters = (*DerivedObject)->GetNumberField(TEXT("carry_distance_meters"));

        if ((*DerivedObject)->HasField(TEXT("offline_distance_meters")))
            OutShotData.OfflineDistanceMeters = (*DerivedObject)->GetNumberField(TEXT("offline_distance_meters"));

        if ((*DerivedObject)->HasField(TEXT("backspin_rpm")))
            OutShotData.BackspinRPM = (*DerivedObject)->GetNumberField(TEXT("backspin_rpm"));

        if ((*DerivedObject)->HasField(TEXT("sidespin_rpm")))
            OutShotData.SidespinRPM = (*DerivedObject)->GetNumberField(TEXT("sidespin_rpm"));
    }

    return true;
}
