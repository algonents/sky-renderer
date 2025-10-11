extern crate sky_renderer;

use sky_renderer::core::{App, Color, Renderable, Renderer, Window};
use sky_renderer::graphics2d::shapes::ShapeRenderable;

use rand::{Rng, rngs::ThreadRng};
use rand::distr::Uniform;

#[derive(Clone, Copy)]
struct Ball {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

const BALL_RADIUS: f32 = 10.0;

fn main() {

    let mut balls = initialize_balls(50, 800.0, 600.0);

    let window = Window::new("Bouncing Balls", 800, 600);

    // so we can access window properties after the window has been moved to app
    let h_wnd = window.handle();

    let mut app = App::new(window);

    let renderer = Renderer::new();
    renderer.set_point_size(6.0);

    // 3) Create one ShapeRenderable per ball (AFTER OpenGL context exists)
    let mut rng = rand::rng();

    let mut shapes: Vec<ShapeRenderable> = (0..balls.len())
        .map(|_| {
            ShapeRenderable::circle(
                0.0,
                0.0,
                BALL_RADIUS,
                Color::from_rgb(rand_f32(&mut rng), rand_f32(&mut rng), rand_f32(&mut rng)),
            )
        })
        .collect();

    // 4) Timekeeping for per-frame delta
    let mut last_time = renderer.get_time();



    // 5) Render loop: update physics, update shapes, render
    app.on_render(move || {
        let current_time = renderer.get_time();
        let dt = (current_time - last_time) as f32;
        last_time = current_time;


        // -- update physics
        for ball in balls.iter_mut() {
            // integrate
            ball.x += ball.vx * dt;
            ball.y += ball.vy * dt;

            // bounce X
            if ball.x - BALL_RADIUS < 0.0 || ball.x + BALL_RADIUS > h_wnd.width() as f32 {
                ball.vx = -ball.vx;
                ball.x = ball.x.clamp(BALL_RADIUS, h_wnd.width() as f32 - BALL_RADIUS);
            }

            // bounce Y
            if ball.y - BALL_RADIUS < 0.0 || ball.y + BALL_RADIUS > h_wnd.height() as f32 {
                ball.vy = -ball.vy;
                ball.y = ball.y.clamp(BALL_RADIUS, h_wnd.height() as f32 - BALL_RADIUS);
            }
        }

        // -- update shapes and render
        for (shape, ball) in shapes.iter_mut().zip(balls.iter()) {
            shape.set_position(ball.x, ball.y);
            shape.render(&renderer);
        }
    });

    app.run();
}

fn initialize_balls(n: usize, screen_width: f32, screen_height: f32) -> Vec<Ball> {
    let mut rng = rand::rng();
    let pos_x = Uniform::new(BALL_RADIUS, screen_width - BALL_RADIUS).unwrap();
    let pos_y = Uniform::new(BALL_RADIUS, screen_height - BALL_RADIUS).unwrap();
    let vel = Uniform::new(-150.0, 150.0).unwrap();

    (0..n)
        .map(|_| Ball {
            x: rng.sample(pos_x),
            y: rng.sample(pos_y),
            vx: rng.sample(vel),
            vy: rng.sample(vel),
        })
        .collect()
}

/// Random float between 0.0 and 1.0
fn rand_f32(rng: &mut ThreadRng) -> f32 {
    rng.random_range(0.0..1.0)
}
