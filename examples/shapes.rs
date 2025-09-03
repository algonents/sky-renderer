extern crate sky_renderer;

use sky_renderer::core::{App, Color, Renderable, Renderer, Window};
use sky_renderer::graphics2d::shapes::Rectangle;
use sky_renderer::graphics2d::shapes::ShapeRenderable;

fn create_equilateral_triangle() -> [(f32, f32); 3] {
    let side = 20.0;
    let height = (3.0f32).sqrt() / 2.0 * side;

    let vertices = [
        (0.0, 2.0 * -height / 3.0),  // Top vertex
        (-0.5 * side, height / 3.0), // Bottom left
        (0.5 * side, height / 3.0),  // Bottom right
    ];
    vertices
}

fn generate_sine_wave(
    start_x: f32,
    start_y: f32,
    amplitude: f32,
    points: usize,
    wavelength: f32,
) -> Vec<(f32, f32)> {
    let mut result = Vec::with_capacity(points);
    let dx = wavelength / (points - 1) as f32;

    for i in 0..points {
        let x = i as f32 * dx;
        let y = amplitude * (x / wavelength * std::f32::consts::TAU).sin();
        result.push((start_x + x, start_y + y));
    }

    result
}

fn main() {
    let window = Window::new("Shapes", 800, 800);
    let mut app = App::new(window);

    let mut shapes = vec![
        ShapeRenderable::line(
            100.0,
            200.0,
            300.0,
            250.0,
            Color::from_rgb(0.0, 1.0, 0.0),
            1.0,
        ),
        ShapeRenderable::polyline(
            &[
                (100.0, 300.0),
                (150.0, 430.0),
                (200.0, 410.0),
                (200.0, 500.0),
            ],
            Color::from_rgb(1.0, 0.0, 0.0),
            10.0,
        ),
        ShapeRenderable::arc(
            (700.0, 600.0),
            70.0,
            0.0,
            std::f32::consts::PI / 2.0,
            Color::from_rgb(0.0, 0.0, 1.0),
            10.0,
            64,
        ),
        ShapeRenderable::rectangle(50.0, 50.0, 200.0, 80.0, Color::from_rgb(0.2, 0.5, 0.9)),
        ShapeRenderable::triangle(
            50.0,
            50.0,
            &create_equilateral_triangle(),
            Color::from_rgb(1.0, 0.0, 0.0),
        ),
        ShapeRenderable::rectangle(400.0, 200.0, 100.0, 50.0, Color::from_rgb(1.0, 0.0, 0.0)),
        ShapeRenderable::circle(400.0, 400.0, 50.0, Color::from_rgb(0.0, 0.0, 1.0)),
        ShapeRenderable::point(600.0, 300.0, Color::from_rgb(1.0, 0.0, 0.0)),
        ShapeRenderable::points(
            &generate_sine_wave(
                500.0, // start_x
                100.0, // start_y
                30.0,  // amplitude
                20,    // number of points
                200.0, // wavelength in pixels (horizontal length of 1 sine cycle)
            ),
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

    /* Uncomment for svg output
    let mut svg = SvgDocument::new(800.0, 800.0);
    svg.add_shapes(&shapes);
    svg.write_to_file("target/shapes.svg")
        .expect("Failed to write SVG");
    */

    let renderer = Renderer::new();
    renderer.set_point_size(6.0);

    app.on_render(move || {
        for shape in &mut shapes {
            shape.render(&renderer);
        }
    });
    app.run();
}
