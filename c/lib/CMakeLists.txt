cmake_minimum_required(VERSION 3.13)  # CMake version check
project(simple_example)               # Create project "simple_example"
set(CMAKE_CXX_STANDARD 14)            # Enable c++14 standard

# Add main.cpp file of the project root directory as a source file
set(SOURCE_FILES uwe_c_lib.c)

# Add executable target with source files listed in SOURCE_FILES variable
add_library(uwe_c_lib ${SOURCE_FILES})
set_target_properties(uwe_c_lib PROPERTIES PUBLIC_HEADER "uwe_c_lib.h")
install(TARGETS uwe_c_lib
        PUBLIC_HEADER DESTINATION "/usr/local/include/uwe_c_lib"
        LIBRARY
        )

target_link_libraries(uwe_c_lib PUBLIC uwe_rs_lib)
target_include_directories(uwe_c_lib PUBLIC
        "/usr/local/include/uwe_rs_lib/"
)
