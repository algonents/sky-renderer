extern crate sky_renderer;

use std::cell::RefCell;
use std::sync::Arc;
use sky_renderer::core::{App, Attribute, Geometry, Mesh, Renderer, Shader, Window};
use sky_renderer::engine::opengl::{GL_POINTS};

static SWITZERLAND_BOUNDS: [f32; 4] = [5.956, 45.817, 10.492, 47.808];

thread_local! {
    static MAP_BOUNDS: RefCell<[f32; 4]> = RefCell::new(SWITZERLAND_BOUNDS);
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

    let mut  window = Window::new("Hello, Switzerland", 800, 600);

    window.on_scroll(move |_, y_offset| {
        MAP_BOUNDS.with(|bounds| {
            let mut b = bounds.borrow_mut();

            let center_long = (b[0] + b[2]) / 2.0;
            let center_lat = (b[1] + b[3]) / 2.0;
            let zoom_factor = if y_offset > 0.0 { 0.95 } else { 1.05 };

            b[0] = center_long + (b[0] - center_long) * zoom_factor;
            b[1] = center_lat + (b[1] - center_lat) * zoom_factor;
            b[2] = center_long + (b[2] - center_long) * zoom_factor;
            b[3] = center_lat + (b[3] - center_lat) * zoom_factor;
        });
    });

    let mut app = App::new(window);


    let vertex_shader_source = include_str!("shaders/waypoints.vert");
    let fragment_shader_source = include_str!("shaders/waypoints.frag");
    let geometry_shader_source = include_str!("shaders/waypoints.geom");

    let shader = Shader::compile(
        vertex_shader_source,
        fragment_shader_source,
        Some(geometry_shader_source),
    )
    .expect("Failed to compile shader");

    let mut geometry = Geometry::new(GL_POINTS);

    geometry.add_buffer(&wgs84_coordinates, 2);
    geometry.add_vertex_attribute(Attribute::new(0, 2, 2usize, 0));

    let mesh = Mesh::new(geometry, Arc::new(shader));

    let renderer = Renderer::new();
    renderer.set_point_size(5.0);

    app.on_render(move || {
        MAP_BOUNDS.with(|bounds| {
            mesh.set_uniform_4f("map_bounds", &bounds.borrow());
        });
        renderer.draw_mesh(&mesh);
    });
    app.run();
}
