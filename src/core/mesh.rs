use crate::core::{geometry::Geometry, shader::Shader};

pub struct Mesh {
    pub geometry: Geometry,
    pub shader: Shader,
}

impl Mesh {
    pub fn new(geometry: Geometry, shader: Shader) -> Self {
        Self { geometry, shader }
    }
}
