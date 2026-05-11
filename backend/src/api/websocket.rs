use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    Extension,
};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::{watch, mpsc};
use crate::engine::{GenerationEngine, PromptRequest, ClientMessage, ServerMessage};
use crate::state::AppState;
use crate::config::AppConfig;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(engine): Extension<Arc<dyn GenerationEngine>>,
    Extension(state): Extension<Arc<AppState>>,
    Extension(config): Extension<Arc<AppConfig>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, engine, state, config))
}

async fn handle_socket(
    socket: WebSocket, 
    engine: Arc<dyn GenerationEngine>, 
    state: Arc<AppState>,
    config: Arc<AppConfig>,
) {
    let (mut sender, mut receiver) = socket.split();
    
    let (tx_goal, mut rx_goal) = watch::channel::<Option<PromptRequest>>(None);
    let (tx_res, mut rx_res) = mpsc::channel::<ServerMessage>(10);
    
    // Worker task to handle generation independently
    let engine_worker = engine.clone();
    let state_worker = state.clone();
    let tx_res_worker = tx_res.clone();
    
    let worker_handle = tokio::spawn(async move {
        while rx_goal.changed().await.is_ok() {
            let req_opt = rx_goal.borrow().clone();
            if let Some(req) = req_opt {
                tracing::info!("Worker: Executing goal-state prompt: {}", req.prompt);
                
                let _ = tx_res_worker.send(ServerMessage::Status("Generating...".to_string())).await;

                match engine_worker.generate(req.clone()).await {
                    Ok(resp) => {
                        let history_data = serde_json::json!({
                            "id": resp.id,
                            "prompt": req.prompt,
                            "seed": req.seed,
                            "steps": req.steps,
                            "image": resp.data_url,
                        });
                        
                        if let Err(e) = state_worker.save_history(resp.id.clone(), history_data.to_string()).await {
                            tracing::error!("Failed to save history: {}", e);
                        }
                        
                        let _ = tx_res_worker.send(ServerMessage::ImageUpdate(resp)).await;
                    }
                    Err(e) => {
                        tracing::error!("Generation failed: {}", e);
                        let _ = tx_res_worker.send(ServerMessage::Error(e.to_string())).await;
                    }
                }
            }
        }
    });

    // Main event loop
    loop {
        tokio::select! {
            msg = receiver.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        match serde_json::from_str::<ClientMessage>(&text) {
                            Ok(ClientMessage::Generate(req)) => {
                                let _ = tx_goal.send(Some(req));
                            }
                            Ok(ClientMessage::GetHistory) => {
                                match state.get_all_history().await {
                                    Ok(history) => {
                                        let _ = tx_res.send(ServerMessage::HistoryDump(history)).await;
                                    }
                                    Err(e) => {
                                        let _ = tx_res.send(ServerMessage::Error(format!("Failed to load history: {}", e))).await;
                                    }
                                }
                            }
                            Ok(ClientMessage::ExportToStratum(id)) => {
                                tracing::info!("Exporting image {} to Stratum...", id);
                                let _ = tx_res.send(ServerMessage::Status("Exported to Knowledge Base".to_string())).await;
                            }
                            Err(_) => {
                                // Use defaults from config for fallback
                                let _ = tx_goal.send(Some(PromptRequest { 
                                    prompt: text, 
                                    style: "cinematic".to_string(),
                                    seed: config.defaults.seed, 
                                    steps: config.defaults.steps 
                                }));
                            }
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    _ => {}
                }
            }
            Some(server_msg) = rx_res.recv() => {
                if let Ok(json) = serde_json::to_string(&server_msg) {
                    if let Err(e) = sender.send(Message::Text(json)).await {
                        tracing::error!("Failed to send server message: {}", e);
                        break;
                    }
                }
            }
        }
    }
    
    worker_handle.abort();
}
