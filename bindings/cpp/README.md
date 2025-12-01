# OpenGolfCoach - C++ Bindings

C++ bindings for the OpenGolfCoach library.

## Building

### 1. Build the Rust Core Library

First, build the Rust library:

```bash
cd ../../core
cargo build --release
```

This will create the native library in `core/target/release/`:
- `libopengolfcoach.so` (Linux)
- `libopengolfcoach.dylib` (macOS)
- `opengolfcoach.dll` (Windows)

### 2. Build with CMake

```bash
mkdir build
cd build
cmake ..
make
```

## Usage

### C++ API

```cpp
#include "opengolfcoach.h"
#include <iostream>

int main() {
    std::string input = R"({
        "ball_speed_meters_per_second": 70.0,
        "vertical_launch_angle_degrees": 12.5,
        "total_spin_rpm": 2800.0,
        "spin_axis_degrees": 15.0
    })";

    try {
        std::string result = OpenGolfCoach::calculateDerivedValues(input);
        std::cout << result << std::endl;
    } catch (const OpenGolfCoach::GolfCalculationError& e) {
        std::cerr << "Error: " << e.what() << std::endl;
        return 1;
    }

    return 0;
}
```

### C API

```c
#include "opengolfcoach.h"
#include <stdio.h>

int main() {
    const char* input = "{"
        "\"ball_speed_meters_per_second\": 70.0,"
        "\"vertical_launch_angle_degrees\": 12.5"
    "}";

    char output[8192];
    int result = calculate_derived_values_ffi(input, output, sizeof(output));

    if (result == 0) {
        printf("%s\n", output);
    } else {
        printf("Error code: %d\n", result);
        return 1;
    }

    return 0;
}
```

## Integration in Your Project

### Option 1: CMake FetchContent

```cmake
include(FetchContent)

FetchContent_Declare(
    opengolfcoach
    GIT_REPOSITORY https://github.com/OpenLaunchLabs/open-golf-coach.git
    GIT_TAG main
)

FetchContent_MakeAvailable(opengolfcoach)

target_link_libraries(your_target PRIVATE opengolfcoach)
```

### Option 2: Manual Integration

1. Copy `opengolfcoach.h` to your include directory
2. Copy the compiled Rust library to your lib directory
3. Link against the library:

```cmake
target_link_libraries(your_target PRIVATE opengolfcoach)
```

## Error Handling

The FFI function returns error codes:
- `0`: Success
- `-1`: Null pointer error
- `-2`: Invalid UTF-8 in input
- `-3`: JSON parsing failed
- `-4`: JSON serialization failed
- `-5`: String conversion failed
- `-6`: Output buffer too small

The C++ wrapper throws `OpenGolfCoach::GolfCalculationError` exceptions.
