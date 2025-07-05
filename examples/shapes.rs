extern crate sky_renderer;

use glam::{Mat4, Vec3};
use sky_renderer::core::{App, Attribute, Geometry, Mesh, Renderer, Shader, Window};
use sky_renderer::graphics2d;

const SCALE_FACTOR: f32 = 1.0;

fn build_transform(x: f32, y: f32, viewport_width: f32, viewport_height: f32) -> Mat4 {
    let scale = Mat4::from_scale(Vec3::splat(SCALE_FACTOR));
    let translation = Mat4::from_translation(Vec3::new(x, y, 0.0));
    let projection = graphics2d::ortho_2d(viewport_width, viewport_height);
    projection * translation * scale
}


fn main() {
    let window = Window::new("Hello, Shapes", 800, 600);
    let mut app = App::new(window);


    let vertex_shader_source = include_str!("shaders/shapes.vert");
    let fragment_shader_source = include_str!("shaders/shapes.frag");

    let rectangle = graphics2d::rectangle(200.0, 100.0, 1.0, 0.0, 0.0);

    let shader = Shader::compile(vertex_shader_source, fragment_shader_source, None)
        .expect("Failed to compile shader");

    let mut mesh = Mesh::new(rectangle, shader);

    let vertex_shader_source = include_str!("shaders/shapes.vert");
    let fragment_shader_source = include_str!("shaders/shapes.frag");

    let rectangle = graphics2d::rectangle(100.0, 50.0, 1.0, 0.0, 0.0);

    let shader = Shader::compile(vertex_shader_source, fragment_shader_source, None)
        .expect("Failed to compile shader");
    let mut mesh2 = Mesh::new(rectangle, shader);

    let circle = graphics2d::circle(60.0, 1000, 0.0, 1.0, 0.0);
    let shader = Shader::compile(vertex_shader_source, fragment_shader_source, None)
        .expect("Failed to compile shader");
    let mut mesh3 = Mesh::new(circle, shader);
    
    
    let renderer = Renderer::new();

    app.on_render(move || {
        // Get current viewport size in case window was resized
        let (width, height) = renderer.viewport_size();
        
        let transform = build_transform(50.0, 50.0, width as f32, height as f32);
        mesh.set_transform(transform);
        renderer.draw_mesh(&mesh);

        let transform = build_transform(500.0, 200.0, width as f32, height as f32);
        mesh2.set_transform(transform);
        renderer.draw_mesh(&mesh2);

        let transform = build_transform(400.0, 300.0, width as f32, height as f32);
        mesh3.set_transform(transform);
        renderer.draw_mesh(&mesh3);

    });
    app.run();
}
