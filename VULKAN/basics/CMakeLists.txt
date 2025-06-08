cmake_minimum_required(VERSION 3.31)
project(VULKAN_TUTO VERSION 0.0.0)

set(CMAKE_C_STANDARD 99)
set(CMAKE_C_STANDARD_REQUIRED True)
set(CMAKE_C_COMPILER gcc)

configure_file(
	info.h.in
	info.h
)

add_executable(example
	src/main.c
)

target_compile_options(example PUBLIC -static)
target_compile_options(example PUBLIC -Wall)
target_compile_options(example PUBLIC -Werror)

target_include_directories(
	example
	PUBLIC
	$(CMAKE_BINARY_DIR)
)
