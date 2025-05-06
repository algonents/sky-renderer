extern crate sky_renderer;

use std::ffi::c_void;
use std::mem;

use sky_renderer::windowing::glfw::{
    GLFWwindow, glfw_create_window, glfw_poll_events, glfw_swap_buffers, glfw_terminate,
    glfw_window_should_close,
};

use sky_renderer::renderer::opengl::{
    GL_ARRAY_BUFFER, GL_TRIANGLES, GL_VIEWPORT, GLboolean, GLfloat, GLsizei, gl_attach_shader,
    gl_bind_buffer, gl_bind_vertex_array, gl_buffer_data, gl_clear_color, gl_compile_shader,
    gl_create_fragment_shader, gl_create_program, gl_create_vertex_shader, gl_draw_arrays,
    gl_enable_vertex_attrib_array, gl_gen_buffer, gl_gen_vertex_array, gl_get_integerv,
    gl_link_program, gl_shader_source, gl_use_program, gl_vertex_attrib_pointer_float, gl_viewport,
};

extern "C" fn on_viewport_resized(_window: *const GLFWwindow, width: i32, height: i32) {
    println!("Viewport resized, width: {}, height: {}", width, height);
    gl_viewport(0, 0, width, height);
}

fn main() {
    let window = glfw_create_window("Hello, Triangle", 800, 600, Some(on_viewport_resized));

    let vertex_shader_source = "
    #version 330 core
    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec3 aColor;
    out vec3 ourColor;
    void main()
    {
       gl_Position = vec4(aPos, 1.0);
       ourColor = aColor;
    }
    ";

    let fragment_shader_source = "
    #version 330 core
    in vec3 ourColor;
    out vec4 FragColor;
    void main()
    {
        FragColor = vec4(ourColor, 1.0f);
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

    let vertices: Vec<GLfloat> = vec![
        /*vertex bottom right*/ 0.5, -0.5, 0.0, /*color */ 1.0, 0.0, 0.0,
        /*vertex bottom left*/ -0.5, -0.5, 0.0, /*color */ 0.0, 1.0, 0.0,
        /*vertex top */ 0.0, 0.5, 0.0, /*color */ 0.0, 0.0, 1.0,
    ];

    let vbo = gl_gen_buffer();
    let vao = gl_gen_vertex_array();

    gl_bind_vertex_array(vao);
    gl_bind_buffer(GL_ARRAY_BUFFER, vbo);
    gl_buffer_data(GL_ARRAY_BUFFER, &vertices);

    gl_enable_vertex_attrib_array(0);
    gl_vertex_attrib_pointer_float(
        0,
        3,
        GLboolean::FALSE,
        (6 * mem::size_of::<GLfloat>()) as GLsizei,
        0,
    );

    gl_enable_vertex_attrib_array(1);
    gl_vertex_attrib_pointer_float(
        1,
        3,
        GLboolean::FALSE,
        (6 * mem::size_of::<GLfloat>()) as GLsizei,
        (3 * mem::size_of::<GLfloat>()) as GLsizei,
    );

    gl_bind_buffer(GL_ARRAY_BUFFER, 0);
    gl_bind_vertex_array(0);

    let mut viewport = [0, 0, 0, 0];

    while !glfw_window_should_close(window) {
        gl_clear_color(0.2, 0.3, 0.3, 1.0);
        gl_get_integerv(GL_VIEWPORT, viewport.as_mut_ptr() as *mut c_void);

        gl_use_program(shader_program);
        gl_bind_vertex_array(vao);

        gl_draw_arrays(GL_TRIANGLES, 0, 3);

        glfw_swap_buffers(window);
        glfw_poll_events();
    }
    glfw_terminate();
}
