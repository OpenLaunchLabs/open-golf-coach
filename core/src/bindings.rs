use crate::{calculate_derived_values_from_input, InputData};
use serde_json::Value;
use wasm_bindgen::prelude::*;

fn merge_existing_derived_fields(value: &mut Value) {
    if let Value::Object(map) = value {
        if let Some(open_value) = map.get("open_golf_coach").cloned() {
            if let Value::Object(open_map) = open_value {
                for (key, val) in open_map {
                    map.entry(key).or_insert(val);
                }
            }
        }
    }
}

fn prepare_input_data(value: &Value) -> Result<InputData, serde_json::Error> {
    let mut merged = value.clone();
    merge_existing_derived_fields(&mut merged);
    serde_json::from_value(merged)
}

/// WebAssembly binding for JavaScript/TypeScript
#[wasm_bindgen]
pub fn calculate_derived_values(json_input: &str) -> Result<String, JsValue> {
    // Set panic hook for better error messages in browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    // Parse input JSON
    let mut input_value: Value = serde_json::from_str(json_input)
        .map_err(|e| JsValue::from_str(&format!("Parse error: {}", e)))?;

    // Extract input data (respect any provided derived values)
    let input: InputData = prepare_input_data(&input_value)
        .map_err(|e| JsValue::from_str(&format!("Invalid input format: {}", e)))?;

    // Calculate derived values
    let derived = calculate_derived_values_from_input(&input);

    // Add derived values under "open_golf_coach" key
    if let Value::Object(ref mut map) = input_value {
        let derived_json = serde_json::to_value(&derived)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))?;
        map.insert("open_golf_coach".to_string(), derived_json);
    }

    serde_json::to_string_pretty(&input_value)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

/// C-compatible FFI function for C++/Unity/Unreal
#[no_mangle]
pub extern "C" fn calculate_derived_values_ffi(
    json_input: *const std::os::raw::c_char,
    output_buffer: *mut std::os::raw::c_char,
    buffer_size: usize,
) -> i32 {
    use std::ffi::CStr;
    use std::ffi::CString;

    if json_input.is_null() || output_buffer.is_null() {
        return -1;
    }

    let input_str = unsafe {
        match CStr::from_ptr(json_input).to_str() {
            Ok(s) => s,
            Err(_) => return -2,
        }
    };

    // Parse input JSON
    let mut input_value: Value = match serde_json::from_str(input_str) {
        Ok(v) => v,
        Err(_) => return -3,
    };

    // Extract input data (respect any provided derived values)
    let input: InputData = match prepare_input_data(&input_value) {
        Ok(i) => i,
        Err(_) => return -3,
    };

    // Calculate derived values
    let derived = calculate_derived_values_from_input(&input);

    // Add derived values under "open_golf_coach" key
    if let Value::Object(ref mut map) = input_value {
        let derived_json = match serde_json::to_value(&derived) {
            Ok(v) => v,
            Err(_) => return -4,
        };
        map.insert("open_golf_coach".to_string(), derived_json);
    }

    let output_json = match serde_json::to_string(&input_value) {
        Ok(s) => s,
        Err(_) => return -4,
    };

    let c_string = match CString::new(output_json) {
        Ok(s) => s,
        Err(_) => return -5,
    };

    let bytes = c_string.as_bytes_with_nul();
    if bytes.len() > buffer_size {
        return -6; // Buffer too small
    }

    unsafe {
        std::ptr::copy_nonoverlapping(
            bytes.as_ptr() as *const std::os::raw::c_char,
            output_buffer,
            bytes.len(),
        );
    }

    0 // Success
}
