use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    Extension,
};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::{watch, mpsc};
use crate::engine::{GenerationEngine, PromptRequest, ClientMessage, ServerMessage, ImageResponse};
use crate::state::AppState;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(engine): Extension<Arc<dyn GenerationEngine>>,
    Extension(state): Extension<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, engine, state))
}

async fn handle_socket(socket: WebSocket, engine: Arc<dyn GenerationEngine>, state: Arc<AppState>) {
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
                        // Parse as ClientMessage enum
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
                            Err(_) => {
                                // Backward compatibility / Fallback
                                if let Ok(req) = serde_json::from_str::<PromptRequest>(&text) {
                                     let _ = tx_goal.send(Some(req));
                                } else {
                                     let _ = tx_goal.send(Some(PromptRequest { prompt: text, seed: 42, steps: 20 }));
                                }
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
