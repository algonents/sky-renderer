extern crate sky_renderer;

use std::rc::Rc;
use sky_renderer::core::{App, Drawable, Mesh, Renderer, Window};
use sky_renderer::graphics2d;

fn main() {
    let window = Window::new("Shapes", 800, 600);
    let mut app = App::new(window);


    let shader = graphics2d::default_shader();

    let mut shapes = vec![
        Drawable::new(Mesh::new(graphics2d::rectangle(200.0, 100.0, 1.0, 0.0, 0.0), Rc::clone(&shader)), 50.0, 50.0),
        Drawable::new(Mesh::new(graphics2d::rectangle(100.0, 50.0, 0.0, 1.0, 0.0), Rc::clone(&shader)), 500.0, 200.0),
        Drawable::new(Mesh::new(graphics2d::circle(60.0, 1000, 0.0, 0.0, 1.0), Rc::clone(&shader)), 400.0, 400.0),
    ];

    let renderer = Renderer::new();

    app.on_render(move || {
        // Get current viewport size in case window was resized
        let (viewport_width, viewport_height) = renderer.viewport_size();
        for drawable in &mut shapes {
            drawable.draw(&renderer, viewport_width as f32, viewport_height as f32);
        }
    });
    app.run();
}
