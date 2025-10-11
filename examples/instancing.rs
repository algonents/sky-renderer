extern crate sky_renderer;

use sky_renderer::core::Color;
use sky_renderer::core::{App, Renderable, Renderer, Window};
use sky_renderer::graphics2d::shapes::ShapeRenderable;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let mut window = Window::new("Instancing Demo", 1600, 1000);
    window.on_resize(|w, h| {
        println!("Window resized: {}x{}", w, h);
    });

    let mut app = App::new(window);

    // 🟢 Prepare instanced renderable (10,000 small circles)
    let mut dots = ShapeRenderable::circle(0.0, 0.0, 3.0, Color::from_rgb(255.0, 0.0, 0.0));
    dots.create_multiple_instances(6_000);

    // Build initial positions in a grid
    let cols = 100;
    let rows = 60;
    let spacing = 15.0;
    let mut base_positions = Vec::with_capacity(cols * rows);
    for j in 0..rows {
        for i in 0..cols {
            base_positions.push((50.0 + i as f32 * spacing, 50.0 + j as f32 * spacing));
        }
    }

    let dots = Rc::new(RefCell::new(dots));
    let base_positions = Rc::new(base_positions);

    let renderer = Renderer::new();
    renderer.set_point_size(4.0);

    // 🌀 Hook into render loop
    app.on_render(move || {
        let dots = Rc::clone(&dots);
        let base_positions = Rc::clone(&base_positions);

        let t = renderer.get_time() as f32;
        let wiggle = (t * 2.0).sin() * 3.0;

        // Build new positions (you could mutate a buffer instead)
        let mut positions = Vec::with_capacity(base_positions.len());
        for &(x, y) in base_positions.iter() {
            positions.push((x + wiggle, y));
        }

        // Upload and draw all instances
        let mut dots = dots.borrow_mut();
        dots.set_instance_positions(&positions);
        dots.render(&renderer);
    });

    app.run();
}
