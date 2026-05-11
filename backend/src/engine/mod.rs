use serde::{Deserialize, Serialize};
use async_trait::async_trait;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PromptRequest {
    pub prompt: String,
    pub style: String,
    pub seed: u32,
    pub steps: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageResponse {
    pub id: String,
    pub data_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum ClientMessage {
    Generate(PromptRequest),
    GetHistory,
    ExportToStratum(String), // Image ID
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum ServerMessage {
    ImageUpdate(ImageResponse),
    HistoryDump(Vec<serde_json::Value>),
    Status(String),
    Error(String),
}

#[async_trait]
pub trait GenerationEngine: Send + Sync {
    async fn generate(&self, req: PromptRequest) -> anyhow::Result<ImageResponse>;
}

pub mod rustorch_impl;
