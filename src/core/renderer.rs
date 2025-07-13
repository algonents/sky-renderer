use crate::core::engine::glfw::glfw_get_time;
use crate::core::engine::opengl::{gl_active_texture, gl_bind_texture, gl_blend_func, gl_enable, gl_get_integerv, gl_uniform_3f, GL_BLEND, GL_ONE_MINUS_SRC_ALPHA, GL_SRC_ALPHA, GL_TEXTURE0, GL_TEXTURE_2D, GL_VIEWPORT};
use crate::core::mesh::Mesh;
use std::ffi::c_void;
use crate::core::engine::opengl::{
    gl_draw_arrays, gl_get_uniform_location, gl_point_size, gl_uniform_matrix_4fv, GLboolean,
    GLfloat,
};

pub struct Renderer {}
pub trait Renderable {
    fn render(&mut self, renderer: &Renderer);
}
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

    pub fn get_time(&self) -> f64 {
        glfw_get_time()
    }

    pub fn draw_mesh(&self, mesh: &Mesh) {
        mesh.shader.use_program();
        mesh.geometry.bind();

        gl_enable(GL_BLEND);
        gl_blend_func(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);

        let transform_loc = gl_get_uniform_location(mesh.shader.program(), "transform");
        if transform_loc != -1 {
            gl_uniform_matrix_4fv(
                transform_loc,
                1,
                GLboolean::FALSE,
                mesh.transform().to_cols_array().as_ptr(),
            );
        }
        let color_loc = gl_get_uniform_location(mesh.shader.program(), "geometryColor");
        if color_loc != -1 {
            if let Some(color) = mesh.color.as_ref() {
                gl_uniform_3f(color_loc, color.red(), color.green(), color.blue());
            }
        }

        if let Some(texture_id) = mesh.texture {
            gl_active_texture(GL_TEXTURE0);
            gl_bind_texture(GL_TEXTURE_2D, texture_id);
        }

        gl_draw_arrays(
            mesh.geometry.drawing_mode(),
            0,
            mesh.geometry.vertex_count(),
        );
        mesh.geometry.unbind();
        if mesh.texture.is_some() {
            gl_bind_texture(GL_TEXTURE_2D, 0);
        }
    }
}
