use anyhow::Result;
use async_trait::async_trait;
use crate::experiences::experience_view::ExperienceView;

#[async_trait]
pub trait ExperienceRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<ExperienceView>>;
    async fn find_by_id(&self, id: i32) -> Result<Option<ExperienceView>>;
}
