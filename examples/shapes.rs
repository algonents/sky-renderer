extern crate sky_renderer;

use sky_renderer::core::{App, Color, Renderable, Renderer, Window};
use sky_renderer::graphics2d::shapes::{
    Arc, Circle, Ellipse, Line, MultiPoint, Polygon, Polyline, Rectangle, RoundedRectangle,
    ShapeKind, ShapeRenderable, ShapeStyle, Triangle,
};

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

fn stroke_style(color: Color, width: f32) -> ShapeStyle {
    ShapeStyle {
        fill: Some(color.clone()),
        stroke_color: Some(color),
        stroke_width: Some(width),
    }
}

fn fill_style(color: Color) -> ShapeStyle {
    ShapeStyle {
        fill: Some(color),
        stroke_color: None,
        stroke_width: None,
    }
}

fn main() {
    let window = Window::new("Shapes", 800, 800);
    let renderer = Renderer::new(window.handle());
    renderer.set_point_size(6.0);
    let mut app = App::new(window);

    // Convert polyline points to relative coordinates (relative to first point)
    let polyline_points = vec![
        (0.0, 0.0),
        (50.0, 130.0),
        (100.0, 110.0),
        (100.0, 200.0),
    ];

    // Convert sine wave points to relative coordinates
    let sine_wave_abs = generate_sine_wave(500.0, 100.0, 30.0, 20, 200.0);
    let (sine_x, sine_y) = sine_wave_abs[0];
    let sine_wave_rel: Vec<(f32, f32)> = sine_wave_abs
        .iter()
        .map(|(x, y)| (x - sine_x, y - sine_y))
        .collect();

    // Convert polygon points to relative coordinates
    let polygon_abs = [
        (600.0, 600.0),
        (575.0, 643.3),
        (525.0, 643.3),
        (500.0, 600.0),
        (525.0, 556.6),
        (575.0, 556.6),
    ];
    let (poly_x, poly_y) = polygon_abs[0];
    let polygon_rel: Vec<(f32, f32)> = polygon_abs
        .iter()
        .map(|(x, y)| (x - poly_x, y - poly_y))
        .collect();

    let mut shapes = vec![
        // Line from (100, 200) to (300, 250)
        ShapeRenderable::from_shape(
            100.0,
            200.0,
            ShapeKind::Line(Line::new(300.0, 250.0)),
            stroke_style(Color::from_rgb(0.0, 1.0, 0.0), 1.0),
        ),
        // Polyline starting at (100, 300)
        ShapeRenderable::from_shape(
            100.0,
            300.0,
            ShapeKind::Polyline(Polyline::new(polyline_points)),
            stroke_style(Color::from_rgb(1.0, 0.0, 0.0), 10.0),
        ),
        // Arc centered at (700, 600)
        ShapeRenderable::from_shape(
            700.0,
            600.0,
            ShapeKind::Arc(Arc::new(70.0, 0.0, std::f32::consts::PI / 2.0)),
            stroke_style(Color::from_rgb(0.0, 0.0, 1.0), 10.0),
        ),
        // Rectangle at (50, 50)
        ShapeRenderable::from_shape(
            50.0,
            50.0,
            ShapeKind::Rectangle(Rectangle::new(200.0, 80.0)),
            fill_style(Color::from_rgb(0.2, 0.5, 0.9)),
        ),
        // Triangle at (50, 50)
        ShapeRenderable::from_shape(
            50.0,
            50.0,
            ShapeKind::Triangle(Triangle::new(create_equilateral_triangle())),
            fill_style(Color::from_rgb(1.0, 0.0, 0.0)),
        ),
        // Rectangle at (400, 200)
        ShapeRenderable::from_shape(
            400.0,
            200.0,
            ShapeKind::Rectangle(Rectangle::new(100.0, 50.0)),
            fill_style(Color::from_rgb(1.0, 0.0, 0.0)),
        ),
        // Circle at (400, 400)
        ShapeRenderable::from_shape(
            400.0,
            400.0,
            ShapeKind::Circle(Circle::new(50.0)),
            fill_style(Color::from_rgb(0.0, 0.0, 1.0)),
        ),
        // Point at (600, 300)
        ShapeRenderable::from_shape(
            600.0,
            300.0,
            ShapeKind::Point,
            fill_style(Color::from_rgb(1.0, 0.0, 0.0)),
        ),
        // MultiPoint (sine wave)
        ShapeRenderable::from_shape(
            sine_x,
            sine_y,
            ShapeKind::MultiPoint(MultiPoint::new(sine_wave_rel)),
            fill_style(Color::from_rgb(0.0, 0.0, 1.0)),
        ),
        // Ellipse at (600, 200)
        ShapeRenderable::from_shape(
            600.0,
            200.0,
            ShapeKind::Ellipse(Ellipse::new(80.0, 40.0)),
            fill_style(Color::from_rgb(0.5, 0.2, 0.8)),
        ),
        // Rounded rectangle at (100, 600)
        ShapeRenderable::from_shape(
            100.0,
            600.0,
            ShapeKind::RoundedRectangle(RoundedRectangle::new(200.0, 80.0, 10.0)),
            fill_style(Color::from_rgb(0.3, 0.6, 0.9)),
        ),
        // Polygon (hexagon)
        ShapeRenderable::from_shape(
            poly_x,
            poly_y,
            ShapeKind::Polygon(Polygon::new(polygon_rel)),
            fill_style(Color::from_rgb(1.0, 0.0, 0.0)),
        ),
        // Rectangle using from_shape
        ShapeRenderable::from_shape(
            600.0,
            400.0,
            ShapeKind::Rectangle(Rectangle::new(100.0, 50.0)),
            fill_style(Color::from_rgb(0.0, 1.0, 0.0)),
        ),
        // Images (still use dedicated methods)
        ShapeRenderable::image_with_size(200.0, 300.0, "images/smiley.png", 40.0, 40.0),
        ShapeRenderable::image(400.0, 500.0, "images/bunny.png"),
    ];

    /* Uncomment for svg output
    let mut svg = SvgDocument::new(800.0, 800.0);
    svg.add_shapes(&shapes);
    svg.write_to_file("target/shapes.svg")
        .expect("Failed to write SVG");
    */

    app.on_render(move || {
        for shape in &mut shapes {
            shape.render(&renderer);
        }
    });
    app.run();
}
