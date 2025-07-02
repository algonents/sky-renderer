extern crate sky_renderer;

use glam::{Mat4, Vec3};
use sky_renderer::core::{App, Attribute, Geometry, Mesh, Renderer, Shader, Window};
use sky_renderer::engine::opengl::{
    GL_TRIANGLES, GLfloat
};

fn ortho_2_d(width: f32, height: f32) -> Mat4 {
    Mat4::orthographic_rh_gl(0.0, width, height, 0.0, 0.0, 1.0)
}

fn main() {
    let vertex_shader_source = include_str!("shaders/transform.vert");
    let fragment_shader_source = include_str!("shaders/transform.frag");

    let vertices: Vec<GLfloat> = vec![
        -10.0, 0.0, // bottom-left
        10.0, 0.0, // bottom-right
        0.0, 10.0, // top-center
    ];
    let values_per_vertex = 2;

    let window = Window::new("Transform", 800, 600);

    let mut geometry = Geometry::new(GL_TRIANGLES);
    geometry.add_buffer(&vertices, values_per_vertex);

    geometry.add_vertex_attribute(Attribute::new(0, 2, 2, 0));

    let shader = Shader::compile(vertex_shader_source, fragment_shader_source, None)
        .expect("Failed to compile shader");

    let mut mesh = Mesh::new(geometry, shader);

    let renderer = Renderer::new();

    let mut app = App::new(window);

    app.on_render(move || {
       // let view_port = window.viewport_size();
        let local_to_world = Mat4::from_scale(Vec3::new(1.3, -1.3, 1.3));

        // define orthographic projection using the viewport size
        let viewport_size = renderer.viewport_size();
        let ortho_2d_projection = ortho_2_d(viewport_size.0 as f32, viewport_size.1 as f32);

        // translate triangle to center of the screen
        let translation = Mat4::from_translation(glam::vec3(
            viewport_size.0 as f32 / 2.0,
            viewport_size.1 as f32 / 2.0,
            0.0,
        ));

        let transform = ortho_2d_projection * translation * local_to_world;

        mesh.set_transform(transform);

        renderer.draw_mesh(&mesh)
    });

    app.run();
}
