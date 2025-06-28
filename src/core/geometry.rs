use crate::renderengine::opengl::{
    GL_ARRAY_BUFFER, GLboolean, GLenum, GLfloat, GLint, GLsizei, GLuint, gl_bind_buffer,
    gl_bind_vertex_array, gl_buffer_data, gl_enable_vertex_attrib_array, gl_gen_buffer,
    gl_gen_vertex_array, gl_vertex_attrib_pointer_float,
};

#[derive(Debug, Clone)]
pub struct Attribute {
    pub index: GLuint,
    pub size: GLint,
    pub normalize: GLboolean,
    pub stride: GLsizei,
    offset: GLsizei,
}

impl Attribute {
    pub fn new(index: u32, size: i32, indice_stride: usize, offset: GLsizei) -> Self {
        Self {
            index,
            size,
            normalize: GLboolean::FALSE,
            stride: (indice_stride * std::mem::size_of::<GLfloat>()) as GLsizei,
            offset,
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
        gl_enable_vertex_attrib_array(attribute.index);
        gl_vertex_attrib_pointer_float(
            attribute.index,
            attribute.size,
            attribute.normalize,
            attribute.stride,
            attribute.offset,
        );
        self.attributes.push(attribute);
    }
}
