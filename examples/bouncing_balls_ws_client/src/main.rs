use sky_renderer::core::{App, Color, Renderable, Renderer, Window};
use sky_renderer::graphics2d::shapes::{Circle, ShapeKind, ShapeRenderable, ShapeStyle};

use std::sync::{Arc, RwLock};
use tokio::runtime::Runtime;
use tokio_tungstenite::connect_async;
use futures_util::StreamExt;

use serde::Deserialize;

#[derive(Deserialize, Clone)]
struct BallSnapshot {
    x: f32,
    y: f32,
    r: f32,
    g: f32,
    b: f32,
}

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;
const BALL_RADIUS: f32 = 10.0;

fn main() {
    let positions: Arc<RwLock<Vec<BallSnapshot>>> = Arc::new(RwLock::new(vec![]));

    {
        let positions_clone = Arc::clone(&positions);
        std::thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            rt.block_on(run_ws_receiver(positions_clone));
        });
    }

    let window = Window::new("WS Client Viewer", SCREEN_WIDTH, SCREEN_HEIGHT);

    let renderer = Renderer::new(window.handle());
    renderer.set_point_size(6.0);

    let mut app = App::new(window);

    let mut shapes: Vec<ShapeRenderable> = Vec::new();

    let positions_render = Arc::clone(&positions);
    app.on_render(move || {
        let pos_data = positions_render.read().unwrap();

        if pos_data.len() > shapes.len() {
            for snap in &pos_data[shapes.len()..] {
                shapes.push(ShapeRenderable::from_shape(
                    snap.x,
                    snap.y,
                    ShapeKind::Circle(Circle::new(BALL_RADIUS)),
                    ShapeStyle {
                        fill: Some(Color::from_rgb(snap.r, snap.g, snap.b)),
                        stroke_color: None,
                        stroke_width: None,
                    },
                ));
            }
        }

        for (shape, snap) in shapes.iter_mut().zip(pos_data.iter()) {
            shape.set_position(snap.x, snap.y);
            shape.render(&renderer);
        }
    });

    app.run();
}

async fn run_ws_receiver(shared: Arc<RwLock<Vec<BallSnapshot>>>) {
    let url = url::Url::parse("ws://127.0.0.1:9001").unwrap();
    println!("🔌 Connecting to {}", url);

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("✅ Connected!");

    let (_, mut reader) = ws_stream.split();

    while let Some(Ok(msg)) = reader.next().await {
        if msg.is_text() {
            if let Ok(parsed) = serde_json::from_str::<Vec<BallSnapshot>>(msg.to_text().unwrap()) {
                let mut lock = shared.write().unwrap();
                *lock = parsed;
            }
        }
    }

    println!("🔌 Connection closed");
}
