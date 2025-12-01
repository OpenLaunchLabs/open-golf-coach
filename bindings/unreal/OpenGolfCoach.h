// Copyright Open Launch Labs. Licensed under Apache 2.0.

#pragma once

#include "CoreMinimal.h"
#include "Kismet/BlueprintFunctionLibrary.h"
#include "OpenGolfCoach.generated.h"

/**
 * Golf shot data structure
 */
USTRUCT(BlueprintType)
struct FGolfShotData
{
    GENERATED_BODY()

    UPROPERTY(BlueprintReadWrite, EditAnywhere, Category = "Golf")
    float BallSpeedMetersPerSecond = 0.0f;

    UPROPERTY(BlueprintReadWrite, EditAnywhere, Category = "Golf")
    float VerticalLaunchAngleDegrees = 0.0f;

    UPROPERTY(BlueprintReadWrite, EditAnywhere, Category = "Golf")
    float HorizontalLaunchAngleDegrees = 0.0f;

    UPROPERTY(BlueprintReadWrite, EditAnywhere, Category = "Golf")
    float TotalSpinRPM = 0.0f;

    UPROPERTY(BlueprintReadWrite, EditAnywhere, Category = "Golf")
    float SpinAxisDegrees = 0.0f;

    UPROPERTY(BlueprintReadWrite, EditAnywhere, Category = "Golf")
    float CarryDistanceMeters = 0.0f;

    UPROPERTY(BlueprintReadWrite, EditAnywhere, Category = "Golf")
    float OfflineDistanceMeters = 0.0f;

    UPROPERTY(BlueprintReadWrite, EditAnywhere, Category = "Golf")
    float BackspinRPM = 0.0f;

    UPROPERTY(BlueprintReadWrite, EditAnywhere, Category = "Golf")
    float SidespinRPM = 0.0f;
};

/**
 * Blueprint function library for golf shot calculations
 */
UCLASS()
class UOpenGolfCoachLibrary : public UBlueprintFunctionLibrary
{
    GENERATED_BODY()

public:
    /**
     * Calculate derived golf shot values
     *
     * @param ShotData - Input golf shot data (only some fields need to be filled)
     * @param OutShotData - Output with all derived values calculated
     * @return true if calculation succeeded, false otherwise
     */
    UFUNCTION(BlueprintCallable, Category = "Golf")
    static bool CalculateDerivedValues(const FGolfShotData& ShotData, FGolfShotData& OutShotData);

private:
    // FFI function from Rust library
    typedef int (*FCalculateDerivedValuesFn)(const char*, char*, size_t);

    static void* LibraryHandle;
    static FCalculateDerivedValuesFn CalculateFunction;

    static bool LoadLibrary();
    static void UnloadLibrary();
};
