extern crate sky_renderer;

use sky_renderer::core::{App, Renderer, Window};
use sky_renderer::graphics2d::Drawable;

fn main() {
    let window = Window::new("Shapes", 800, 600);
    let mut app = App::new(window);

    let mut shapes = vec![
        Drawable::line(100.0, 200.0, 300.0, 250.0, 0.0, 1.0, 0.0),
        Drawable::rectangle(50.0, 50.0, 200.0, 80.0, 0.2, 0.5, 0.9),
        Drawable::rectangle(400.0, 200.0, 100.0, 50.0, 1.0, 0.0, 0.0),
        Drawable::circle(400.0, 400.0, 50.0, 0.0, 0.0, 1.0)
    ];

    let renderer = Renderer::new();

    app.on_render(move || {
        // Get current viewport size in case window was resized
        for drawable in &mut shapes {
            drawable.draw(&renderer);
        }
    });
    app.run();
}
