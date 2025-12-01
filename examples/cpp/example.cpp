#include "opengolfcoach.h"
#include <iostream>
#include <string>
#include <iomanip>

// Simple JSON value extractor (for demonstration - use a real JSON library in production)
double extractJsonValue(const std::string& json, const std::string& key) {
    std::string searchKey = "\"" + key + "\":";
    size_t pos = json.find(searchKey);
    if (pos == std::string::npos) {
        return 0.0;
    }

    pos += searchKey.length();
    while (pos < json.length() && (json[pos] == ' ' || json[pos] == '\t')) {
        pos++;
    }

    size_t endPos = pos;
    while (endPos < json.length() &&
           (std::isdigit(json[endPos]) || json[endPos] == '.' ||
            json[endPos] == '-' || json[endPos] == 'e' || json[endPos] == 'E')) {
        endPos++;
    }

    return std::stod(json.substr(pos, endPos - pos));
}

void printSeparator() {
    std::cout << std::string(60, '=') << std::endl;
}

void printExample(int num, const std::string& description) {
    std::cout << "\nExample " << num << ": " << description << std::endl;
    printSeparator();
}

int main() {
    try {
        // Example 1: Basic shot with total spin and spin axis
        printExample(1, "Calculate carry, offline, backspin, and sidespin");

        std::string shot1 = R"({
            "ball_speed_meters_per_second": 70.0,
            "vertical_launch_angle_degrees": 12.5,
            "horizontal_launch_angle_degrees": -2.0,
            "total_spin_rpm": 2800.0,
            "spin_axis_degrees": 15.0
        })";

        std::cout << "\nInput:\n" << shot1 << std::endl;

        std::string result1 = OpenGolfCoach::calculateDerivedValues(shot1);

        std::cout << "\nOutput:\n" << result1 << std::endl;

        std::cout << "\nDerived values:" << std::endl;
        std::cout << std::fixed << std::setprecision(2);
        std::cout << "  Carry distance: "
                  << extractJsonValue(result1, "carry_distance_meters")
                  << " meters" << std::endl;
        std::cout << "  Offline distance: "
                  << extractJsonValue(result1, "offline_distance_meters")
                  << " meters" << std::endl;
        std::cout << std::setprecision(1);
        std::cout << "  Backspin: "
                  << extractJsonValue(result1, "backspin_rpm")
                  << " RPM" << std::endl;
        std::cout << "  Sidespin: "
                  << extractJsonValue(result1, "sidespin_rpm")
                  << " RPM" << std::endl;

        // Example 2: Shot with backspin and sidespin
        printExample(2, "Calculate total spin and spin axis from components");

        std::string shot2 = R"({
            "ball_speed_meters_per_second": 65.0,
            "vertical_launch_angle_degrees": 14.0,
            "horizontal_launch_angle_degrees": 1.5,
            "backspin_rpm": 3500.0,
            "sidespin_rpm": -800.0
        })";

        std::cout << "\nInput:\n" << shot2 << std::endl;

        std::string result2 = OpenGolfCoach::calculateDerivedValues(shot2);

        std::cout << "\nOutput:\n" << result2 << std::endl;

        std::cout << "\nDerived values:" << std::endl;
        std::cout << std::setprecision(1);
        std::cout << "  Total spin: "
                  << extractJsonValue(result2, "total_spin_rpm")
                  << " RPM" << std::endl;
        std::cout << std::setprecision(2);
        std::cout << "  Spin axis: "
                  << extractJsonValue(result2, "spin_axis_degrees")
                  << " degrees" << std::endl;
        std::cout << "  Carry distance: "
                  << extractJsonValue(result2, "carry_distance_meters")
                  << " meters" << std::endl;
        std::cout << "  Offline distance: "
                  << extractJsonValue(result2, "offline_distance_meters")
                  << " meters" << std::endl;

        // Example 3: Minimal input
        printExample(3, "Minimal input");

        std::string shot3 = R"({
            "ball_speed_meters_per_second": 75.0,
            "vertical_launch_angle_degrees": 11.0
        })";

        std::cout << "\nInput:\n" << shot3 << std::endl;

        std::string result3 = OpenGolfCoach::calculateDerivedValues(shot3);

        std::cout << "\nOutput:\n" << result3 << std::endl;

        std::cout << "\nDerived values:" << std::endl;
        std::cout << std::setprecision(2);
        std::cout << "  Carry distance: "
                  << extractJsonValue(result3, "carry_distance_meters")
                  << " meters" << std::endl;
        std::cout << "  Offline distance: "
                  << extractJsonValue(result3, "offline_distance_meters")
                  << " meters" << std::endl;

        // Example 4: Using C API directly
        printExample(4, "Using C API directly");

        const char* shot4 = R"({
            "ball_speed_meters_per_second": 68.0,
            "vertical_launch_angle_degrees": 13.0
        })";

        char output[8192];
        int result = calculate_derived_values_ffi(shot4, output, sizeof(output));

        if (result == 0) {
            std::cout << "\nC API result:\n" << output << std::endl;
        } else {
            std::cerr << "C API error code: " << result << std::endl;
        }

    } catch (const OpenGolfCoach::GolfCalculationError& e) {
        std::cerr << "Golf calculation error: " << e.what() << std::endl;
        return 1;
    } catch (const std::exception& e) {
        std::cerr << "Error: " << e.what() << std::endl;
        return 1;
    }

    return 0;
}
