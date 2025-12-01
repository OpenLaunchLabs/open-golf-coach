cmake_minimum_required(VERSION 3.15)
project(OpenGolfCoach)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Determine the library extension based on platform
if(WIN32)
    set(LIB_EXT "dll")
    set(LIB_PREFIX "")
elseif(APPLE)
    set(LIB_EXT "dylib")
    set(LIB_PREFIX "lib")
else()
    set(LIB_EXT "so")
    set(LIB_PREFIX "lib")
endif()

# Path to the Rust library (workspace builds to root target/)
set(RUST_LIB_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../../target/release")
set(RUST_LIB "${RUST_LIB_DIR}/${LIB_PREFIX}opengolfcoach.${LIB_EXT}")

# Check if library exists, if not, provide build instructions
if(NOT EXISTS ${RUST_LIB})
    message(WARNING "Rust library not found at ${RUST_LIB}")
    message(WARNING "Please build the Rust library first:")
    message(WARNING "  cd ../../core")
    message(WARNING "  cargo build --release")
endif()

# Create an interface library that links to the Rust library
add_library(opengolfcoach_rust SHARED IMPORTED)
set_target_properties(opengolfcoach_rust PROPERTIES
    IMPORTED_LOCATION ${RUST_LIB}
)

# Create header-only interface library for C++ wrapper
add_library(opengolfcoach INTERFACE)
target_include_directories(opengolfcoach INTERFACE ${CMAKE_CURRENT_SOURCE_DIR})
target_link_libraries(opengolfcoach INTERFACE opengolfcoach_rust)

# For systems that need to link pthread and dl
if(UNIX AND NOT APPLE)
    target_link_libraries(opengolfcoach INTERFACE pthread dl)
endif()

# Install targets
install(FILES opengolfcoach.h DESTINATION include)
install(FILES ${RUST_LIB} DESTINATION lib)
