use crate::engine::opengl::{
    GL_ARRAY_BUFFER, GLboolean, GLenum, GLfloat, GLint, GLsizei, GLuint, gl_bind_buffer,
    gl_bind_vertex_array, gl_buffer_data, gl_enable_vertex_attrib_array, gl_gen_buffer,
    gl_gen_vertex_array, gl_vertex_attrib_pointer_float,
};

#[derive(Debug, Clone)]
pub struct Attribute {
    pub location: GLuint,
    pub size: GLint,
    pub normalize: GLboolean,
    pub stride: GLsizei,
    offset: GLsizei,
}

impl Attribute {
    pub fn new(
        location: u32,
        size: i32,
        stride_components: usize,
        offset_components: usize,
    ) -> Self {
        Self {
            location,
            size,
            normalize: GLboolean::FALSE,
            stride: (stride_components * std::mem::size_of::<GLfloat>()) as GLsizei,
            offset: (offset_components * std::mem::size_of::<GLfloat>()) as GLsizei,
        }
    }
}

pub struct Geometry {
    vao: GLuint,
    vbo: GLuint,
    vertex_count: i32,
    drawing_mode: GLenum,
    attributes: Vec<Attribute>,
}

impl Geometry {
    pub fn new(drawing_mode: GLenum) -> Self {
        let vao = gl_gen_vertex_array();
        Geometry {
            vao,
            vbo: 0,
            vertex_count: 0,
            attributes: Vec::new(),
            drawing_mode,
        }
    }

    pub fn drawing_mode(&self) -> GLenum {
        self.drawing_mode
    }

    pub fn vertex_count(&self) -> i32 {
        self.vertex_count
    }

    pub fn bind(&self) {
        gl_bind_vertex_array(self.vao)
    }

    pub fn unbind(&self) {
        gl_bind_vertex_array(0)
    }

    pub fn add_buffer(&mut self, buffer: &[GLfloat], values_per_vertex: i32) {
        self.vbo = gl_gen_buffer();
        self.vertex_count = buffer.len() as i32 / values_per_vertex;

        gl_bind_vertex_array(self.vao);
        gl_bind_buffer(GL_ARRAY_BUFFER, self.vbo);
        gl_buffer_data(GL_ARRAY_BUFFER, buffer);
        gl_bind_vertex_array(0);
    }

    pub fn add_vertex_attribute(&mut self, attribute: Attribute) {
        gl_bind_vertex_array(self.vao);

        gl_enable_vertex_attrib_array(attribute.location);
        gl_vertex_attrib_pointer_float(
            attribute.location,
            attribute.size,
            attribute.normalize,
            attribute.stride,
            attribute.offset,
        );
        gl_bind_vertex_array(0);
        self.attributes.push(attribute);
    }
}
