extern crate sky_renderer;

use sky_renderer::core::{App, Color, Renderable, Renderer, Window};
use sky_renderer::graphics2d::shapes::Rectangle;
use sky_renderer::graphics2d::shapes::ShapeRenderable;

fn main() {
    let window = Window::new("Shapes", 800, 800);
    let mut app = App::new(window);

    let mut shapes = vec![
        ShapeRenderable::line(100.0, 200.0, 300.0, 250.0, Color::from_rgb(0.0, 1.0, 0.0), 5.0),
        ShapeRenderable::polyline(
            &[
                (100.0, 300.0),
                (150.0, 430.0),
                (200.0, 410.0),
                (200.0, 500.0),
            ],
            Color::from_rgb(1.0, 0.0, 0.0),
            10.0
        ),
        ShapeRenderable::rectangle(50.0, 50.0, 200.0, 80.0, Color::from_rgb(0.2, 0.5, 0.9)),
        ShapeRenderable::rectangle(400.0, 200.0, 100.0, 50.0, Color::from_rgb(1.0, 0.0, 0.0)),
        ShapeRenderable::circle(400.0, 400.0, 50.0, Color::from_rgb(0.0, 0.0, 1.0)),
        ShapeRenderable::point(600.0, 300.0, Color::from_rgb(1.0, 0.0, 0.0)),
        ShapeRenderable::points(
            &[
                (600.0, 100.0), // anchor point
                (620.0, 120.0),
                (580.0, 120.0),
            ],
            Color::from_rgb(0.0, 0.0, 1.0),
        ),
        ShapeRenderable::ellipse(600.0, 200.0, 80.0, 40.0, Color::from_rgb(0.5, 0.2, 0.8)),
        ShapeRenderable::rounded_rectangle(
            100.0,
            600.0,
            200.0,
            80.0,
            10.0,
            Color::from_rgb(0.3, 0.6, 0.9),
        ),
        ShapeRenderable::polygon(
            &[
                (600.0, 600.0),
                (575.0, 643.3),
                (525.0, 643.3),
                (500.0, 600.0),
                (525.0, 556.6),
                (575.0, 556.6),
            ],
            Color::from_rgb(1.0, 0.0, 0.0),
        ),
        ShapeRenderable::from_shape(
            600.0,
            400.0,
            Rectangle::new(100.0, 50.0),
            Color::from_rgb(0.0, 1.0, 0.0),
        ),
        ShapeRenderable::image_with_size(200.0, 300.0, "images/smiley.png", 40.0, 40.0),
        ShapeRenderable::image(400.0, 500.0, "images/bunny.png"),
    ];

    let renderer = Renderer::new();
    renderer.set_point_size(6.0);

    app.on_render(move || {
        for shape in &mut shapes {
            shape.render(&renderer);
        }
    });
    app.run();
}
