cmake_minimum_required(VERSION 3.27)
project(glrenderer)

set(CMAKE_CXX_STANDARD 17)

find_package(OpenGL REQUIRED)
find_package(glfw3 REQUIRED)

include_directories(glad/include)
add_library(glrenderer STATIC glrenderer.cpp glad/src/glad.c)


# Add GLFW include directories
include_directories(${GLFW_INCLUDE_DIR})



target_link_libraries(glrenderer glfw)
target_link_libraries(glrenderer OpenGL::GL)
