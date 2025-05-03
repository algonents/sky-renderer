cmake_minimum_required(VERSION 3.27)
project(glrenderer)

set(CMAKE_CXX_STANDARD 17)

find_package(OpenGL REQUIRED)

include_directories(glad/include)

add_library(glrenderer SHARED glrenderer.cpp glad/src/glad.c)

target_link_libraries(glrenderer glfw)
target_link_libraries(glrenderer OpenGL::GL)
