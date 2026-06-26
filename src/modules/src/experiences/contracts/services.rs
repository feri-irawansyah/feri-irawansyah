use anyhow::Result;
use async_trait::async_trait;
use crate::experiences::experience_view::ExperienceView;

#[async_trait]
pub trait ExperienceService: Send + Sync {
    async fn list(&self) -> Result<Vec<ExperienceView>>;
}
