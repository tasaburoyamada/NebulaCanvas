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
        hasher.update(req.style.as_bytes());
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

            // Simulate style-based color palettes
            let (r_base, g_base, b_base) = match req.style.to_lowercase().as_str() {
                "cinematic" => (30, 40, 60),
                "watercolor" => (200, 220, 240),
                "neon" => (255, 20, 147),
                "sketch" => (240, 240, 240),
                _ => (128, 128, 128),
            };

            let color_hash = (req.prompt.chars().map(|c| c as u32).sum::<u32>() + req.seed) % 100;
            let mut img = RgbImage::new(512, 512);
            
            for (x, y, pixel) in img.enumerate_pixels_mut() {
                let density = (req.steps as f32 / 50.0 * 255.0) as u8;
                let pattern = ((x as f32 * 0.1).sin() * (y as f32 * 0.1).cos() * 127.0 + 128.0) as u8;
                
                let r = (r_base as i32 + color_hash as i32 + pattern as i32 / 4).clamp(0, 255) as u8;
                let g = (g_base as i32 + density as i32 / 2).clamp(0, 255) as u8;
                let b = (b_base as i32 + (255 - density) as i32 / 4).clamp(0, 255) as u8;
                
                *pixel = Rgb([r, g, b]);
            }

            let data_url = crate::util::encode_to_data_url(&img)?;
            Ok(ImageResponse { id: compute_id, data_url })
        }).await?
    }
}
