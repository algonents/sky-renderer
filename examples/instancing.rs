extern crate sky_renderer;

use sky_renderer::core::{App, Color, Renderable, Renderer, Window};
use sky_renderer::graphics2d::shapes::ShapeRenderable;

const WIDTH: i32 = 1600;
const HEIGHT: i32 = 1000;
const COLS: usize = 100;
const ROWS: usize = 60;
const SPACING: f32 = 15.0;
const ORIGIN_X: f32 = 50.0;
const ORIGIN_Y: f32 = 50.0;
const RADIUS: f32 = 3.0;

// Darker-than-DeepSky: SteelBlue (0..1)
const STEEL_BLUE: (f32, f32, f32) = (0.274510, 0.509804, 0.705882);

fn main() {
    let mut window = Window::new("Instancing Demo — Clean & Fast", WIDTH, HEIGHT);
    window.on_resize(|w, h| println!("Window resized: {}x{}", w, h));
    let renderer = Renderer::new(window.handle());
    

    // One shape, many instances
    let mut dots = ShapeRenderable::circle(
        0.0,
        0.0,
        RADIUS,
        Color::from_rgb(STEEL_BLUE.0, STEEL_BLUE.1, STEEL_BLUE.2),
    );
    let instance_count = COLS * ROWS;
    dots.create_multiple_instances(instance_count);

    // Static base grid
    let mut base_positions = Vec::with_capacity(instance_count);
    for j in 0..ROWS {
        for i in 0..COLS {
            base_positions.push((
                ORIGIN_X + i as f32 * SPACING,
                ORIGIN_Y + j as f32 * SPACING,
            ));
        }
    }

    let mut positions = base_positions.clone();
    dots.set_instance_positions(&positions);

    let mut app = App::new(window);
    
    // render loop
    app.on_render(move || {
        // Compute dt (if you want time-based motion later)
        let now = renderer.get_time();
        
        // “Wiggle” deformation (feel free to swap with your physics later)
        let t = now as f32;
        let wiggle = (t * 2.0).sin() * 3.0;

        for (dst, &(x, y)) in positions.iter_mut().zip(base_positions.iter()) {
            *dst = (x + wiggle, y+wiggle);
        }

        dots.set_instance_positions(&positions);
        dots.render(&renderer);
    });

    app.run();
}
