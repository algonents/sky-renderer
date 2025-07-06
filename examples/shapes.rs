extern crate sky_renderer;

use sky_renderer::core::{App, Renderer, Window};
use sky_renderer::graphics2d::shape::Rectangle;
use sky_renderer::graphics2d::renderable::Renderable;
use sky_renderer::graphics2d::renderable::RenderableShape;


fn main() {
    let window = Window::new("Shapes", 800, 600);
    let mut app = App::new(window);

    let mut shapes = vec![
        RenderableShape::line(100.0, 200.0, 300.0, 250.0, (0.0, 1.0, 0.0)),
        RenderableShape::rectangle(50.0, 50.0, 200.0, 80.0, (0.2, 0.5, 0.9)),
        RenderableShape::rectangle(400.0, 200.0, 100.0, 50.0, (1.0, 0.0, 0.0)),
        RenderableShape::circle(400.0, 400.0, 50.0, (0.0, 0.0, 1.0)),
        RenderableShape::from_shape(600.0, 500.0, Rectangle::new(100.0, 50.0),(0.0, 1.0, 0.0))
    ];

    let renderer = Renderer::new();

    app.on_render(move || {
        for shape in &mut shapes {
            shape.render(&renderer);
        }
    });
    app.run();
}
