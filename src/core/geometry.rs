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

/// A GPU-backed buffer representing a drawable shape or mesh.
///
/// `Geometry` encapsulates the OpenGL resources (such as VAOs and VBOs)  and metadata required to render
/// a shape using a specific drawing mode (e.g., triangles, lines).
///
pub struct Geometry {
    vao: GLuint,
    vbo: GLuint,
    vertex_count: i32,
    drawing_mode: GLenum,
    attributes: Vec<Attribute>,
}

impl Geometry {
    /// Creates a new empty [`Geometry`] object with the specified OpenGL drawing mode.
    ///
    /// This initializes a new Vertex Array Object (VAO) and prepares an empty set of vertex attributes.
    /// The geometry is not yet ready for rendering until a Vertex Buffer Object (VBO) and vertex data
    /// are assigned.
    ///
    /// # Parameters
    /// - `drawing_mode`: The OpenGL drawing mode (e.g., `GL_TRIANGLES`, `GL_LINES`) that determines how
    ///   the vertex data will be interpreted when rendering.
    ///
    /// # Returns
    /// A new, uninitialized [`Geometry`] instance.
    ///
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

    /// Uploads vertex data to the GPU and binds it to this geometry object.
    ///
    /// This method creates a new Vertex Buffer Object (VBO), uploads the provided vertex data,
    /// and associates it with the previously created Vertex Array Object (VAO).
    ///
    /// # Parameters
    ///
    /// - `buffer`: A flat slice of vertex attribute data. The data layout must match the expected
    ///   format for the associated shader (e.g., `[x0, y0, x1, y1, ...]` for 2D positions).
    /// - `values_per_vertex`: The number of scalar values that make up one vertex
    ///   (e.g., `2` for a 2D position, `3` for a 3D position).
    ///
    /// # Notes
    ///
    /// - This method **does not define vertex attribute pointers**. You must call another method
    ///   (e.g., `add_attribute(...)`) to configure how vertex data is interpreted.
    /// - The VAO is unbound after the operation to avoid unintended side effects.
    ///
    pub fn add_buffer(&mut self, buffer: &[GLfloat], values_per_vertex: i32) {
        self.vbo = gl_gen_buffer();
        self.vertex_count = buffer.len() as i32 / values_per_vertex;

        gl_bind_vertex_array(self.vao);
        gl_bind_buffer(GL_ARRAY_BUFFER, self.vbo);
        gl_buffer_data(GL_ARRAY_BUFFER, buffer);
        gl_bind_vertex_array(0);
    }

    /// Defines a vertex attribute layout for this geometry object.
    ///
    /// This sets up how each vertex's data is interpreted in the currently bound Vertex Array Object (VAO).
    /// The attribute configuration specifies the format, stride, and offset for a particular input in the shader.
    ///
    /// # Parameters
    ///
    /// - `attribute`: An [`Attribute`] describing the layout of a single vertex attribute
    /// (e.g., position, color, texture coordinate). It includes:
    ///   - `location`: The attribute index as used in the shader (e.g., `layout(location = 0)`).
    ///   - `size`: Number of components for this attribute (e.g., 2 for vec2, 3 for vec3).
    ///   - `normalize`: Whether to normalize the values.
    ///   - `stride`: Byte offset between consecutive vertices.
    ///   - `offset`: Byte offset to the start of this attribute within the vertex.
    /// # Notes
    ///
    /// - This must be called *after* [Self::add_buffer] has uploaded vertex data.
    /// - The VAO is bound during the call and unbound afterward to preserve OpenGL state.
    /// - You can call this multiple times to add multiple attributes (e.g., position and color).
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

    
}
