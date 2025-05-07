extern crate sky_renderer;

use std::mem;

use glam::{Mat4, Vec3};
use sky_renderer::renderer::opengl::{
    GL_ARRAY_BUFFER, GL_TRIANGLES, GLboolean, GLsizei, gl_attach_shader, gl_bind_buffer,
    gl_bind_vertex_array, gl_buffer_data, gl_clear_color, gl_compile_shader,
    gl_create_fragment_shader, gl_create_program, gl_create_vertex_shader,
    gl_draw_arrays_instanced, gl_enable_vertex_attrib_array, gl_gen_buffer, gl_gen_vertex_array,
    gl_link_program, gl_shader_source, gl_use_program, gl_vertex_attrib_divisor,
    gl_vertex_attrib_pointer_float,
};
use sky_renderer::windowing::glfw::{
    GLFWwindow, glfw_create_window, glfw_poll_events, glfw_swap_buffers, glfw_terminate,
    glfw_window_should_close,
};

const VERTEX_LOCATION: u32 = 0;
const MATRICES_LOCATION: u32 = 1;
const NUM_INSTANCES: usize = 1000;

extern "C" fn on_viewport_resized(_window: *const GLFWwindow, width: i32, height: i32) {
    sky_renderer::renderer::opengl::gl_viewport(0, 0, width, height);
}

fn main() {
    let window = glfw_create_window("Instanced Rendering", 600, 600, Some(on_viewport_resized));

    // Vertex Shader
    let vertex_shader_source = "
            #version 330 core
            layout (location = 0) in vec3 Vertex;
            layout (location = 1) in mat4 Matrix;
            void main() {
                gl_Position = Matrix * vec4(Vertex, 1.0);
            }
        ";

    // Fragment Shader
    let fragment_shader_source = "
            #version 330 core
            out vec4 FragColor;
            void main() {
                FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
            }
        ";

    // Compile Shaders
    let vertex_shader = gl_create_vertex_shader();
    gl_shader_source(vertex_shader, vertex_shader_source);
    gl_compile_shader(vertex_shader);

    let fragment_shader = gl_create_fragment_shader();
    gl_shader_source(fragment_shader, fragment_shader_source);
    gl_compile_shader(fragment_shader);

    // Create Shader Program
    let shader_program = gl_create_program();
    gl_attach_shader(shader_program, vertex_shader);
    gl_attach_shader(shader_program, fragment_shader);
    gl_link_program(shader_program);
    gl_use_program(shader_program);

    // Triangle Vertex Data
    let vertices: [f32; 9] = [
        -0.3, -0.3, 0.0, // Left
        0.3, -0.3, 0.0, // Right
        0.0, 0.25, 0.0, // Top
    ];

    // Generate VAO and VBO
    let vao = gl_gen_vertex_array();
    gl_bind_vertex_array(vao);

    let vbo = gl_gen_buffer();
    gl_bind_buffer(GL_ARRAY_BUFFER, vbo);
    gl_buffer_data(GL_ARRAY_BUFFER, &vertices);

    gl_vertex_attrib_pointer_float(
        VERTEX_LOCATION,
        3,
        GLboolean::FALSE,
        (3 * mem::size_of::<f32>()) as GLsizei,
        0,
    );
    gl_enable_vertex_attrib_array(VERTEX_LOCATION);

    // Generate Transformation Matrices
    let mut matrices: Vec<f32> = Vec::with_capacity(NUM_INSTANCES * 16);
    for i in 0..NUM_INSTANCES {
        let angle = 40.0 * std::f32::consts::PI * i as f32 / NUM_INSTANCES as f32;
        let pos_x = 0.002 * i as f32 * angle.cos();
        let pos_y = 0.002 * i as f32 * angle.sin();
        let scale = 0.0004 * i as f32;

        let transform = Mat4::from_translation(Vec3::new(pos_x, pos_y, 0.0))
            * Mat4::from_scale(Vec3::splat(scale));

        matrices.extend_from_slice(&transform.to_cols_array());
    }

    // Generate MBO
    let mbo = gl_gen_buffer();
    gl_bind_buffer(GL_ARRAY_BUFFER, mbo);
    gl_buffer_data(GL_ARRAY_BUFFER, &matrices);

    // Set up vertex attribute pointers for matrix (4 vec4 attributes)
    let stride = (16 * mem::size_of::<f32>()) as GLsizei;
    for i in 0..4 {
        gl_enable_vertex_attrib_array(MATRICES_LOCATION + i);
        gl_vertex_attrib_pointer_float(
            MATRICES_LOCATION + i,
            4,
            GLboolean::FALSE,
            stride,
            (i as usize * 4 * mem::size_of::<f32>()) as i32,
        );
        gl_vertex_attrib_divisor(MATRICES_LOCATION + i, 1);
    }

    // Unbind VAO
    gl_bind_vertex_array(0);

    // Render Loop
    while !glfw_window_should_close(window) {
        gl_clear_color(0.2, 0.3, 0.3, 1.0);
        gl_use_program(shader_program);
        gl_bind_vertex_array(vao);

        gl_draw_arrays_instanced(GL_TRIANGLES, 0, 3, NUM_INSTANCES as i32);

        glfw_swap_buffers(window);
        glfw_poll_events();
    }

    glfw_terminate();
}
