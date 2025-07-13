use std::rc::Rc;
use glam::Mat4;

use crate::core::{geometry::Geometry, shader::Shader};
use crate::core::color::Color;
use crate::core::engine::opengl::{gl_get_uniform_location, gl_uniform_4f};
use crate::core::engine::opengl::GLuint;

pub struct Mesh {
    pub geometry: Geometry,
    pub shader: Rc<Shader>,
    transform: Mat4,
    pub color: Option<Color>,
    pub texture: Option<GLuint>,
}

impl Mesh {
    
    pub fn new(shader: Rc<Shader>, geometry: Geometry) -> Self {
        Self {
            geometry,
            shader,
            transform: Mat4::IDENTITY,
            color: None,
            texture: None
        }
    }
    
    pub fn with_color(shader: Rc<Shader>, geometry: Geometry, color: Option<Color>) -> Self {
        Self {
            geometry,
            shader,
            transform: Mat4::IDENTITY,
            color,
            texture: None
        }
    }
    
    pub fn with_texture(shader: Rc<Shader>, geometry: Geometry, texture: Option<GLuint>)->Self{
        Self {
            geometry,
            shader,
            transform: Mat4::IDENTITY,
            color: None,
            texture
        }
    }

    // needs to go into renderer!
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
