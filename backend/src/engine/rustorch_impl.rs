use super::{GenerationEngine, ImageResponse, PromptRequest};
use async_trait::async_trait;
use rustorch::{Tensor, Device, DType};
use image::{Rgb, RgbImage};
use crate::config::AppConfig;

pub struct RusTorchEngine {
    device: Device,
}

impl RusTorchEngine {
    pub fn new(config: &AppConfig) -> Self {
        let device = if config.engine.device == "cpu" {
            Device::Cpu
        } else if config.engine.device.starts_with("accelerated:") {
            let id = config.engine.device.split(':').nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
            Device::Accelerated(id)
        } else {
            Device::Cpu
        };

        Self { device }
    }
}

#[async_trait]
impl GenerationEngine for RusTorchEngine {
    async fn generate(&self, req: PromptRequest) -> anyhow::Result<ImageResponse> {
        // Deterministic ID generation using Blake3
        let mut hasher = blake3::Hasher::new();
        hasher.update(req.prompt.as_bytes());
        hasher.update(&req.seed.to_le_bytes());
        hasher.update(&req.steps.to_le_bytes());
        let id = hasher.finalize().to_string();

        let device = self.device;
        let compute_id = id.clone();
        
        tokio::task::spawn_blocking(move || {
            tracing::info!("RusTorch: Starting compute for ID: {} on {:?}", compute_id, device);
            
            let shape = vec![512, 512, 3];
            let _latent = Tensor::from_vec(
                vec![req.seed as f32; 512 * 512 * 3],
                shape,
                DType::F32,
                device
            );

            let color_val = (req.prompt.chars().map(|c| c as u32).sum::<u32>() + req.seed) % 255;
            let mut img = RgbImage::new(512, 512);
            for pixel in img.pixels_mut() {
                let density = (req.steps as f32 / 50.0 * 255.0) as u8;
                *pixel = Rgb([color_val as u8, density, 255 - color_val as u8]);
            }

            let data_url = crate::util::encode_to_data_url(&img)?;
            Ok(ImageResponse { id: compute_id, data_url })
        }).await?
    }
}
