use anyhow::Result;
use async_trait::async_trait;
use crate::projects::project_view::ProjectView;

#[async_trait]
pub trait ProjectService: Send + Sync {
    async fn list(&self) -> Result<Vec<ProjectView>>;
    async fn featured(&self) -> Result<Vec<ProjectView>>;
    async fn get_by_slug(&self, slug: &str) -> Result<Option<ProjectView>>;
}
