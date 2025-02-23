use crate::plugins::plugin::Plugin;
use crate::renderer::bmp_renderer::BmpRenderer;
use anyhow::anyhow;
use async_trait::async_trait;
use bitvec::bitvec;
use bitvec::order::Msb0;
use image::{GenericImageView, Pixel};
use logcall::logcall;
use rocket::serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize)]
pub struct StaticImagePlugin {
    pub path: String,
}

#[async_trait]
impl Plugin for StaticImagePlugin {
    #[logcall(err = "warn")]
    async fn template(&self) -> anyhow::Result<String> {
        Ok(self.path.clone())
    }

    #[logcall(err = "warn")]
    async fn render(&self, _t: String, bmp_renderer: &BmpRenderer) -> anyhow::Result<Vec<u8>> {
        let img = image::load(
            BufReader::new(File::open(self.path.clone())?),
            image::ImageFormat::from_extension(
                PathBuf::from(self.path.clone())
                    .extension()
                    .ok_or(anyhow!("Unknown image extension"))?
                    .to_str()
                    .ok_or(anyhow!("Unknown image extension"))?,
            )
            .ok_or(anyhow!("Unknown image extension"))?,
        )?;
        // Resize to 800x480
        let img = img.resize_exact(
            bmp_renderer.width,
            bmp_renderer.width,
            image::imageops::FilterType::Gaussian,
        );

        // Convert to 1-bit
        let threshold = 128u8;
        let mut bw_bits = bitvec![u8, Msb0; 0; img.width() as usize * img.height() as usize];

        for (i, pixel) in img.pixels().enumerate() {
            let (_, _, p) = pixel;

            bw_bits.set(i, p.to_luma()[0] >= threshold);
        }

        // Save as 1-bit BMP
        let buffer =
            bmp_renderer.save_1bit_bmp(bmp_renderer.width, bmp_renderer.height, &bw_bits)?;

        Ok(buffer)
    }
}
