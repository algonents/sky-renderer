use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures::{SinkExt, StreamExt};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use rand::{Rng, thread_rng};
use rand::distr::Uniform;

use serde::Serialize;
use tokio::sync::broadcast;

#[derive(Serialize, Clone)]
struct Ball {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

const BALL_RADIUS: f32 = 10.0;
const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 600.0;
const TICK_MS: u64 = 16;
const NUM_BALLS: usize = 10;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:9001").await.unwrap();
    println!("🚀 WebSocket server running at ws://127.0.0.1:9001");

    let (tx, _) = broadcast::channel::<String>(32);
    let balls = Arc::new(Mutex::new(initialize_balls(NUM_BALLS)));

    // Spawn simulation and broadcast loop
    {
        let balls = Arc::clone(&balls);
        let tx = tx.clone();

        tokio::spawn(async move {
            let mut last_time = Instant::now();
            loop {
                tokio::time::sleep(Duration::from_millis(TICK_MS)).await;
                let now = Instant::now();
                let dt = now.duration_since(last_time).as_secs_f32();
                last_time = now;

                let mut balls = balls.lock().unwrap();
                for (i, b) in balls.iter_mut().enumerate() {
                    b.x += b.vx * dt;
                    b.y += b.vy * dt;

                    if b.x - BALL_RADIUS < 0.0 || b.x + BALL_RADIUS > WIDTH {
                        b.vx = -b.vx;
                        b.x = b.x.clamp(BALL_RADIUS, WIDTH - BALL_RADIUS);
                    }

                    if b.y - BALL_RADIUS < 0.0 || b.y + BALL_RADIUS > HEIGHT {
                        b.vy = -b.vy;
                        b.y = b.y.clamp(BALL_RADIUS, HEIGHT - BALL_RADIUS);
                    }

                    // 🔍 Log each ball position
                    println!(
                        "Ball #{:<2}  pos: ({:>6.1}, {:>6.1})  vel: ({:>6.1}, {:>6.1})",
                        i, b.x, b.y, b.vx, b.vy
                    );
                }

                // Serialize x/y for client
                let positions_only: Vec<_> = balls.iter().map(|b| Position {
                    x: b.x,
                    y: b.y,
                }).collect();

                let msg = serde_json::to_string(&positions_only).unwrap();
                let _ = tx.send(msg);
            }
        });
    }

    // Accept WebSocket clients
    while let Ok((stream, _)) = listener.accept().await {
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.unwrap();
            let (mut ws_sink, _) = ws_stream.split();

            while let Ok(msg) = rx.recv().await {
                if ws_sink.send(tokio_tungstenite::tungstenite::Message::Text(msg.clone())).await.is_err() {
                    break;
                }
            }
        });
    }
}

#[derive(Serialize)]
struct Position {
    x: f32,
    y: f32,
}

fn initialize_balls(n: usize) -> Vec<Ball> {
    let mut rng = thread_rng();
    let pos_x = Uniform::new(BALL_RADIUS, WIDTH - BALL_RADIUS).unwrap();
    let pos_y = Uniform::new(BALL_RADIUS, HEIGHT - BALL_RADIUS).unwrap();
    let vel = Uniform::new(-150.0, 150.0).unwrap();

    (0..n)
        .map(|_| Ball {
            x: rng.sample(&pos_x),
            y: rng.sample(&pos_y),
            vx: rng.sample(&vel),
            vy: rng.sample(&vel),
        })
        .collect()
}
