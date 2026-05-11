use super::{GenerationEngine, ImageResponse, PromptRequest};
use async_trait::async_trait;
use image::{Rgb, RgbImage};

pub struct MockEngine;

#[async_trait]
impl GenerationEngine for MockEngine {
    async fn generate(&self, req: PromptRequest) -> anyhow::Result<ImageResponse> {
        let mut hasher = blake3::Hasher::new();
        hasher.update(req.prompt.as_bytes());
        hasher.update(&req.seed.to_le_bytes());
        hasher.update(&req.steps.to_le_bytes());
        let id = hasher.finalize().to_string();

        let color_val = (req.prompt.chars().map(|c| c as u32).sum::<u32>() + req.seed) % 255;
        let mut img = RgbImage::new(512, 512);
        for pixel in img.pixels_mut() {
            let density = (req.steps as f32 / 50.0 * 255.0) as u8;
            *pixel = Rgb([color_val as u8, density, 255 - color_val as u8]);
        }

        let data_url = crate::util::encode_to_data_url(&img)?;
        Ok(ImageResponse { id, data_url })
    }
}
