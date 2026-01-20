extern crate sky_renderer;

use sky_renderer::core::{App, Color, Renderable, Renderer, Window, Vec2};
use sky_renderer::graphics2d::shapes::ShapeRenderable;

use rand::{rngs::ThreadRng, Rng};
use rand::distr::Uniform;

#[derive(Clone, Copy)]
struct Ball {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

const BALL_RADIUS: f32 = 5.0;

fn main() {
    let mut balls = initialize_balls(1000, 1600.0, 1200.0);

    let window = Window::new("Bouncing Balls — Instanced", 1600, 1200);
    let h_wnd = window.handle(); // read-only window info after move
    let mut app = App::new(window);

    let renderer = Renderer::new();
    renderer.set_point_size(6.0);

    let mut dots = ShapeRenderable::circle(
        0.0,
        0.0,
        BALL_RADIUS,
        Color::from_rgb(0.254902, 0.411765, 0.882353)
    );
    dots.create_multiple_instances(balls.len());
    // upload initial positions
    {
        let positions: Vec<Vec2> = balls.iter().map(|b| Vec2::new(b.x, b.y)).collect();
        dots.set_instance_positions(&positions);
    }

    // Timekeeping
    let mut last_time = renderer.get_time();

    app.on_render(move || {
        let current_time = renderer.get_time();
        let dt = (current_time - last_time) as f32;
        last_time = current_time;

        // Physics update
        for ball in balls.iter_mut() {
            ball.x += ball.vx * dt;
            ball.y += ball.vy * dt;

            // Bounce X
            if ball.x - BALL_RADIUS < 0.0 || ball.x + BALL_RADIUS > h_wnd.width() as f32 {
                ball.vx = -ball.vx;
                ball.x = ball.x.clamp(BALL_RADIUS, h_wnd.width() as f32 - BALL_RADIUS);
            }
            // Bounce Y
            if ball.y - BALL_RADIUS < 0.0 || ball.y + BALL_RADIUS > h_wnd.height() as f32 {
                ball.vy = -ball.vy;
                ball.y = ball.y.clamp(BALL_RADIUS, h_wnd.height() as f32 - BALL_RADIUS);
            }
        }

        // 🔁 Update instance positions (single VBO upload)
        let positions: Vec<Vec2> = balls.iter().map(|b| Vec2::new(b.x, b.y)).collect();
        dots.set_instance_positions(&positions);

        // 🖼️ Single instanced draw call
        dots.render(&renderer);
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

// Random float between 0.0 and 1.0
fn rand_f32(rng: &mut ThreadRng) -> f32 {
    rng.random_range(0.0..1.0)
}
