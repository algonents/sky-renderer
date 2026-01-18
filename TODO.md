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

## Code Style

- [ ] Run `rustfmt` to fix inconsistent spacing (`zoom_level:1.0` vs `zoom_level: 1.0`, return arrows, etc.)
- [ ] Run `cargo clippy` and address warnings
- [ ] Clean up unused imports (`PI` imported but `TAU` used)
- [ ] Add doc comments to public API functions
