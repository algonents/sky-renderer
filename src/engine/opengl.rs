use std::ffi::{CString, c_char, c_float, c_int, c_long, c_uint, c_void};

pub type GLenum = c_uint;
pub type GLsizei = c_int;
pub type GLsizeiptr = c_long;
pub type GLchar = c_char;
pub type GLint = c_int;
pub type GLuint = c_uint;
pub type GLfloat = c_float;
pub type GLvoid = c_void;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum GLboolean {
    FALSE = 0,
    TRUE = 1,
}

pub const GL_ARRAY_BUFFER: u32 = 0x8892;
pub const GL_ELEMENT_ARRAY_BUFFER: u32 = 0x8893;

pub const GL_FRAGMENT_SHADER: u32 = 0x8B30;
pub const GL_VERTEX_SHADER: u32 = 0x8B31;
pub const GL_GEOMETRY_SHADER: u32 = 0x8DD9;

pub const GL_STATIC_DRAW: u32 = 0x88E4;
pub const GL_DYNAMIC_DRAW: u32 = 0x88E8;
pub const GL_FLOAT: u32 = 0x1406;
pub const GL_UNSIGNED_INT: u32 = 0x1405;
pub const GL_UNSIGNED_BYTE: u32 = 0x1401;
pub const GL_POINTS: u32 = 0x0000;
pub const GL_LINES: u32 = 0x0001;
pub const GL_LINE_STRIP: u32 = 0x0003;
pub const GL_TRIANGLES: u32 = 0x0004;
pub const  GL_TRIANGLE_FAN :u32=0x0006;
pub const  GL_TRIANGLE_STRIP:u32=0x0005;
pub const GL_VIEWPORT: u32 = 0x0BA2;
pub const GL_TEXTURE_2D: u32 = 0x0DE1;
pub const GL_RED: u32 = 0x1903;
pub const GL_CULL_FACE: u32 = 0x0B44;
pub const GL_BLEND: u32 = 0x0BE2;
pub const GL_SRC_ALPHA: u32 = 0x0302;
pub const GL_ONE_MINUS_SRC_ALPHA: u32 = 0x0303;
pub const GL_TEXTURE0: u32 = 0x84C0;

pub const GL_TEXTURE_WRAP_S: u32 = 0x2802;
pub const GL_TEXTURE_WRAP_T: u32 = 0x2803;
pub const GL_REPEAT: GLint = 0x2901;
pub const GL_TEXTURE_MAG_FILTER: u32 = 0x2800;
pub const GL_TEXTURE_MIN_FILTER: u32 = 0x2801;
pub const GL_LINEAR: GLint = 0x2601;
pub const GL_LINEAR_MIPMAP_LINEAR: GLint = 0x2703;
pub const GL_RGB: GLint = 0x1907;
pub const GL_RGBA: GLint = 0x1908;

unsafe extern "C" {
    fn _glClearColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
    fn _glViewPort(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
    fn _glGetIntegerv(pname: GLenum, data: *mut GLvoid);
    fn _glCreateShader(shaderType: GLenum) -> GLuint;
    fn _glShaderSource(shader: GLuint, source: *const c_char);
    fn _glCompileShader(shader: GLuint);
    fn _glCreateProgram() -> GLuint;
    fn _glAttachShader(program: GLuint, shader: GLuint);
    fn _glLinkProgram(program: GLuint);
    fn _glUseProgram(program: GLuint);
    fn _glGenBuffer() -> GLuint;
    fn _glGenBuffers(n: GLsizei, buffers: *mut GLuint);
    fn _glBindBuffer(target: GLenum, buffer: GLuint);
    fn _glBufferData(target: GLenum, size: GLsizeiptr, data: *const GLvoid, usage: GLenum);
    fn _glBufferSubData(target: GLenum, offset: GLsizeiptr, size: GLsizeiptr, data: *const GLvoid);
    fn _glGenVertexArray() -> GLuint;
    fn _glBindVertexArray(VAO: GLuint);
    fn _glVertexAttribPointer(
        index: GLuint,
        size: GLint,
        dataType: GLenum,
        normalize: GLboolean,
        stride: GLsizei,
        offset: GLsizei,
    );
    fn _glActiveTexture(unit: GLenum);
    fn _glGenTexture() -> GLuint;
    fn _glBindTexture(target: GLenum, texture: GLuint);
    fn _glTexParameteri(target: GLenum, pname: GLenum, param: GLint);
    fn _glGenerateMipmap(target: GLenum);
    fn _glTexImage2D(
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        dataType: GLenum,
        data: *const GLvoid,
    );
    fn _glEnableVertexAttribArray(index: GLuint);
    fn _glDrawArrays(mode: GLenum, first: GLint, count: GLsizei);
    fn _glDrawArraysInstanced(mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei);
    fn _glVertexAttribDivisor(index: GLuint, divisor: GLuint);
    fn _glDrawElements(mode: GLenum, count: GLsizei, element_type: GLenum, offset: GLuint);

    fn _glGetUniformLocation(program: GLuint, name: *const GLchar) -> GLint;
    fn _glUniform1f(location: GLint, v0: GLfloat);
    fn _glUniform2f(location: GLint, v0: GLfloat, v1: GLfloat);
    fn _glUniform4f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);
    fn _glUniformMatrix4fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );
    fn _glPointSize(size: GLfloat);
    fn _glEnable(cap: GLenum);
    fn _glBlendFunc(sfactor: GLenum, dfactor: GLenum);
}

pub fn gl_clear_color(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) {
    unsafe { _glClearColor(red, green, blue, alpha) }
}

pub fn gl_viewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    unsafe {
        _glViewPort(x, y, width, height);
    }
}

pub fn gl_get_integerv(pname: GLenum, data: *mut GLvoid) {
    unsafe {
        _glGetIntegerv(pname, data);
    }
}

pub fn gl_create_fragment_shader() -> GLuint {
    unsafe { _glCreateShader(GL_FRAGMENT_SHADER) }
}

pub fn gl_create_vertex_shader() -> GLuint {
    unsafe { _glCreateShader(GL_VERTEX_SHADER) }
}

pub fn gl_create_geometry_shader() -> GLuint {
    unsafe { _glCreateShader(GL_GEOMETRY_SHADER) }
}

pub fn gl_shader_source(shader: GLuint, source: &str) {
    let c_string = CString::new(source).expect("CString::new failed");
    unsafe { _glShaderSource(shader, c_string.as_ptr()) }
}

pub fn gl_compile_shader(shader: GLuint) {
    unsafe { _glCompileShader(shader) }
}

pub fn gl_create_program() -> GLuint {
    unsafe { _glCreateProgram() }
}

pub fn gl_attach_shader(program: GLuint, shader: GLuint) {
    unsafe { _glAttachShader(program, shader) }
}

pub fn gl_link_program(program: GLuint) {
    unsafe {
        _glLinkProgram(program);
    }
}

pub fn gl_use_program(program: GLuint) {
    unsafe {
        _glUseProgram(program);
    }
}

pub fn gl_gen_buffer() -> GLuint {
    unsafe { _glGenBuffer() }
}

pub fn gl_gen_buffers(buffers: &mut Vec<GLuint>) {
    unsafe {
        _glGenBuffers(buffers.len().try_into().unwrap(), buffers.as_mut_ptr());
    }
}

pub fn gl_bind_buffer(target: GLuint, buffer: GLuint) {
    unsafe {
        _glBindBuffer(target, buffer);
    }
}

pub fn gl_gen_texture() -> GLuint {
    unsafe { _glGenTexture() }
}

pub fn gl_bind_texture(target: GLenum, texture: GLuint) {
    unsafe { _glBindTexture(target, texture) }
}

pub fn gl_gen_vertex_array() -> GLuint {
    unsafe { _glGenVertexArray() }
}

pub fn gl_bind_vertex_array(array: GLuint) {
    unsafe {
        _glBindVertexArray(array);
    }
}

pub fn gl_buffer_data<T>(target: GLenum, data: &[T]) {
    unsafe {
        _glBufferData(
            target,
            std::mem::size_of_val(data) as GLsizeiptr,
            data.as_ptr() as *const GLvoid,
            GL_STATIC_DRAW,
        )
    }
}

pub fn gl_buffer_sub_data<T>(target: GLenum, offset: GLsizeiptr, data: &[T]) {
    unsafe {
        _glBufferSubData(
            target,
            offset,
            std::mem::size_of_val(data) as GLsizeiptr,
            data.as_ptr() as *const GLvoid,
        );
    }
}

pub fn gl_enable_vertex_attrib_array(index: GLuint) {
    unsafe {
        _glEnableVertexAttribArray(index);
    }
}

pub fn gl_vertex_attrib_pointer_float(
    index: GLuint,
    size: GLint,
    normalize: GLboolean,
    stride: GLsizei,
    offset: GLsizei,
) {
    unsafe {
        _glVertexAttribPointer(index, size, GL_FLOAT, normalize, stride, offset);
    }
}

pub fn gl_draw_arrays(mode: GLenum, first: GLint, count: GLsizei) {
    unsafe {
        _glDrawArrays(mode, first, count);
    }
}

pub fn gl_draw_arrays_instanced(
    mode: GLenum,
    first: GLint,
    count: GLsizei,
    instance_cout: GLsizei,
) {
    unsafe {
        _glDrawArraysInstanced(mode, first, count, instance_cout);
    }
}

pub fn gl_vertex_attrib_divisor(index: GLuint, divisor: GLuint) {
    unsafe {
        _glVertexAttribDivisor(index, divisor);
    }
}

pub fn gl_draw_elements(mode: GLenum, count: GLsizei, element_type: GLenum, offset: GLuint) {
    unsafe { _glDrawElements(mode, count, element_type, offset) }
}

pub fn gl_get_uniform_location(program: GLuint, name: &str) -> GLint {
    let c_string = CString::new(name).expect("CString::new failed");
    unsafe { _glGetUniformLocation(program, c_string.as_ptr()) }
}

pub fn gl_uniform_1f(location: GLint, v0: GLfloat) {
    unsafe {
        _glUniform1f(location, v0);
    }
}

pub fn gl_uniform_2f(location: GLint, v0: GLfloat, v1: GLfloat) {
    unsafe {
        _glUniform2f(location, v0, v1);
    }
}

pub fn gl_uniform_4f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) {
    unsafe {
        _glUniform4f(location, v0, v1, v2, v3);
    }
}

pub fn gl_uniform_matrix_4fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    unsafe {
        _glUniformMatrix4fv(location, count, transpose, value);
    }
}

pub fn gl_point_size(size: GLfloat) {
    unsafe { _glPointSize(size) }
}

pub fn gl_enable(cap: u32) {
    unsafe {
        _glEnable(cap);
    }
}

pub fn gl_blend_func(sfactor: GLenum, dfactor: GLenum) {
    unsafe { _glBlendFunc(sfactor, dfactor) }
}

pub fn gl_active_texture(unit: GLenum) {
    unsafe {
        _glActiveTexture(unit);
    }
}

pub fn gl_tex_parameteri(target: GLenum, pname: GLenum, param: GLint) {
    unsafe {
        _glTexParameteri(target, pname, param);
    }
}

pub fn gl_generate_mipmap(target: GLenum) {
    unsafe {
        _glGenerateMipmap(target);
    }
}

pub fn gl_tex_image_2d(
    target: GLenum,
    level: GLint,
    internalformat: GLint,
    width: GLsizei,
    height: GLsizei,
    border: GLint,
    format: GLenum,
    data_type: GLenum,
    data: *const GLvoid,
) {
    unsafe {
        _glTexImage2D(
            target,
            level,
            internalformat,
            width,
            height,
            border,
            format,
            data_type,
            data,
        );
    }
}
