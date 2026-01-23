use crate::core::engine::glfw::glfw_get_time;
use crate::core::engine::opengl::{gl_active_texture, gl_bind_texture, gl_blend_func, gl_draw_arrays_instanced, gl_enable, gl_get_integerv, gl_uniform_3f, GL_BLEND, GL_ONE_MINUS_SRC_ALPHA, GL_SRC_ALPHA, GL_TEXTURE0, GL_TEXTURE_2D, GL_VIEWPORT};
use crate::core::mesh::Mesh;
use std::ffi::c_void;
use crate::core::engine::opengl::{
    gl_draw_arrays, gl_get_uniform_location, gl_point_size, gl_uniform_matrix_4fv, GLboolean,
    GLfloat,
};
use crate::core::window::WindowHandle;

pub struct Renderer {
    pub zoom_level: f32,
    pub window_handle: WindowHandle
}
pub trait Renderable {
    fn render(&mut self, renderer: &Renderer);
}

impl Renderer {
    pub fn new(window_handle: WindowHandle) -> Self {
        Renderer {zoom_level:1.0, window_handle}
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

        let transform_loc = gl_get_uniform_location(mesh.shader.program(), "u_Transform");
        if transform_loc != -1 {
            gl_uniform_matrix_4fv(
                transform_loc,
                1,
                GLboolean::FALSE,
                mesh.transform().to_cols_array().as_ptr(),
            );
        }
        
        let offset_loc = gl_get_uniform_location(mesh.shader.program(), "u_screen_offset");
        if offset_loc != -1 {
            let (ox, oy) = mesh.screen_offset();
            crate::core::engine::opengl::gl_uniform_2f(offset_loc, ox, oy);
        }
        
        let color_loc = gl_get_uniform_location(mesh.shader.program(), "geometryColor");
        if color_loc != -1 {
            if let Some(color) = mesh.color.as_ref() {
                gl_uniform_3f(color_loc, color.red_value(), color.green_value(), color.blue_value());
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

    pub fn draw_mesh_instanced(&self, mesh: &Mesh) {
        mesh.shader.use_program();
        mesh.geometry.bind();

        gl_enable(GL_BLEND);
        gl_blend_func(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);

        let transform_loc = gl_get_uniform_location(mesh.shader.program(), "u_Transform");
        if transform_loc != -1 {
            gl_uniform_matrix_4fv(
                transform_loc, 1, GLboolean::FALSE, mesh.transform().to_cols_array().as_ptr(),
            );
        }

        // instanced path uses attribute aInstanceXY → force u_offset = (0,0)
        let off_loc = gl_get_uniform_location(mesh.shader.program(), "u_screen_offset");
        if off_loc != -1 {
            crate::core::engine::opengl::gl_uniform_2f(off_loc, 0.0, 0.0);
        }

        let color_loc = gl_get_uniform_location(mesh.shader.program(), "geometryColor");
        if color_loc != -1 {
            if let Some(color) = mesh.color.as_ref() {
                gl_uniform_3f(color_loc, color.red_value(), color.green_value(), color.blue_value());
            }
        }

        if let Some(texture_id) = mesh.texture {
            gl_active_texture(GL_TEXTURE0);
            gl_bind_texture(GL_TEXTURE_2D, texture_id);
        }

        gl_draw_arrays_instanced(
            mesh.geometry.drawing_mode(),
            0,
            mesh.geometry.vertex_count(),
            mesh.geometry.instance_count().max(0),
        );

        mesh.geometry.unbind();
        if mesh.texture.is_some() {
            gl_bind_texture(GL_TEXTURE_2D, 0);
        }
    }
}
