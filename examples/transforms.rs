extern crate sky_renderer;

use glam::{Mat4, Vec3};
use sky_renderer::core::{App, Attribute, Geometry, Mesh, Renderer, Shader, Window};
use sky_renderer::engine::opengl::{GL_TRIANGLES, GLfloat};
use sky_renderer::graphics;

const SCALE_FACTOR: f32 = 2.0;

fn build_transform(viewport_width: f32, viewport_height: f32, time: f64) -> Mat4 {
    let scale = Mat4::from_scale(Vec3::new(SCALE_FACTOR, -SCALE_FACTOR, SCALE_FACTOR));
    let rotation = Mat4::from_rotation_z(2.0*time as f32);

    // Translate triangle to center of the screen
    let translation = Mat4::from_translation(Vec3::new(
        viewport_width / 2.0,
        viewport_height / 2.0,
        0.0,
    ));

    let projection = graphics::ortho_2d(viewport_width, viewport_height);

    projection * translation * rotation * scale
}

fn main() {
    let vertices: Vec<GLfloat> = vec![
        -10.0, 0.0,  // bottom-left
        10.0, 0.0,  // bottom-right
        0.0,  10.0, // top-center
    ];
    let values_per_vertex = 2;

    let window = Window::new("Transform", 800, 600);
    let mut geometry = Geometry::new(GL_TRIANGLES);
    geometry.add_buffer(&vertices, values_per_vertex);
    geometry.add_vertex_attribute(Attribute::new(0, 2, 2, 0));

    let vertex_shader_source = include_str!("shaders/transform.vert");
    let fragment_shader_source = include_str!("shaders/transform.frag");

    let shader = Shader::compile(vertex_shader_source, fragment_shader_source, None)
        .expect("Failed to compile shader");

    let mut mesh = Mesh::new(geometry, shader);
    let renderer = Renderer::new();
    let mut app = App::new(window);

    app.on_render(move || {
        // Get current viewport size in case window was resized
        let (width, height) = renderer.viewport_size();

        let transform = build_transform(width as f32, height as f32, renderer.get_time());
        mesh.set_transform(transform);

        renderer.draw_mesh(&mesh)
    });

    app.run();
}
