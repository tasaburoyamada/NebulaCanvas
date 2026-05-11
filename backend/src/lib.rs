mod api;
mod engine;
mod state;
mod util;
pub mod config;

use axum::{routing::get, Router, Extension};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::api::websocket::ws_handler;
use crate::engine::rustorch_impl::RusTorchEngine;
use crate::state::AppState;
use crate::config::AppConfig;

pub async fn start_backend() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = AppConfig::load().expect("Failed to load configuration");
    tracing::info!("Config loaded: {:?}", config);

    let state = Arc::new(AppState::new(&config.database.path).expect("Failed to init DB"));
    let engine = Arc::new(RusTorchEngine::new(&config));

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .layer(Extension(engine))
        .layer(Extension(state))
        .layer(Extension(Arc::new(config.clone())))
        .layer(CorsLayer::permissive());

    let addr: SocketAddr = format!("{}:{}", config.server.host, config.server.port)
        .parse()
        .expect("Invalid address");
    
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
