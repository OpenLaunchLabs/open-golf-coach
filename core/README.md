# OpenGolfCoach Core Library

This is the core Rust implementation that powers all language bindings.

## Building

### WebAssembly (for JavaScript/Node.js)

```bash
# Install wasm-pack if you haven't already
cargo install wasm-pack

# Build for web
wasm-pack build --target web

# Build for Node.js
wasm-pack build --target nodejs

# Build for bundlers (webpack, etc.)
wasm-pack build --target bundler
```

### Native Library (for C++/Python/Unity/Unreal)

```bash
# Build release version
cargo build --release

# The compiled library will be in target/release/
# - libopengolfcoach.so (Linux)
# - libopengolfcoach.dylib (macOS)
# - opengolfcoach.dll (Windows)
```

### Running Tests

```bash
cargo test
```

## API

The core library provides two main interfaces:

### 1. WebAssembly Interface (for JavaScript/TypeScript)

```rust
pub fn calculate_derived_values(json_input: &str) -> Result<String, JsValue>
```

### 2. C FFI Interface (for C++/C#/Python)

```rust
pub extern "C" fn calculate_derived_values_ffi(
    json_input: *const c_char,
    output_buffer: *mut c_char,
    buffer_size: usize,
) -> i32
```

Returns:
- `0` on success
- `-1` if input/output pointers are null
- `-2` if input string is not valid UTF-8
- `-3` if JSON parsing failed
- `-4` if JSON serialization failed
- `-5` if output string conversion failed
- `-6` if output buffer is too small
