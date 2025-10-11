use crate::core::engine::opengl::{GL_ARRAY_BUFFER, GLboolean, GLenum, GLfloat, GLint, GLsizei, GLuint, gl_bind_buffer, gl_bind_vertex_array, gl_buffer_data, gl_delete_buffer, gl_enable_vertex_attrib_array, gl_gen_buffer, gl_gen_vertex_array, gl_vertex_attrib_pointer_float, gl_buffer_data_empty, gl_buffer_sub_data_vec2, GLsizeiptr, gl_vertex_attrib_divisor};

#[derive(Debug, Clone)]
pub struct Attribute {
    pub location: GLuint,
    pub size: GLint,
    pub normalize: GLboolean,
    pub stride: GLsizei,
    offset: GLsizei,
    pub divisor: GLuint, // 0 = per-vertex, 1 = per-instance
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
            divisor: 0,
        }
    }

    pub fn instanced_vec2(location: u32) -> Self {
        // tightly packed vec2, divisor=1
        Self {
            location,
            size: 2,
            normalize: GLboolean::FALSE,
            stride: (2 * std::mem::size_of::<GLfloat>()) as GLsizei,
            offset: 0,
            divisor: 1,
        }
    }
}

/// A trait for types that can be converted into a [`Geometry`] object.
///
/// Types implementing this trait can produce a [`Geometry`] instance, which
/// encapsulates vertex and index data suitable for rendering with a GPU pipeline.
///
///
/// # See Also
/// - [`Geometry`]: The output type containing renderable mesh data.
pub trait GeometryProvider{
    /// Converts the implementing type into a [`Geometry`] instance.
    ///
    /// This function returns a [`Geometry`] object containing the necessary
    /// vertex and index buffers for rendering the shape.
    ///
    /// # Returns
    /// A [`Geometry`] object that encapsulates all required GPU data.
    fn to_geometry(&self) ->Geometry;
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
    // NEW
    instance_vbo: GLuint,
    instance_count: i32,
}

impl Drop for Geometry {
    fn drop(&mut self) {
        if self.vbo != 0 {
            gl_delete_buffer(self.vbo);
        }
        if self.vao != 0 {
            //gl::DeleteVertexArrays(1, &self.vao);
        }
    }
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
            instance_vbo: 0,
            instance_count: 0,
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

        gl_vertex_attrib_divisor(attribute.location, attribute.divisor);
        
        gl_bind_vertex_array(0);
        self.attributes.push(attribute);
    }

    pub fn enable_instancing_xy(&mut self, max_instances: usize) {
        if self.instance_vbo == 0 {
            self.instance_vbo = gl_gen_buffer();
        }
        gl_bind_vertex_array(self.vao);
        gl_bind_buffer(GL_ARRAY_BUFFER, self.instance_vbo);

        // Allocate empty buffer of required capacity (you can also upload a zero slice)
        let bytes = (max_instances * 2 * std::mem::size_of::<GLfloat>()) as GLsizei;
        crate::core::engine::opengl::gl_buffer_data_empty(GL_ARRAY_BUFFER, bytes as GLsizeiptr); // add this tiny helper in FFI OR upload a zero slice

        // Attribute at location=1, vec2, divisor=1
        let inst_attr = Attribute::instanced_vec2(1);
        gl_enable_vertex_attrib_array(inst_attr.location);
        gl_vertex_attrib_pointer_float(
            inst_attr.location,
            inst_attr.size,
            inst_attr.normalize,
            inst_attr.stride,
            inst_attr.offset,
        );
        gl_vertex_attrib_divisor(inst_attr.location, 1);

        gl_bind_vertex_array(0);
        gl_bind_buffer(GL_ARRAY_BUFFER, 0);
    }

    pub fn update_instance_xy(&mut self, xy: &[(f32, f32)]) {
        if self.instance_vbo == 0 { return; }
        gl_bind_vertex_array(self.vao);
        gl_bind_buffer(GL_ARRAY_BUFFER, self.instance_vbo);

        // orphan + upload (if you don’t have “empty” helper, just gl_buffer_data with slice len)
        let bytes = (xy.len() * 2 * std::mem::size_of::<GLfloat>()) as GLsizei;
        gl_buffer_data_empty(GL_ARRAY_BUFFER, bytes as GLsizeiptr);
        gl_buffer_sub_data_vec2(GL_ARRAY_BUFFER, xy); // add a helper that takes &[(f32,f32)]

        gl_bind_vertex_array(0);
        gl_bind_buffer(GL_ARRAY_BUFFER, 0);

        self.instance_count = xy.len() as i32;
    }

    pub fn clear_instancing(&mut self) {
        self.instance_count = 0;
        // keep instance_vbo for reuse
    }

    pub fn instance_count(&self) -> i32 { self.instance_count }




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
