use crate::engine::opengl::{
    GL_ARRAY_BUFFER, GLfloat, GLuint, gl_bind_buffer, gl_bind_vertex_array, gl_buffer_data,
    gl_gen_buffer, gl_gen_vertex_array,
};

pub struct Geometry {
    vao: GLuint,
    vbo: GLuint,
}

impl Default for Geometry {
    fn default() -> Self {
        Geometry::new()
    }
}

impl Geometry {
    pub fn new() -> Self {
        let vao = gl_gen_vertex_array();
        Geometry { vao, vbo: 0 }
    }

    pub fn bind(&self) {
        gl_bind_vertex_array(self.vao)
    }

    pub fn unbind(&self) {
        gl_bind_vertex_array(0)
    }

    pub fn add_vertices(&mut self, vertices: &[GLfloat]) {
        self.vbo = gl_gen_buffer();
        gl_bind_buffer(GL_ARRAY_BUFFER, self.vbo);
        gl_buffer_data(GL_ARRAY_BUFFER, vertices);
    }
}
