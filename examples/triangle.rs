extern crate sky_renderer;

use std::rc::Rc;
use sky_renderer::core::{App, Attribute, Geometry, Mesh, Renderer, Shader, Window};
use sky_renderer::engine::opengl::{GLfloat, GL_TRIANGLES};

fn main() {
    let window = Window::new("Hello Window", 800, 600);
    let mut app = App::new(window);


    let vertex_shader_source = include_str!("shaders/geometry.vert");
    let fragment_shader_source = include_str!("shaders/geometry.frag");

    /* 5 values per vertex: x,y,r,g,b */
    let vertices: Vec<GLfloat> = vec![
        /* bottom right */ 0.5, -0.5, /*color */ 1.0, 0.0, 0.0,
        /* bottom left */ -0.5, -0.5, /*color */ 0.0, 1.0, 0.0,
        /* top */ 0.0, 0.5, /*color */ 0.0, 0.0, 1.0,
    ];

    let position_values_per_vertex = 2;
    let color_values_per_vertex = 3;
    let values_per_vertex = 5;


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

    let shader = Shader::compile(vertex_shader_source, fragment_shader_source, None)
        .expect("Failed to compile shader");

    let mesh = Mesh::new(geometry, Rc::new(shader));

    let renderer = Renderer::new();

    app.on_render(move || {
        renderer.draw_mesh(&mesh)
    });
    app.run();
}
