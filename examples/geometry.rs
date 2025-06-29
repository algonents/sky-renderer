extern crate sky_renderer;

use sky_renderer::core::{Attribute, Geometry, Mesh, Renderer, Shader};
use sky_renderer::engine::opengl::{GL_TRIANGLES, GLfloat, gl_clear_color, gl_viewport};

use sky_renderer::windowing::glfw::{
    GLFWwindow, glfw_create_window, glfw_poll_events, glfw_swap_buffers, glfw_terminate,
    glfw_window_should_close,
};

extern "C" fn on_viewport_resized(_window: *const GLFWwindow, width: i32, height: i32) {
    gl_viewport(0, 0, width, height);
}

fn main() {
    let vertex_shader_source = "
    #version 330 core
    layout (location = 0) in vec2 aPos;
    layout (location = 1) in vec3 aColor;
    uniform mat4 transform;
    out vec3 ourColor;
    void main()
    {
       gl_Position = transform * vec4(aPos, 0.0, 1.0);
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

    /* 6 values per vertex */
    let vertices: Vec<GLfloat> = vec![
        /*vertex bottom right*/ 0.5, -0.5, /*color */ 1.0, 0.0, 0.0,
        /*vertex bottom left*/ -0.5, -0.5, /*color */ 0.0, 1.0, 0.0,
        /*vertex top */ 0.0, 0.5, /*color */ 0.0, 0.0, 1.0,
    ];

    let position_values_per_vertex = 2;
    let color_values_per_vertex = 3;
    let values_per_vertex = 5;

    let window = glfw_create_window("Geometry", 800, 600, Some(on_viewport_resized));

    let mut geometry = Geometry::new(GL_TRIANGLES);
    geometry.add_buffer(&vertices, values_per_vertex);

    let position_attribute =
        Attribute::new(0, position_values_per_vertex, values_per_vertex as usize, 0);
    geometry.add_vertex_attribute(position_attribute);

    let color_attribute = Attribute::new(
        1,
        color_values_per_vertex,
        values_per_vertex as usize,
        position_values_per_vertex as usize,
    );
    geometry.add_vertex_attribute(color_attribute);

    let shader = Shader::compile(vertex_shader_source, fragment_shader_source)
        .expect("Failed to compile shader");

    let mesh = Mesh::new(geometry, shader);

    let renderer = Renderer::new();

    while !glfw_window_should_close(window) {
        gl_clear_color(0.2, 0.3, 0.3, 1.0);

        renderer.draw_mesh(&mesh);

        glfw_swap_buffers(window);
        glfw_poll_events();
    }

    glfw_terminate();
}
