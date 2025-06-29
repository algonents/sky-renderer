use crate::{
    core::{geometry::Geometry, shader::Shader},
    engine::opengl::{gl_get_uniform_location, gl_uniform_4f},
};

pub struct Mesh {
    pub geometry: Geometry,
    pub shader: Shader,
}

impl Mesh {
    pub fn new(geometry: Geometry, shader: Shader) -> Self {
        Self { geometry, shader }
    }

    pub fn set_uniform_4f(&self, location: &str, vec4: &[f32; 4]) {
        let loc = gl_get_uniform_location(self.shader.program(), location);
        gl_uniform_4f(loc, vec4[0], vec4[1], vec4[2], vec4[3]);
    }
}
