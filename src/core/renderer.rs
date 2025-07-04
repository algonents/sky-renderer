use std::ffi::c_void;
use crate::{
    core::mesh::Mesh,
    engine::opengl::{
        GLboolean, GLfloat, gl_draw_arrays, gl_get_uniform_location, gl_point_size,
        gl_uniform_matrix_4fv,
    },
};
use crate::engine::glfw::glfw_get_time;
use crate::engine::opengl::{gl_get_integerv, GL_VIEWPORT};

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

    pub fn viewport_size(&self) -> (i32, i32) {
        let mut viewport = [0, 0, 0, 0];
        gl_get_integerv(GL_VIEWPORT, viewport.as_mut_ptr() as *mut c_void);
        (viewport[2], viewport[3]) // width, height
    }

    pub fn get_time(&self)->f64{
        glfw_get_time()
    }

    pub fn draw_mesh(&self, mesh: &Mesh) {
        mesh.shader.use_program();
        mesh.geometry.bind();

        let transform_loc = gl_get_uniform_location(mesh.shader.program(), "transform");
        if transform_loc != -1 {
            gl_uniform_matrix_4fv(
                transform_loc,
                1,
                GLboolean::FALSE,
                mesh.transform().to_cols_array().as_ptr(),
            );
        }
        gl_draw_arrays(
            mesh.geometry.drawing_mode(),
            0,
            mesh.geometry.vertex_count(),
        );
        mesh.geometry.unbind();
    }
}
