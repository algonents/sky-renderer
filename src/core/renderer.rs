use crate::{
    core::mesh::Mesh,
    engine::opengl::{
        GLboolean, GLfloat, gl_draw_arrays, gl_get_uniform_location, gl_point_size,
        gl_uniform_matrix_4fv,
    },
};

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

    pub fn set_point_size(&self, point_size: GLfloat) {
        gl_point_size(point_size);
    }

    pub fn draw_mesh(&self, mesh: &Mesh) {
        mesh.shader.use_program();
        mesh.geometry.bind();

        let transform_loc = gl_get_uniform_location(mesh.shader.program(), "transform");
        gl_uniform_matrix_4fv(
            transform_loc,
            1,
            GLboolean::FALSE,
            mesh.transform().to_cols_array().as_ptr(),
        );

        gl_draw_arrays(
            mesh.geometry.drawing_mode(),
            0,
            mesh.geometry.vertex_count(),
        );
        mesh.geometry.unbind();
    }
}
