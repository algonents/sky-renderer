extern crate sky_renderer;

use sky_renderer::core::{App, Color, Renderable, Renderer, Window};
use sky_renderer::graphics2d::shapes::{
    Circle, Ellipse, Line, MultiPoint, Point, Polygon, Polyline, Rectangle, RoundedRectangle,
    ShapeRenderable, ShapeStyle,
};

use std::cell::Cell;

thread_local! {
    static ZOOM_LEVEL: Cell<f32> = Cell::new(1.0);
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
    let mut window = Window::new("Shapes", 800, 800);
    let mut renderer = Renderer::new(window.handle());
    renderer.set_point_size(6.0);

    window.on_scroll(move |_, y_offset| {
        let zoom_step = 1.1;
        let zoom_factor = if y_offset > 0.0 {
            zoom_step
        } else {
            1.0 / zoom_step
        };

        ZOOM_LEVEL.with(|z| {
            let new_zoom = (z.get() * zoom_factor).clamp(0.1, 10.0);
            z.set(new_zoom);
            println!("zoom level: {}", new_zoom);
        });
    });

    let mut app = App::new(window);

    // Polyline points (relative to first point)
    let polyline_points = vec![
        (0.0, 0.0),
        (50.0, 130.0),
        (100.0, 110.0),
        (150.0, 160.0),
    ];

    // MultiPoint points (relative to first point)
    let multipoint_points = vec![
        (0.0, 0.0),   // anchor at (600, 100)
        (20.0, 20.0),
        (-20.0, 20.0),
    ];

    // Polygon points (relative to first point at 600, 600)
    let polygon_points = vec![
        (0.0, 0.0),
        (-25.0, 43.3),
        (-75.0, 43.3),
        (-100.0, 0.0),
        (-75.0, -43.4),
        (-25.0, -43.4),
    ];

    let mut shapes = vec![
        // Line from (100, 200) to (300, 250)
        ShapeRenderable::from_shape(
            100.0,
            200.0,
            Box::new(Line::new(300.0, 250.0)),
            stroke_style(Color::from_rgb(0.0, 1.0, 0.0), 1.0),
        ),
        // Polyline starting at (100, 300)
        ShapeRenderable::from_shape(
            100.0,
            300.0,
            Box::new(Polyline::new(polyline_points)),
            stroke_style(Color::from_rgb(0.0, 1.0, 0.0), 10.0),
        ),
        // Rectangle at (50, 50)
        ShapeRenderable::from_shape(
            50.0,
            50.0,
            Box::new(Rectangle::new(200.0, 80.0)),
            fill_style(Color::from_rgb(0.2, 0.5, 0.9)),
        ),
        // Rectangle at (400, 200)
        ShapeRenderable::from_shape(
            400.0,
            200.0,
            Box::new(Rectangle::new(100.0, 50.0)),
            fill_style(Color::from_rgb(1.0, 0.0, 0.0)),
        ),
        // Circle at (400, 400)
        ShapeRenderable::from_shape(
            400.0,
            400.0,
            Box::new(Circle::new(50.0)),
            fill_style(Color::from_rgb(0.0, 0.0, 1.0)),
        ),
        // Point at (600, 300)
        ShapeRenderable::from_shape(
            600.0,
            300.0,
            Box::new(Point::new()),
            fill_style(Color::from_rgb(1.0, 0.0, 0.0)),
        ),
        // MultiPoint at (600, 100)
        ShapeRenderable::from_shape(
            600.0,
            100.0,
            Box::new(MultiPoint::new(multipoint_points)),
            fill_style(Color::from_rgb(0.0, 0.0, 1.0)),
        ),
        // Ellipse at (600, 200)
        ShapeRenderable::from_shape(
            600.0,
            200.0,
            Box::new(Ellipse::new(80.0, 40.0)),
            fill_style(Color::from_rgb(0.5, 0.2, 0.8)),
        ),
        // Rounded rectangle at (100, 600)
        ShapeRenderable::from_shape(
            100.0,
            600.0,
            Box::new(RoundedRectangle::new(200.0, 80.0, 10.0)),
            fill_style(Color::from_rgb(0.3, 0.6, 0.9)),
        ),
        // Polygon (hexagon) at (600, 600)
        ShapeRenderable::from_shape(
            600.0,
            600.0,
            Box::new(Polygon::new(polygon_points)),
            fill_style(Color::from_rgb(1.0, 0.0, 0.0)),
        ),
        // Rectangle using from_shape
        ShapeRenderable::from_shape(
            600.0,
            400.0,
            Box::new(Rectangle::new(100.0, 50.0)),
            ShapeStyle::default(),
        ),
        // Images
        ShapeRenderable::image_with_size(200.0, 300.0, "images/smiley.png", 40.0, 40.0),
        ShapeRenderable::image(400.0, 500.0, "images/bunny.png"),
    ];

    app.on_render(move || {
        for shape in &mut shapes {
            ZOOM_LEVEL.with(|z| {
                renderer.zoom_level = z.get();
            });
            shape.render(&renderer);
        }
    });
    app.run();
}
