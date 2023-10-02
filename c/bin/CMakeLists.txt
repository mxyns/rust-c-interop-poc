cmake_minimum_required(VERSION 3.13)  # CMake version check
project(simple_example)               # Create project "simple_example"
set(CMAKE_CXX_STANDARD 14)            # Enable c++14 standard

# Add main.cpp file of the project root directory as a source file
set(SOURCE_FILES main.c)

# Add executable target with source files listed in SOURCE_FILES variable
add_executable(uwe_c_bin ${SOURCE_FILES})

target_link_libraries(uwe_c_bin PUBLIC uwe_rs_lib)
target_include_directories(uwe_c_bin PUBLIC
        "/usr/local/include/uwe_rs_lib/"
)
