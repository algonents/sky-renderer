use crate::{core::mesh::Mesh, renderengine::opengl::gl_draw_arrays};

pub struct Renderer {}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {}
    }

    pub fn draw_mesh(&self, mesh: &Mesh) {
        mesh.shader.use_program();
        mesh.geometry.bind();
        gl_draw_arrays(
            mesh.geometry.drawing_mode(),
            0,
            mesh.geometry.vertex_count(),
        );
    }
}
