extern crate sky_renderer;

use std::ffi::c_void;
use std::mem;

use glam::{Mat4, Vec3};

use sky_renderer::windowing::glfw::{
    GLFWwindow, glfw_create_window, glfw_poll_events, glfw_swap_buffers, glfw_terminate,
    glfw_window_should_close,
};

use sky_renderer::engine::opengl::{
    GL_ARRAY_BUFFER, GL_TRIANGLES, GL_VIEWPORT, GLboolean, GLfloat, GLsizei, gl_attach_shader,
    gl_bind_buffer, gl_bind_vertex_array, gl_buffer_data, gl_clear_color, gl_compile_shader,
    gl_create_fragment_shader, gl_create_program, gl_create_vertex_shader,
    gl_draw_arrays_instanced, gl_enable_vertex_attrib_array, gl_gen_buffer, gl_gen_vertex_array,
    gl_get_integerv, gl_link_program, gl_shader_source, gl_use_program, gl_vertex_attrib_divisor,
    gl_vertex_attrib_pointer_float, gl_viewport,
};

extern "C" fn on_viewport_resized(_window: *const GLFWwindow, width: i32, height: i32) {
    gl_viewport(0, 0, width, height);
}

fn ortho_2_d(width: f32, height: f32) -> Mat4 {
    Mat4::orthographic_rh_gl(0.0, width, height, 0.0, 0.0, 1.0)
}

fn main() {
    let window = glfw_create_window("Hello Instancing", 800, 600, Some(on_viewport_resized));

    let vertex_shader_source = "
    #version 330 core
    layout (location = 0) in vec2 aPos;
    layout (location = 1) in mat4 instanceTransform;
    void main()
    {
       gl_Position = instanceTransform * vec4(aPos, 0.0, 1.0);
    }
    ";

    let fragment_shader_source = "
    #version 330 core
    out vec4 FragColor;
    void main()
    {
        FragColor = vec4(1.0, 0.5, 0.2, 1.0);
    }
    ";

    let vertex_shader = gl_create_vertex_shader();
    gl_shader_source(vertex_shader, vertex_shader_source);
    gl_compile_shader(vertex_shader);

    let fragment_shader = gl_create_fragment_shader();
    gl_shader_source(fragment_shader, fragment_shader_source);
    gl_compile_shader(fragment_shader);

    let shader_program = gl_create_program();
    gl_attach_shader(shader_program, vertex_shader);
    gl_attach_shader(shader_program, fragment_shader);
    gl_link_program(shader_program);

    // Triangle vertices
    let vertices: Vec<GLfloat> = vec![
        -10.0, 0.0, // bottom-left
        10.0, 0.0, // bottom-right
        0.0, 10.0, // top-center
    ];

    let vao = gl_gen_vertex_array();
    gl_bind_vertex_array(vao);

    // Vertex VBO
    let vbo = gl_gen_buffer();
    gl_bind_buffer(GL_ARRAY_BUFFER, vbo);
    gl_buffer_data(GL_ARRAY_BUFFER, &vertices);

    gl_vertex_attrib_pointer_float(
        0,
        2,
        GLboolean::FALSE,
        (2 * mem::size_of::<GLfloat>()) as GLsizei,
        0,
    );
    gl_enable_vertex_attrib_array(0);

    // Instance data (100 transforms)
    let mut viewport = [0, 0, 0, 0];
    gl_get_integerv(GL_VIEWPORT, viewport.as_mut_ptr() as *mut c_void);

    let mut instance_transforms: Vec<GLfloat> = Vec::new();
    let projection = ortho_2_d(viewport[2] as f32, viewport[3] as f32);

    for i in 0..200 {
        let offset_x = (i % 10) as f32 * 20.0;
        let offset_y = (i / 10) as f32 * 20.0;

        let local_to_world = Mat4::from_scale(Vec3::new(1.0, -1.0, 1.0));
        let model = Mat4::from_translation(Vec3::new(offset_x + 30.0, offset_y + 30.0, 0.0));
        let transform = projection * model * local_to_world;

        instance_transforms.extend_from_slice(&transform.to_cols_array());
    }

    // Instance VBO
    let instance_vbo = gl_gen_buffer();
    gl_bind_buffer(GL_ARRAY_BUFFER, instance_vbo);
    gl_buffer_data(GL_ARRAY_BUFFER, &instance_transforms);

    // Bind instance matrix as 4 separate vec4 attributes (mat4 = 4 * vec4)
    // stride is the distance in bytes to the next matrix.
    let stride = (16 * mem::size_of::<GLfloat>() as GLsizei);
    for i in 0..4 {
        let location = 1 + i;
        gl_vertex_attrib_pointer_float(
            location,
            4,
            GLboolean::FALSE,
            stride,
            (i as usize * 4 * mem::size_of::<f32>()) as i32, // distance in bytes to the next column
        );
        gl_vertex_attrib_divisor(location, 1);
        gl_enable_vertex_attrib_array(location);
    }

    gl_bind_buffer(GL_ARRAY_BUFFER, 0);
    gl_bind_vertex_array(0);

    while !glfw_window_should_close(window) {
        gl_clear_color(0.2, 0.3, 0.3, 1.0);

        gl_use_program(shader_program);
        gl_bind_vertex_array(vao);

        gl_draw_arrays_instanced(GL_TRIANGLES, 0, 3, 200);

        glfw_swap_buffers(window);
        glfw_poll_events();
    }

    glfw_terminate();
}
