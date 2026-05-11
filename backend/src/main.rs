mod api;
mod engine;
mod state;
mod util;

use axum::{routing::get, Router, Extension};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::api::websocket::ws_handler;
use crate::engine::rustorch_impl::RusTorchEngine;
use crate::state::AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = Arc::new(AppState::new("nebula_canvas.redb").expect("Failed to init DB"));
    let engine = Arc::new(RusTorchEngine);

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .layer(Extension(engine))
        .layer(Extension(state))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
