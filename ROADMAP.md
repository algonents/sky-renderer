# Roadmap: sky-renderer Library Enhancements

Features to add to the sky-renderer library to support interactive 2D visualization applications (including **SkyTracker**, the ATM radar visualization application).

> **Note**: Domain-specific ATM features (aircraft symbols, track management, airspace boundaries, etc.) belong in the separate closed-source **SkyTracker** repository.

---

## Phase 1: Text Rendering (Critical Path)

- [ ] Integrate font rasterization (`fontdue` or `ab_glyph` crate)
- [ ] Generate font atlas texture at startup
- [ ] Store glyph metrics and UV coordinates
- [ ] Create text shader (instanced quads with texture sampling)
- [ ] Implement `Text` struct with API: `Text::new(x, y, "label", font_size, color)`
- [ ] Support text anchoring (left, center, right)
- [ ] Batch multiple text draws into single draw call

## Phase 2: Coordinate System & Projection

- [ ] Define `Projection` trait for coordinate transforms
- [ ] Implement identity projection (screen coordinates)
- [ ] World-to-screen and screen-to-world conversion functions
- [ ] Unit tests for projection accuracy

> **SkyTracker**: Stereographic projection, lat/lon viewport, nautical mile units

## Phase 3: Interaction

### Picking/Selection
- [ ] Screen-to-world coordinate conversion using projection
- [ ] Spatial index for efficient hit testing (grid or quadtree)
- [ ] Click to select/deselect entities
- [ ] Multi-select support (shift+click or box select)
- [ ] Selection callback/event system

### Pan/Zoom Controls
- [ ] Mouse drag to pan (update viewport center)
- [ ] Scroll wheel to zoom (update viewport range)
- [ ] Keyboard shortcuts (arrow keys for pan, +/- for zoom)
- [ ] Zoom-to-fit selected entities
- [ ] Min/max zoom limits

## Phase 4: Layer System

- [ ] `Layer` struct with z-order, visibility, opacity
- [ ] Layer visibility toggles
- [ ] Per-layer rendering with proper depth ordering
- [ ] Layer-based draw call batching

> **SkyTracker**: Predefined layers (background, map, routes, aircraft, labels, selection)

## Phase 5: Trail Rendering

- [ ] Circular buffer storing N past positions per entity
- [ ] Configurable trail length and decay (fade older positions)
- [ ] Efficient rendering via instancing or line strips

## Phase 6: Performance & Scalability

### Current Limitations
The current architecture uses 1 draw call per shape, which becomes a CPU bottleneck at high entity counts. For 10,000+ shapes, architectural changes are needed.

### Instancing Enhancements (High Priority)
- [ ] Per-instance rotation attribute (for oriented symbols)
- [ ] Per-instance color attribute (vec4)
- [ ] Per-instance scale attribute (vec2 or uniform float)
- [ ] Generic `InstancedShape` API supporting position + rotation + color + scale

### Draw Call Batching
- [ ] Implement `BatchRenderer` component for collecting and rendering shapes with minimal draw calls
- [ ] Batch static geometry (coastlines, airways, sectors) into single VBOs
- [ ] Group shapes by shader, issue one draw call per group
- [ ] Separate batches for static vs dynamic geometry
- [ ] `BatchRenderer` coexists with `ShapeRenderable` - opt-in for performance-critical paths

### Render State Optimization
- [ ] Cache uniform locations after shader compilation (from TODO.md)
- [ ] Set GL state (blend, depth) once at init, not per draw (from TODO.md)
- [ ] Minimize VAO binds between draws (from TODO.md)
- [ ] Sort draws by shader to reduce shader switches

### Culling
- [ ] Viewport frustum culling (skip shapes outside visible area)
- [ ] Spatial index (quadtree or grid) for efficient culling and picking

### Performance Targets
| Scenario | Target |
|----------|--------|
| 10,000 static shapes | <2ms frame time |
| 1,000 dynamic shapes @ 4Hz | <16ms frame time |
| Pan/zoom responsiveness | <16ms frame time |

### Stability
- [x] Resource cleanup on shutdown (Drop impls added)
- [ ] Error handling improvements (from TODO.md)
- [ ] Memory leak detection/validation

### Utilities
- [ ] Distance measuring tool (generic, pixel/world units)
- [ ] Screenshot/export capability
- [ ] Compass rose rendering

---

## Dependencies

External crates to evaluate:
- `fontdue` or `ab_glyph` - font rasterization for text rendering
- `image` - already used, for font atlas

## Milestones

| Milestone | Deliverable |
|-----------|-------------|
| M1 | Text rendering working |
| M2 | Projection trait, picking/selection |
| M3 | Pan/zoom controls |
| M4 | Layer system |
| M5 | Trail rendering, performance validated |

---

## Out of Scope (SkyTracker Repository)

The following features belong in the closed-source SkyTracker application:

### Radar-Specific Projections
- Stereographic projection implementation
- Viewport with lat/lon center and nautical mile range

### Aircraft Visualization
- Aircraft symbol (rotatable icon with heading)
- Velocity vector line
- Label block (callsign, altitude, speed)
- Selection state highlighting
- Coasting indicator (stale track)

### Aviation Map Elements
- Range rings with NM labels
- Waypoints with labels
- Airways/routes as labeled polylines
- Sector/airspace boundaries

### Track Management
- `TrackManager` for create/update/delete operations
- Track ID mapping
- Batch position updates
- Update rate handling (1-4 Hz)
- Track timeout/deletion

### ATM-Specific Features
- Conflict visualization (aircraft pair connecting lines)
- Altitude/speed/heading filters
- Aviation-specific distance measuring (NM)

### SkyTracker Success Criteria
- Render 500+ aircraft tracks at 4 Hz update rate
- Text labels readable at all zoom levels
- Pan/zoom responsive (<16ms frame time)
- Select track by clicking
- Show/hide layers independently
