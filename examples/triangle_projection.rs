extern crate sky_renderer;

use std::ffi::c_void;
use std::mem;

use glam::{Mat4, Vec3};

use sky_renderer::windowing::glfw::{
    GLFWwindow, glfw_create_window, glfw_poll_events, glfw_swap_buffers, glfw_terminate,
    glfw_window_should_close,
};

use sky_renderer::renderengine::opengl::{
    GL_ARRAY_BUFFER, GL_TRIANGLES, GL_VIEWPORT, GLboolean, GLfloat, GLsizei, gl_attach_shader,
    gl_bind_buffer, gl_bind_vertex_array, gl_buffer_data, gl_clear_color, gl_compile_shader,
    gl_create_fragment_shader, gl_create_program, gl_create_vertex_shader, gl_draw_arrays,
    gl_enable_vertex_attrib_array, gl_gen_buffer, gl_gen_vertex_array, gl_get_integerv,
    gl_get_uniform_location, gl_link_program, gl_shader_source, gl_uniform_matrix_4fv,
    gl_use_program, gl_vertex_attrib_pointer_float, gl_viewport,
};

extern "C" fn on_viewport_resized(_window: *const GLFWwindow, width: i32, height: i32) {
    gl_viewport(0, 0, width, height);
}

fn ortho_2_d(width: f32, height: f32) -> Mat4 {
    Mat4::orthographic_rh_gl(0.0, width, height, 0.0, 0.0, 1.0)
}

fn main() {
    let window = glfw_create_window("Hello Projection", 800, 600, Some(on_viewport_resized));

    let vertex_shader_source = "
    #version 330 core
    layout (location = 0) in vec2 aPos;
    uniform mat4 projection;
    void main()
    {
       gl_Position = projection * vec4(aPos.x, aPos.y, 0.0, 1.0);
    }
    ";

    let fragment_shader_source = "
    #version 330 core
    out vec4 FragColor;
    void main()
    {
        FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
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
        -10.0, 0.0, // bottom-left
        10.0, 0.0, // bottom-right
        0.0, 10.0, // top-center
    ];

    let vao = gl_gen_vertex_array();
    let vbo = gl_gen_buffer();

    gl_bind_vertex_array(vao);

    gl_bind_buffer(GL_ARRAY_BUFFER, vbo);
    gl_buffer_data(GL_ARRAY_BUFFER, &vertices);

    gl_enable_vertex_attrib_array(0);
    gl_vertex_attrib_pointer_float(
        0,
        2,
        GLboolean::FALSE,
        (2 * mem::size_of::<GLfloat>()) as GLsizei,
        0,
    );

    gl_bind_buffer(GL_ARRAY_BUFFER, 0);
    gl_bind_vertex_array(0);

    let mut viewport = [0, 0, 0, 0];

    while !glfw_window_should_close(window) {
        gl_get_integerv(GL_VIEWPORT, viewport.as_mut_ptr() as *mut c_void);

        let local_to_world = Mat4::from_scale(Vec3::new(1.0, -1.0, 1.0));

        let projection = ortho_2_d(viewport[2] as f32, viewport[3] as f32);

        let model = Mat4::from_translation(glam::vec3(
            viewport[2] as f32 / 2.0,
            viewport[3] as f32 / 2.0,
            0.0,
        ));

        let transform = projection * model * local_to_world;

        gl_clear_color(0.2, 0.3, 0.3, 1.0);

        gl_use_program(shader_program);
        gl_bind_vertex_array(vao);

        let projection_location = gl_get_uniform_location(shader_program, "projection");
        gl_uniform_matrix_4fv(
            projection_location,
            1,
            GLboolean::FALSE,
            transform.to_cols_array().as_ptr(),
        );

        gl_draw_arrays(GL_TRIANGLES, 0, 3);

        glfw_swap_buffers(window);
        glfw_poll_events();
    }
    glfw_terminate();
}
