use image::RgbImage;
use std::io::Cursor;
use base64::{Engine as _, engine::general_purpose};

pub fn encode_to_data_url(img: &RgbImage) -> anyhow::Result<String> {
    let mut buffer = Cursor::new(Vec::new());
    img.write_to(&mut buffer, image::ImageFormat::Png)?;
    let base64_img = general_purpose::STANDARD.encode(buffer.into_inner());
    Ok(format!("data:image/png;base64,{}", base64_img))
}
