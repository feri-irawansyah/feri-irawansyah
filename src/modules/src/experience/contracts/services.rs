use anyhow::Result;
use async_trait::async_trait;

use crate::experience::experience_command::ExperienceCommand;
use crate::experience::experience_view::ExperienceView;

#[async_trait]
pub trait ExperienceService: Send + Sync {
    async fn list(&self) -> Result<Vec<ExperienceView>>;
    async fn list_page(&self, page: i64, per_page: i64) -> Result<(Vec<ExperienceView>, i64)>;
    async fn get_by_ids(&self, ids: &[i32]) -> Result<Vec<ExperienceView>>;
    async fn create(&self, input: ExperienceCommand) -> Result<ExperienceView>;
    async fn update(&self, id: i32, input: ExperienceCommand) -> Result<Option<ExperienceView>>;
    async fn delete(&self, id: i32) -> Result<bool>;
}
