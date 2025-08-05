use sky_renderer::core::{App, Color, Renderable, Renderer, Window};
use sky_renderer::graphics2d::shapes::ShapeRenderable;

use std::sync::{Arc, RwLock};
use tokio::runtime::Runtime;
use tokio_tungstenite::connect_async;
use futures_util::StreamExt;

use serde::Deserialize;

#[derive(Deserialize, Clone, Copy)]
struct BallPosition {
    x: f32,
    y: f32,
}

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;
const BALL_RADIUS: f32 = 10.0;

fn main() {
    // Shared global state of positions
    let positions: Arc<RwLock<Vec<BallPosition>>> = Arc::new(RwLock::new(vec![]));

    // Spawn background websocket receiver in a Tokio runtime
    {
        let positions_clone = Arc::clone(&positions);
        std::thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            rt.block_on(run_ws_receiver(positions_clone));
        });
    }

    // Setup OpenGL window
    let window = Window::new("WS Client Viewer", SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut app = App::new(window);
    let renderer = Renderer::new();
    renderer.set_point_size(6.0);

    // Prepare a pool of renderables (we'll reuse and resize as needed)
    let mut shapes: Vec<ShapeRenderable> = Vec::new();

    let positions_render = Arc::clone(&positions);
    app.on_render(move || {
        let pos_data = positions_render.read().unwrap();

        // Resize shape pool if needed
        if pos_data.len() > shapes.len() {
            let extra = pos_data.len() - shapes.len();
            for _ in 0..extra {
                shapes.push(ShapeRenderable::circle(
                    0.0,
                    0.0,
                    BALL_RADIUS,
                    Color::from_rgb(0.0, 1.0, 0.0),
                ));
            }
        }

        for (shape, pos) in shapes.iter_mut().zip(pos_data.iter()) {
            shape.set_position(pos.x, pos.y);
            shape.render(&renderer);
        }
    });

    app.run();
}

async fn run_ws_receiver(shared: Arc<RwLock<Vec<BallPosition>>>) {
    let url = url::Url::parse("ws://127.0.0.1:9001").unwrap();
    println!("🔌 Connecting to {}", url);

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("✅ Connected!");

    let (_, mut reader) = ws_stream.split();

    while let Some(Ok(msg)) = reader.next().await {
        if msg.is_text() {
            if let Ok(parsed) = serde_json::from_str::<Vec<BallPosition>>(msg.to_text().unwrap()) {
                let mut lock = shared.write().unwrap();
                *lock = parsed;
            }
        }
    }

    println!("🔌 Connection closed");
}
