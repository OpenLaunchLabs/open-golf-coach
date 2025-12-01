// Copyright Open Launch Labs. Licensed under Apache 2.0.
// Example component showing how to use OpenGolfCoach in Unreal Engine

#pragma once

#include "CoreMinimal.h"
#include "Components/ActorComponent.h"
#include "OpenGolfCoach.h"
#include "GolfShotComponent.generated.h"

/**
 * Example component that demonstrates golf shot calculations
 */
UCLASS(ClassGroup=(Custom), meta=(BlueprintSpawnableComponent))
class UGolfShotComponent : public UActorComponent
{
    GENERATED_BODY()

public:
    UGolfShotComponent();

    /**
     * Calculate and log a sample golf shot
     */
    UFUNCTION(BlueprintCallable, Category = "Golf")
    void CalculateSampleShot();

    /**
     * Calculate custom golf shot from parameters
     */
    UFUNCTION(BlueprintCallable, Category = "Golf")
    void CalculateCustomShot(
        float BallSpeed,
        float VerticalLaunch,
        float HorizontalLaunch,
        float TotalSpin,
        float SpinAxis,
        FGolfShotData& OutResult
    );

protected:
    virtual void BeginPlay() override;

public:
    // Example input values
    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Golf|Example 1")
    FGolfShotData ExampleShot1;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Golf|Example 2")
    FGolfShotData ExampleShot2;
};
