# Roadmap: Air Traffic Management Radar Visualization MVP

Target: 6-month MVP for real-time radar map visualization for air traffic management.

## Phase 1: Core Infrastructure (Month 1-2)

### Text Rendering (Critical Path)
- [ ] Integrate font rasterization (`fontdue` or `ab_glyph` crate)
- [ ] Generate font atlas texture at startup
- [ ] Store glyph metrics and UV coordinates
- [ ] Create text shader (instanced quads with texture sampling)
- [ ] Implement `Text` struct with API: `Text::new(x, y, "AWY123", font_size, color)`
- [ ] Support text anchoring (left, center, right)
- [ ] Batch multiple text draws into single draw call

### Coordinate System
- [ ] Define `Projection` trait for coordinate transforms
- [ ] Implement stereographic projection (common for radar)
- [ ] Create `Viewport` struct with center (lat/lon) and range (nautical miles)
- [ ] World-to-screen and screen-to-world conversion functions
- [ ] Unit tests for projection accuracy

## Phase 2: Radar Primitives (Month 2-3)

### Aircraft Symbol
- [ ] Rotatable aircraft icon (triangle/chevron) with heading
- [ ] Velocity vector line (speed/heading indicator)
- [ ] Label block positioned relative to symbol (callsign, altitude, speed)
- [ ] Selection state with visual highlight
- [ ] Coasting indicator (stale track)

### Map Elements
- [ ] Range rings at configurable intervals (use existing `arc`)
- [ ] Waypoints as named points with labels
- [ ] Airways/routes as polylines with optional labels
- [ ] Sector/airspace boundaries as polygons with fill and stroke
- [ ] Compass rose or north indicator

### Trail History
- [ ] Circular buffer storing N past positions per track
- [ ] Configurable trail length and decay (fade older positions)
- [ ] Efficient rendering via instancing or line strips

## Phase 3: Interaction (Month 3-4)

### Picking/Selection
- [ ] Screen-to-world coordinate conversion using projection
- [ ] Spatial index for efficient hit testing (grid or quadtree)
- [ ] Click to select/deselect track
- [ ] Multi-select support (shift+click or box select)
- [ ] Selection callback/event system

### Pan/Zoom Controls
- [ ] Mouse drag to pan (update viewport center)
- [ ] Scroll wheel to zoom (update viewport range)
- [ ] Keyboard shortcuts (arrow keys for pan, +/- for zoom)
- [ ] Zoom-to-fit selected tracks
- [ ] Min/max zoom limits

## Phase 4: Data Integration (Month 4-5)

### Track Management
- [ ] `TrackManager` struct for create/update/delete operations
- [ ] Track ID to internal handle mapping
- [ ] Batch position updates via instancing
- [ ] Configurable update rate handling (1-4 Hz typical radar)
- [ ] Track timeout/deletion for lost targets

### Layer System
- [ ] `Layer` struct with z-order, visibility, opacity
- [ ] Predefined layers: background, map, routes, aircraft, labels, selection
- [ ] Layer visibility toggles
- [ ] Per-layer rendering with proper depth ordering

## Phase 5: Polish & Performance (Month 5-6)

### Performance Optimization
- [ ] Cache uniform locations (from TODO.md)
- [ ] Set GL state once at init, not per draw (from TODO.md)
- [ ] Batch draw calls by shader/layer
- [ ] Profile with 500+ tracks at 4 Hz update rate
- [ ] Memory usage optimization for long-running sessions

### Additional Features
- [ ] Conflict visualization (connecting lines between aircraft pairs)
- [ ] Altitude filter (show only FL200-FL400, etc.)
- [ ] Speed/heading filter
- [ ] Distance measuring tool (click two points)
- [ ] Screenshot/export capability

### Stability
- [ ] Error handling improvements (from TODO.md)
- [ ] Resource cleanup on shutdown (from TODO.md)
- [ ] Integration tests with simulated track data
- [ ] Memory leak detection

---

## Dependencies

External crates to evaluate:
- `fontdue` or `ab_glyph` - font rasterization for text rendering
- `image` - already used, for font atlas

## Milestones

| Milestone | Target | Deliverable |
|-----------|--------|-------------|
| M1 | End of Month 2 | Text rendering working, basic projection |
| M2 | End of Month 3 | Aircraft symbols with labels, map elements |
| M3 | End of Month 4 | Interactive pan/zoom/select |
| M4 | End of Month 5 | Live track updates, layer system |
| M5 | End of Month 6 | Performance validated, MVP complete |

## Success Criteria for MVP

- [ ] Render 500+ aircraft tracks at 4 Hz update rate without frame drops
- [ ] Text labels readable at all zoom levels
- [ ] Pan/zoom responsive (<16ms frame time)
- [ ] Select track by clicking
- [ ] Show/hide layers independently
- [ ] Range rings and sector boundaries visible
