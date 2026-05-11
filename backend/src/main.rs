use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use image::{Rgb, RgbImage};
use std::io::Cursor;
use base64::{Engine as _, engine::general_purpose};
use serde::Deserialize;

#[derive(Deserialize)]
struct PromptRequest {
    prompt: String,
    seed: u32,
    steps: u32,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            return;
        };

        if let Message::Text(text) = msg {
            // Parse JSON request
            let req: PromptRequest = match serde_json::from_str(&text) {
                Ok(req) => req,
                Err(_) => {
                    // Fallback to plain text for backward compatibility during dev
                    PromptRequest {
                        prompt: text,
                        seed: 42,
                        steps: 20,
                    }
                }
            };

            tracing::info!("Received prompt: {} (Seed: {}, Steps: {})", req.prompt, req.seed, req.steps);
            
            // Generate a dynamic mock image (colored square based on prompt hash + seed)
            let color_val = (req.prompt.chars().map(|c| c as u32).sum::<u32>() + req.seed) % 255;
            let mut img = RgbImage::new(512, 512);
            for pixel in img.pixels_mut() {
                // Incorporate steps into the color density
                let density = (req.steps as f32 / 50.0 * 255.0) as u8;
                *pixel = Rgb([color_val as u8, density, 255 - color_val as u8]);
            }

            let mut buffer = Cursor::new(Vec::new());
            img.write_to(&mut buffer, image::ImageFormat::Png).unwrap();
            let base64_img = general_purpose::STANDARD.encode(buffer.into_inner());
            let response = format!("data:image/png;base64,{}", base64_img);

            if socket.send(Message::Text(response)).await.is_err() {
                return;
            }
        }
    }
}
