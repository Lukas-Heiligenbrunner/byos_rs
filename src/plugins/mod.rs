use async_trait::async_trait;

pub mod github_commit_graph;
mod utils;

#[async_trait]
pub trait Plugin: Send + Sync {
    async fn render(&self) -> anyhow::Result<String>;
}
