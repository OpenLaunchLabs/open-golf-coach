#ifndef OPENGOLFCOACH_H
#define OPENGOLFCOACH_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stddef.h>

/**
 * Calculate derived golf shot values from JSON input
 *
 * This function processes a JSON string containing golf shot parameters and
 * returns a JSON string with additional derived values.
 *
 * @param json_input - Input JSON string (null-terminated)
 * @param output_buffer - Buffer to store output JSON string
 * @param buffer_size - Size of output buffer in bytes
 * @return 0 on success, negative error code on failure:
 *         -1: Input or output pointer is null
 *         -2: Input string is not valid UTF-8
 *         -3: JSON parsing failed
 *         -4: JSON serialization failed
 *         -5: Output string conversion failed
 *         -6: Output buffer too small
 */
int calculate_derived_values_ffi(
    const char* json_input,
    char* output_buffer,
    size_t buffer_size
);

#ifdef __cplusplus
}
#endif

#ifdef __cplusplus

#include <string>
#include <stdexcept>
#include <vector>

namespace OpenGolfCoach {

/**
 * Exception thrown when golf calculation fails
 */
class GolfCalculationError : public std::runtime_error {
public:
    explicit GolfCalculationError(const std::string& message)
        : std::runtime_error(message) {}
};

/**
 * Calculate derived golf shot values (C++ wrapper)
 *
 * @param json_input - JSON string with golf shot parameters
 * @return JSON string with derived values added
 * @throws GolfCalculationError if calculation fails
 */
inline std::string calculateDerivedValues(const std::string& json_input) {
    // Allocate buffer for output (8KB should be plenty for golf shot JSON)
    std::vector<char> buffer(8192);

    int result = calculate_derived_values_ffi(
        json_input.c_str(),
        buffer.data(),
        buffer.size()
    );

    switch (result) {
        case 0:
            return std::string(buffer.data());
        case -1:
            throw GolfCalculationError("Null pointer error");
        case -2:
            throw GolfCalculationError("Input string is not valid UTF-8");
        case -3:
            throw GolfCalculationError("JSON parsing failed");
        case -4:
            throw GolfCalculationError("JSON serialization failed");
        case -5:
            throw GolfCalculationError("Output string conversion failed");
        case -6:
            throw GolfCalculationError("Output buffer too small");
        default:
            throw GolfCalculationError("Unknown error");
    }
}

} // namespace OpenGolfCoach

#endif // __cplusplus

#endif // OPENGOLFCOACH_H
