use std::rc::Rc;
use glam::Mat4;

use crate::{
    core::{geometry::Geometry, shader::Shader},
    engine::opengl::{gl_get_uniform_location, gl_uniform_4f},
};
use crate::core::color::Color;

pub struct Mesh {
    pub geometry: Geometry,
    pub shader: Rc<Shader>,
    transform: Mat4,
    pub color: Color
}

impl Mesh {
    pub fn new(shader: Rc<Shader>, geometry: Geometry, color: Color) -> Self {
        Self {
            geometry,
            shader,
            transform: Mat4::IDENTITY,
            color
        }
    }

    pub fn set_uniform_4f(&self, location: &str, vec4: &[f32; 4]) {
        let loc = gl_get_uniform_location(self.shader.program(), location);
        gl_uniform_4f(loc, vec4[0], vec4[1], vec4[2], vec4[3]);
    }

    pub fn set_transform(&mut self, transform: Mat4) {
        self.transform = transform
    }

    pub fn transform(&self) -> Mat4 {
        self.transform
    }
}
