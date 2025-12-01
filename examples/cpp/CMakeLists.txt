cmake_minimum_required(VERSION 3.15)
project(OpenGolfCoach_Example)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Add the OpenGolfCoach library
add_subdirectory(../../bindings/cpp ${CMAKE_BINARY_DIR}/opengolfcoach)

# Create the example executable
add_executable(golf_example example.cpp)
target_link_libraries(golf_example PRIVATE opengolfcoach)

# Copy the library to the output directory on Windows and macOS
if(WIN32)
    add_custom_command(TARGET golf_example POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy_if_different
        "${CMAKE_SOURCE_DIR}/../../target/release/opengolfcoach.dll"
        $<TARGET_FILE_DIR:golf_example>
    )
elseif(APPLE)
    add_custom_command(TARGET golf_example POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy_if_different
        "${CMAKE_SOURCE_DIR}/../../target/release/libopengolfcoach.dylib"
        $<TARGET_FILE_DIR:golf_example>
    )
endif()
