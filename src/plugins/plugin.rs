use crate::renderer::bmp_renderer::BmpRenderer;
use async_trait::async_trait;

#[async_trait]
pub trait Plugin: Send + Sync {
    async fn template(&self) -> anyhow::Result<String>;
    async fn render(&self, template: String, bmp_renderer: &BmpRenderer)
        -> anyhow::Result<Vec<u8>>;
}
