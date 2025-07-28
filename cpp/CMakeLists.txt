cmake_minimum_required(VERSION 3.27)
project(glrenderer)

set(CMAKE_CXX_STANDARD 17)

find_package(OpenGL REQUIRED)

set(GLFW_BUILD_DOCS OFF CACHE BOOL "" FORCE)
set(GLFW_BUILD_EXAMPLES OFF CACHE BOOL "" FORCE)
set(GLFW_BUILD_TESTS OFF CACHE BOOL "" FORCE)
set(BUILD_SHARED_LIBS OFF CACHE BOOL "" FORCE)

# Force only X11 support for GLFW
set(GLFW_BUILD_WAYLAND OFF CACHE BOOL "" FORCE)
set(GLFW_BUILD_X11 ON CACHE BOOL "" FORCE)

add_subdirectory(glfw-3.4)

include_directories(glad/include)
include_directories(glfw-3.4/include)

add_library(glrenderer STATIC glrenderer.cpp glad/src/glad.c)

target_link_libraries(glrenderer glfw)
target_link_libraries(glrenderer OpenGL::GL)

# Export both static libraries
set_target_properties(glfw PROPERTIES ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}")
