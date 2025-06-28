use crate::{core::geometry::Geometry, renderengine::opengl::gl_draw_arrays};

pub struct Renderer {}

impl Renderer {
    pub fn draw_geometry(&self, geometry: &Geometry) {
        geometry.bind();
        gl_draw_arrays(geometry.drawing_mode(), 0, geometry.vertex_count());
    }
}
