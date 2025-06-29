extern crate sky_renderer;

use sky_renderer::core::{Attribute, Geometry, Mesh, Renderer, Shader};
use sky_renderer::engine::opengl::{GL_POINTS, gl_clear_color, gl_viewport};

use sky_renderer::windowing::glfw::{
    GLFWwindow, glfw_create_window, glfw_poll_events, glfw_swap_buffers, glfw_terminate,
    glfw_window_should_close,
};

extern "C" fn on_viewport_resized(_window: *const GLFWwindow, width: i32, height: i32) {
    gl_viewport(0, 0, width, height);
}

fn main() {
    let switzerland_bounds = [5.956, 45.817, 10.492, 47.808];

    let wgs84_coordinates = vec![
        6.1432, 46.2044, // Geneva
        6.6323, 46.5197, // Lausanne
        7.4474, 46.9480, // Bern
        8.2457, 46.8959, // Sarnen
        8.5417, 47.3769, // Zurich
        9.8355, 46.4908, // St-Moritz
    ];

    let vertex_shader_source = include_str!("shaders_src/waypoints.vert");
    let fragment_shader_source = include_str!("shaders_src/waypoints.frag");

    let window = glfw_create_window("Geometry", 800, 600, Some(on_viewport_resized));

    let mut geometry = Geometry::new(GL_POINTS);

    geometry.add_buffer(&wgs84_coordinates, 2);
    geometry.add_vertex_attribute(Attribute::new(0, 2, 2 as usize, 0));

    let shader = Shader::compile(vertex_shader_source, fragment_shader_source)
        .expect("Failed to compile shader");

    let mesh = Mesh::new(geometry, shader);

    let renderer = Renderer::new();
    renderer.set_point_size(5.0);

    while !glfw_window_should_close(window) {
        gl_clear_color(0.2, 0.3, 0.3, 1.0);

        mesh.set_uniform_4f("map_bounds", &switzerland_bounds);
        renderer.draw_mesh(&mesh);

        glfw_swap_buffers(window);
        glfw_poll_events();
    }

    glfw_terminate();
}
