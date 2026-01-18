# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

sky-renderer is a minimalist 2D graphics engine written in Rust with native OpenGL bindings. It provides rendering of 2D shapes and visualization of 2-dimensional data in real time. The project uses OpenGL 3.3 Core Profile with GLFW 3.4 (bundled) for window management.

## Build Commands

```bash
# Build the library
cargo build

# Run examples
cargo run --example shapes
cargo run --example shapes_with_zoom
cargo run --example triangle
cargo run --example instancing

# Run standalone example projects
cd examples/bouncing_balls && cargo run
```

### Build Requirements

- C/C++ compiler and CMake (cmake crate invokes CMake during build)
- Linux: `libgl1-mesa-dev`, `libwayland-dev`, `libxkbcommon-dev`, `xorg-dev`
- GLFW 3.4 is bundled, no external dependency needed

## Architecture

### Three-Layer Design

1. **FFI Layer** (`src/core/engine/`)
   - `opengl.rs`: Raw OpenGL function bindings
   - `glfw.rs`: GLFW window and input wrappers
   - C++ implementation in `cpp/glrenderer.cpp` provides the actual wrappers that link to GLFW and OpenGL

2. **Core Rendering Engine** (`src/core/`)
   - `app.rs`: Main application loop with render callback
   - `renderer.rs`: Mesh drawing, viewport management, zoom
   - `window.rs`: GLFW window creation, event callbacks (resize, scroll, cursor)
   - `geometry.rs`: VAO/VBO management, vertex attributes, instancing support
   - `mesh.rs`: Combines geometry + shader + transform + color/texture
   - `shader.rs`: GLSL compilation wrapper
   - `texture.rs` / `image.rs`: Image loading and GPU texture management

3. **Graphics2D API** (`src/graphics2d/`)
   - `shapes/shaperenderable.rs`: High-level shape rendering (line, polyline, circle, rectangle, polygon, arc, image, etc.)
   - Uses lazy-loaded singleton shaders via thread_local OnceCell
   - Orthographic projection with zoom support

### Key Patterns

- **FFI Wrapper Pattern**: Safe Rust wrappers around C/C++ functions in `src/core/engine/`
- **Interior Mutability**: Window uses `Rc<Cell<>>` for shared state across callbacks
- **Component-Based Meshes**: Mesh = Geometry + Shader + Transform
- **Callback-Driven App Loop**: App uses closures for render logic

### C++ FFI Build

`build.rs` uses CMake to compile the C++ layer (`cpp/`). Platform-specific linking:
- Linux: Statically links glrenderer, glfw3; dynamically links GL
- macOS: Links Cocoa, CoreFoundation, IOKit, CoreVideo frameworks
- Windows: Links opengl32, gdi32, user32, shell32

## Key Files

- `src/lib.rs`: Library root, exports `core` and `graphics2d` modules
- `src/core/geometry.rs`: VAO/VBO management and instancing setup
- `src/graphics2d/shapes/shaperenderable.rs`: Main shape rendering implementation (~800 lines)
- `cpp/glrenderer.cpp`: C++ wrapper functions called via FFI
- `build.rs`: CMake integration and platform-specific linking

## Supported Shape Types

Point, MultiPoint, Line, Polyline, Arc, Triangle, Rectangle, RoundedRectangle, Circle, Ellipse, Polygon, Image

## Platform Notes

- Uses X11 only on Linux (Wayland disabled in CMakeLists.txt to avoid scaling issues)
- OpenGL 3.3 Core Profile for macOS compatibility
- MSAA 4x multisampling enabled by default
