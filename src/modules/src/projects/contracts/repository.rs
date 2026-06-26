use anyhow::Result;
use async_trait::async_trait;
use crate::projects::project_view::ProjectView;

#[async_trait]
pub trait ProjectRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<ProjectView>>;
    async fn find_featured(&self) -> Result<Vec<ProjectView>>;
    async fn find_by_slug(&self, slug: &str) -> Result<Option<ProjectView>>;
}
