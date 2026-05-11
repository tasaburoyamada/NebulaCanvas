use super::{GenerationEngine, ImageResponse, PromptRequest};
use async_trait::async_trait;
use rustorch::{Tensor, Device, DType};
use image::{Rgb, RgbImage};

pub struct RusTorchEngine;

#[async_trait]
impl GenerationEngine for RusTorchEngine {
    async fn generate(&self, req: PromptRequest) -> anyhow::Result<ImageResponse> {
        // Deterministic ID generation using Blake3 (moved here for engine integrity)
        let mut hasher = blake3::Hasher::new();
        hasher.update(req.prompt.as_bytes());
        hasher.update(&req.seed.to_le_bytes());
        hasher.update(&req.steps.to_le_bytes());
        let id = hasher.finalize().to_string();

        // Move compute to a blocking task
        let compute_id = id.clone();
        tokio::task::spawn_blocking(move || {
            tracing::info!("RusTorch: Starting compute for ID: {}", compute_id);
            
            // Simulation of a RusTorch-based generation process.
            let shape = vec![512, 512, 3];
            let _latent = Tensor::from_vec(
                vec![req.seed as f32; 512 * 512 * 3],
                shape,
                DType::F32,
                Device::Cpu
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
