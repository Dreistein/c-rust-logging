# Version from Ubuntu 22.04
cmake_minimum_required(VERSION 3.22.1)

# Project wide setup
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Target options
set(TARGET "sample_app")
set(OUT_DIR "${CMAKE_BINARY_DIR}/out")

# Setup project
project(${TARGET} C)

# Set paths
set(SOURCE_DIR ${CMAKE_SOURCE_DIR}/src)
set(INCLUDE_DIR ${CMAKE_SOURCE_DIR}/include)

# Source files
set(SOURCES
    ${SOURCE_DIR}/log.c
    ${SOURCE_DIR}/main.c
)

# Include directories
include_directories(${INCLUDE_DIR})

# Add the library
add_executable(log_test ${SOURCES})


target_link_libraries(log_test ${CMAKE_SOURCE_DIR}/lib/liblog_lib.a)
