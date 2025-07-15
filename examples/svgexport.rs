extern crate sky_renderer;

use sky_renderer::core::{App, Color, Renderable, Renderer, Window};
use sky_renderer::graphics2d::shapes::ShapeRenderable;
use sky_renderer::graphics2d::svg::SvgDocument;

fn main() {
    // Note: you must create a window and an app before any rendering
    let window = Window::new("Shapes", 800, 800);
    let mut app = App::new(window);

    let mut svg = SvgDocument::new(800.0, 800.0);


    let mut shapes = vec![
        ShapeRenderable::rectangle(100.0, 100.0, 120.0, 60.0, Color::from_rgb(0.2, 0.4, 0.8)),
        ShapeRenderable::circle(300.0, 300.0, 40.0, Color::from_rgb(0.0, 1.0, 0.0)),
        ShapeRenderable::line(50.0, 50.0, 150.0, 80.0, Color::from_rgb(1.0, 0.0, 0.0), 1.0),
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
        ShapeRenderable::points(
            &[
                (600.0, 100.0), // anchor point
                (620.0, 120.0),
                (580.0, 120.0),
            ],
            Color::from_rgb(0.0, 0.0, 1.0),
        ),
    ];

    svg.add_shapes(&shapes);
    svg.write_to_file("target/shapes.svg").expect("Failed to write SVG");

    let renderer = Renderer::new();
    renderer.set_point_size(6.0);

    app.on_render(move || {
        for shape in &mut shapes {
            shape.render(&renderer);
        }
    });
    app.run();

}
