extern crate sky_renderer;

use sky_renderer::core::{App, Color, Renderable, Renderer, Window};
use sky_renderer::graphics2d::shapes::ShapeRenderable;

use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, Instant};

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
    // 1. Shared list of 10 balls
    let balls = Arc::new(RwLock::new(initialize_balls(10)));

    // 2. Spawn background thread to update all ball positions
    {
        let balls_clone = Arc::clone(&balls);
        thread::spawn(move || {
            let mut last_time = Instant::now();

            loop {
                thread::sleep(Duration::from_millis(16)); // ~60 FPS
                let now = Instant::now();
                let dt = now.duration_since(last_time).as_secs_f32();
                last_time = now;

                let mut balls_lock = balls_clone.write().unwrap();
                for ball in balls_lock.iter_mut() {
                    // Move
                    ball.x += ball.vx * dt;
                    ball.y += ball.vy * dt;

                    // Bounce on X
                    if ball.x - BALL_RADIUS < 0.0 || ball.x + BALL_RADIUS > SCREEN_WIDTH {
                        ball.vx = -ball.vx;
                        ball.x = ball.x.clamp(BALL_RADIUS, SCREEN_WIDTH - BALL_RADIUS);
                    }

                    // Bounce on Y
                    if ball.y - BALL_RADIUS < 0.0 || ball.y + BALL_RADIUS > SCREEN_HEIGHT {
                        ball.vy = -ball.vy;
                        ball.y = ball.y.clamp(BALL_RADIUS, SCREEN_HEIGHT - BALL_RADIUS);
                    }
                }
            }
        });
    }

    // 3. Create OpenGL window and renderer
    let window = Window::new("Bouncing Balls", SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
    let mut app = App::new(window);
    let renderer = Renderer::new();
    renderer.set_point_size(6.0);

    // 4. Create one ShapeRenderable per ball (AFTER OpenGL context exists)
    let mut rng = rand::rng();
    let mut shapes: Vec<ShapeRenderable> = (0..10)
        .map(|_| {
            ShapeRenderable::circle(
                0.0,
                0.0,
                BALL_RADIUS,
                Color::from_rgb(rand_f32(&mut rng), rand_f32(&mut rng), rand_f32(&mut rng)),
            )
        })
        .collect();

    // 5. Render loop: read positions, update each shape, render it
    let balls_render = Arc::clone(&balls);
    app.on_render(move || {
        let balls_lock = balls_render.read().unwrap();
        for (shape, ball) in shapes.iter_mut().zip(balls_lock.iter()) {
            shape.set_position(ball.x, ball.y);
            shape.render(&renderer);
        }
    });

    app.run();
}

/// Initializes N balls with random directions and positions
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
