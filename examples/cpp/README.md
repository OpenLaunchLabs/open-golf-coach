# C++ Example

This example demonstrates how to use OpenGolfCoach in a C++ application.

## Building

### 1. Build the Rust Core Library

```bash
cd ../../core
cargo build --release
cd -
```

### 2. Build the Example

```bash
mkdir build
cd build
cmake ..
make
```

## Running

```bash
./golf_example
```

## Output

The example will demonstrate:
1. Calculating carry, offline, backspin, and sidespin from total spin
2. Calculating total spin and spin axis from backspin and sidespin
3. Minimal input calculation
4. Using the C API directly

## Notes

- The example uses a simple JSON value extractor for demonstration purposes
- In production code, use a proper JSON library like:
  - [nlohmann/json](https://github.com/nlohmann/json)
  - [RapidJSON](https://github.com/Tencent/rapidjson)
  - [simdjson](https://github.com/simdjson/simdjson)

## Integration

To integrate OpenGolfCoach into your C++ project:

1. Include the header:
```cpp
#include "opengolfcoach.h"
```

2. Use the C++ API:
```cpp
std::string result = OpenGolfCoach::calculateDerivedValues(json_input);
```

3. Or use the C API:
```cpp
char output[8192];
int result = calculate_derived_values_ffi(input, output, sizeof(output));
```
