extern crate sky_renderer;

use sky_renderer::core::{App, Color, Window};
use sky_renderer::graphics2d::shaperenderable::ShapeRenderable;
use sky_renderer::graphics2d::svgdocument::SvgDocument;

fn main() {
    // Note: you must create a window and an app before any rendering
    let window = Window::new("Shapes", 800, 600);
    let app = App::new(window);

    let mut svg = SvgDocument::new(800.0, 600.0);


    let shapes = vec![
        ShapeRenderable::rectangle(100.0, 100.0, 120.0, 60.0, Color::from_rgb(0.2, 0.4, 0.8)),
        ShapeRenderable::circle(300.0, 300.0, 40.0, Color::from_rgb(0.0, 1.0, 0.0)),
        ShapeRenderable::line(50.0, 50.0, 150.0, 80.0, Color::from_rgb(1.0, 0.0, 0.0)),
    ];

    svg.add_shapes(&shapes);
    svg.write_to_file("target/shapes.svg").expect("Failed to write SVG");

    //app.run();

}
