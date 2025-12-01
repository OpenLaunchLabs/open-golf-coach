# OpenGolfCoach - Unreal Engine Integration

Unreal Engine 5.x integration for the OpenGolfCoach library.

## Installation

### 1. Build the Rust Core Library

```bash
cd ../../core
cargo build --release
```

### 2. Create Plugin Structure in Your Unreal Project

```
YourProject/
  Plugins/
    OpenGolfCoach/
      Binaries/
        Win64/           # Windows
        Mac/             # macOS
        Linux/           # Linux
      Source/
        OpenGolfCoach/
          Public/
            OpenGolfCoach.h
          Private/
            OpenGolfCoach.cpp
          OpenGolfCoach.Build.cs
      OpenGolfCoach.uplugin
```

### 3. Copy Files

1. Copy `OpenGolfCoach.h` to `Plugins/OpenGolfCoach/Source/OpenGolfCoach/Public/`
2. Copy `OpenGolfCoach.cpp` to `Plugins/OpenGolfCoach/Source/OpenGolfCoach/Private/`
3. Copy the compiled Rust library to the appropriate Binaries folder:
   - Windows: `libopengolfcoach.dll` → `Binaries/Win64/opengolfcoach.dll`
   - macOS: `libopengolfcoach.dylib` → `Binaries/Mac/libopengolfcoach.dylib`
   - Linux: `libopengolfcoach.so` → `Binaries/Linux/libopengolfcoach.so`

### 4. Create Build Configuration

Create `OpenGolfCoach.Build.cs`:

```csharp
using UnrealBuildTool;

public class OpenGolfCoach : ModuleRules
{
    public OpenGolfCoach(ReadOnlyTargetRules Target) : base(Target)
    {
        PCHUsage = PCHUsageMode.UseExplicitOrSharedPCHs;

        PublicDependencyModuleNames.AddRange(new string[] {
            "Core",
            "CoreUObject",
            "Engine",
            "Json",
            "JsonUtilities"
        });

        // Add runtime dependencies for the native library
        string LibraryPath = "";
        if (Target.Platform == UnrealTargetPlatform.Win64)
        {
            LibraryPath = "$(PluginDir)/Binaries/Win64/opengolfcoach.dll";
        }
        else if (Target.Platform == UnrealTargetPlatform.Mac)
        {
            LibraryPath = "$(PluginDir)/Binaries/Mac/libopengolfcoach.dylib";
        }
        else if (Target.Platform == UnrealTargetPlatform.Linux)
        {
            LibraryPath = "$(PluginDir)/Binaries/Linux/libopengolfcoach.so";
        }

        if (!string.IsNullOrEmpty(LibraryPath))
        {
            RuntimeDependencies.Add(LibraryPath);
        }
    }
}
```

### 5. Create Plugin Descriptor

Create `OpenGolfCoach.uplugin`:

```json
{
    "FileVersion": 3,
    "Version": 1,
    "VersionName": "0.1.0",
    "FriendlyName": "OpenGolfCoach",
    "Description": "Golf shot calculation library",
    "Category": "Sports",
    "CreatedBy": "OpenGolfCoach Contributors",
    "CreatedByURL": "https://github.com/OpenLaunchLabs/open-golf-coach",
    "DocsURL": "",
    "MarketplaceURL": "",
    "SupportURL": "",
    "CanContainContent": false,
    "IsBetaVersion": true,
    "Installed": false,
    "Modules": [
        {
            "Name": "OpenGolfCoach",
            "Type": "Runtime",
            "LoadingPhase": "Default"
        }
    ]
}
```

## Usage

### C++ Usage

```cpp
#include "OpenGolfCoach.h"

void CalculateShot()
{
    FGolfShotData InputShot;
    InputShot.BallSpeedMeterPerSecond = 70.0f;
    InputShot.VerticalLaunchAngleDegrees = 12.5f;
    InputShot.HorizontalLaunchAngleDegrees = -2.0f;
    InputShot.TotalSpinRPM = 2800.0f;
    InputShot.SpinAxisDegrees = 15.0f;

    FGolfShotData OutputShot;
    if (UOpenGolfCoachLibrary::CalculateDerivedValues(InputShot, OutputShot))
    {
        UE_LOG(LogTemp, Log, TEXT("Carry: %.2f meters"), OutputShot.CarryDistanceMeters);
        UE_LOG(LogTemp, Log, TEXT("Offline: %.2f meters"), OutputShot.OfflineDistanceMeters);
        UE_LOG(LogTemp, Log, TEXT("Backspin: %.1f RPM"), OutputShot.BackspinRPM);
        UE_LOG(LogTemp, Log, TEXT("Sidespin: %.1f RPM"), OutputShot.SidespinRPM);
    }
}
```

### Blueprint Usage

The library is fully exposed to Blueprints:

1. Create a **Golf Shot Data** struct variable
2. Set the input values (ball speed, launch angles, etc.)
3. Call **Calculate Derived Values**
4. Access the output values (carry distance, offline, spin components)

## Example Component

See `examples/unreal/GolfShotComponent.h` for a complete component example.

## Troubleshooting

- If the library fails to load, check that the native library is in the correct Binaries folder
- Ensure the library architecture matches your Unreal Engine build (x64)
- Check the Output Log for error messages from the OpenGolfCoach module
