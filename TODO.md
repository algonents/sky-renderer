# TODO

Technical debt and improvement areas identified in code review.

## Resource Leaks

- [ ] `shader.rs:53-57` - Delete shader objects after linking (currently commented out)
- [ ] `geometry.rs:65` - Implement VAO deletion in `Drop` (currently commented out)
- [ ] Add `Drop` impl for `Shader` to delete the GL program
- [ ] Add `Drop` impl for `Window` to clean up GLFW resources

## Error Handling

- [ ] `shader.rs:24-37,48-51` - Re-enable shader compilation error checking (currently commented out, failures are silent)

## Bugs

- [ ] `shaperenderable.rs:144` - `points()` panics on empty input (accesses `points[0]` without checking)
- [ ] `shaperenderable.rs:181,183` - Remove duplicate assertion for polyline length
- [ ] `shaperenderable.rs:315-317` - Image loaded twice (once for dimensions, again in `image_with_size`)

## Incomplete Code

- [ ] `mesh.rs:54` - Move `set_uniform_4f` to renderer (comment says "needs to go into renderer!")
- [ ] `shaperenderable.rs:121` - Implement `from_shape` for all shape types (currently `unimplemented!()`)
- [ ] `geometry.rs:54` - Clean up "// NEW" WIP comment

## API Design

- [ ] `app.rs:25` - Make clear color configurable (hardcoded to `0.07, 0.13, 0.17`)
- [ ] `shaperenderable.rs:11` - Make `SCALE_FACTOR` configurable (hardcoded to `1.0`)

## Performance

### Per-Frame Overhead (High Priority)

- [ ] `renderer.rs:48,58,64,94,102,107` - Cache uniform locations after shader compilation instead of looking up by string every draw call
- [ ] `renderer.rs:45-46,91-92` - Set `gl_enable(GL_BLEND)` and `gl_blend_func` once at init, not every draw call
- [ ] `shaperenderable.rs:86` - Use cached window size from `InnerWindow` instead of calling `gl_get_integerv` every frame
- [ ] `renderer.rs:43,81` - Remove unnecessary VAO unbind between consecutive draws

### Architectural (Medium Priority)

- [ ] Implement draw call batching for rendering many shapes of the same type
- [ ] Sort draws by shader to minimize shader switches
- [ ] `shaperenderable.rs:283,290` - Scale circle/ellipse segment count based on radius/screen size (currently hardcoded 100/64)

### Memory (Low Priority)

- [ ] `geometry.rs:196-197` - Use single `gl_buffer_data` with data instead of `gl_buffer_data_empty` + `gl_buffer_sub_data`

## FFI Layer (C++)

### Bugs

- [x] `glrenderer.cpp:30-35` - Window creation continues after failure (missing `return nullptr`), will crash on `glfwMakeContextCurrent(nullptr)`
- [x] `glrenderer.cpp:40-41` - GLAD initialized twice, first call result ignored

### Design Issues

- [ ] `glrenderer.cpp:113-118` - `_glClearColor` also calls `glClear()` - surprising hidden side effect, should be separate functions
- [ ] `glrenderer.cpp:216` - Debug print on every texture upload, should be `#ifndef NDEBUG` guarded

### Missing Wrappers

- [ ] Add `_glDeleteVertexArrays` wrapper (needed for VAO cleanup)
- [ ] Add `_glDeleteShader` wrapper (needed for shader cleanup)
- [ ] Add `_glDeleteProgram` wrapper (needed for program cleanup)
- [ ] Add `_glGetShaderiv` / `_glGetShaderInfoLog` wrappers (for error reporting to Rust side)
- [ ] Add `_glGetProgramiv` / `_glGetProgramInfoLog` wrappers (for link error reporting)

## Wayland / HiDPI Scaling

### The Problem

On HiDPI displays (common on Wayland), window coordinates and framebuffer coordinates differ:

| Concept | X11 (no scaling) | Wayland @ 2x scale |
|---------|------------------|-------------------|
| Window size | 800×600 | 800×600 (logical) |
| Framebuffer size | 800×600 | 1600×1200 (physical) |
| Mouse coordinates | 0-800, 0-600 | 0-800, 0-600 (logical) |
| OpenGL viewport | 800×600 | 1600×1200 |

Mouse coordinates are in **window/logical** space, but rendering is in **framebuffer/physical** space. Mouse-to-world mapping will be off by the scale factor without correction.

### Current State

- Rendering correctly uses framebuffer size (via `glfwSetFramebufferSizeCallback` and `glfwGetFramebufferSize`)
- Mouse coordinates come in window/logical space (uncorrected)


- [ ] Bug: `shaperenderable.rs:86-87` uses framebuffer size for orthographic projection, but shapes are positioned in logical coordinates. On scaled displays, shapes appear at wrong positions/sizes.

### Required Fix

- [ ] Add `_glfwGetWindowContentScale` wrapper to query current scale factor
- [ ] Expose content scale to Rust `Window` / `WindowHandle`
- [ ] Fix `shaperenderable.rs`: divide framebuffer size by content scale for orthographic projection, so shape coordinates match logical window units

### Additional Wrappers (Lower Priority)

- [ ] Add `_glfwSetWindowContentScaleCallback` wrapper to detect scale changes (window moved between monitors)
- [ ] Document or provide helper for mouse-to-world coordinate conversion

## FFI Layer (Rust)

### Bugs

- [x] `opengl.rs:261-268` - `gl_buffer_sub_data_vec2` assumes `(f32, f32)` is tightly packed (potential UB), use `#[repr(C)]` struct instead
- [x] `opengl.rs:298` - Typo: `instance_cout` should be `instance_count`
- [x] `glfw.rs:69` - `glfw_set_window_user_pointer` takes `*const c_void` but should be `*mut c_void`

### Performance

- [x] `opengl.rs:315-317` - `gl_get_uniform_location` allocates CString on every call, contributes to per-frame heap allocations

### Dead Code

- [ ] `glfw.rs:38` - `_glfwSetFramebufferSizeCallback` declared but never used

### API Consistency

- [ ] `opengl.rs:189` - `gl_gen_buffers` takes `&mut Vec<GLuint>` instead of `&mut [GLuint]` (unusual API)

## Code Style

- [ ] Run `rustfmt` to fix inconsistent spacing (`zoom_level:1.0` vs `zoom_level: 1.0`, return arrows, etc.)
- [ ] Run `cargo clippy` and address warnings
- [ ] Clean up unused imports (`PI` imported but `TAU` used)
- [ ] Add doc comments to public API functions
