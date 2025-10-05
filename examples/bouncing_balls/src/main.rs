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
const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;

fn main() {
    // 1) Balls live on the main thread now
    let mut balls = initialize_balls(10);

    // 2) Create window and renderer
    let window = Window::new("Bouncing Balls", SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
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
            if ball.x - BALL_RADIUS < 0.0 || ball.x + BALL_RADIUS > SCREEN_WIDTH {
                ball.vx = -ball.vx;
                ball.x = ball.x.clamp(BALL_RADIUS, SCREEN_WIDTH - BALL_RADIUS);
            }

            // bounce Y
            if ball.y - BALL_RADIUS < 0.0 || ball.y + BALL_RADIUS > SCREEN_HEIGHT {
                ball.vy = -ball.vy;
                ball.y = ball.y.clamp(BALL_RADIUS, SCREEN_HEIGHT - BALL_RADIUS);
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

fn initialize_balls(n: usize) -> Vec<Ball> {
    let mut rng = rand::rng();
    let pos_x = Uniform::new(BALL_RADIUS, SCREEN_WIDTH - BALL_RADIUS).unwrap();
    let pos_y = Uniform::new(BALL_RADIUS, SCREEN_HEIGHT - BALL_RADIUS).unwrap();
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
