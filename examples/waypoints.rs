extern crate sky_renderer;

use std::cell::RefCell;

use sky_renderer::core::{Attribute, Geometry, Mesh, Renderer, Shader};
use sky_renderer::engine::opengl::{GL_POINTS, gl_clear_color, gl_viewport};

use sky_renderer::windowing::glfw::{
    GLFWwindow, glfw_create_window, glfw_poll_events, glfw_set_scroll_callback, glfw_swap_buffers,
    glfw_terminate, glfw_window_should_close,
};

static SWITZERLAND_BOUNDS: [f32; 4] = [5.956, 45.817, 10.492, 47.808];

thread_local! {
    static MAP_BOUNDS: RefCell<[f32; 4]> = RefCell::new(SWITZERLAND_BOUNDS);
}

extern "C" fn on_viewport_resized(_window: *const GLFWwindow, width: i32, height: i32) {
    gl_viewport(0, 0, width, height);
}

extern "C" fn on_scroll(_window: *const GLFWwindow, _xoffset: f64, yoffset: f64) {
    MAP_BOUNDS.with(|bounds| {
        let mut b = bounds.borrow_mut();

        let center_long = (b[0] + b[2]) / 2.0;
        let center_lat = (b[1] + b[3]) / 2.0;
        let zoom_factor = if yoffset > 0.0 { 0.95 } else { 1.05 };

        b[0] = center_long + (b[0] - center_long) * zoom_factor;
        b[1] = center_lat + (b[1] - center_lat) * zoom_factor;
        b[2] = center_long + (b[2] - center_long) * zoom_factor;
        b[3] = center_lat + (b[3] - center_lat) * zoom_factor;
    });
}

fn main() {
    let wgs84_coordinates = vec![
        6.1432, 46.2044, // Geneva
        6.6323, 46.5197, // Lausanne
        7.4474, 46.9480, // Bern
        8.2457, 46.8959, // Sarnen
        8.5417, 47.3769, // Zurich
        9.8355, 46.4908, // St-Moritz
    ];

    let vertex_shader_source = include_str!("shaders/waypoints.vert");
    let fragment_shader_source = include_str!("shaders/waypoints.frag");
    let geometry_shader_source = include_str!("shaders/waypoints.geom");

    let window = glfw_create_window("Switzerland Waypoints", 800, 600, Some(on_viewport_resized));
    glfw_set_scroll_callback(window, Some(on_scroll));

    let mut geometry = Geometry::new(GL_POINTS);

    geometry.add_buffer(&wgs84_coordinates, 2);
    geometry.add_vertex_attribute(Attribute::new(0, 2, 2 as usize, 0));

    let shader = Shader::compile(
        vertex_shader_source,
        fragment_shader_source,
        Some(geometry_shader_source),
    )
    .expect("Failed to compile shader");

    let mesh = Mesh::new(geometry, shader);

    let renderer = Renderer::new();
    renderer.set_point_size(5.0);

    while !glfw_window_should_close(window) {
        gl_clear_color(0.2, 0.3, 0.3, 1.0);

        MAP_BOUNDS.with(|bounds| {
            mesh.set_uniform_4f("map_bounds", &bounds.borrow());
        });
        renderer.draw_mesh(&mesh);

        glfw_swap_buffers(window);
        glfw_poll_events();
    }

    glfw_terminate();
}
