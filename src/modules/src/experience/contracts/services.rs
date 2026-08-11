use anyhow::Result;
use async_trait::async_trait;

use crate::experience::experience_command::ExperienceCommand;
use crate::experience::experience_view::ExperienceView;

#[async_trait]
pub trait ExperienceService: Send + Sync {
    async fn find_all_async(&self) -> Result<Vec<ExperienceView>>;
    async fn find_page_async(&self, page: i64, per_page: i64) -> Result<(Vec<ExperienceView>, i64)>;
    async fn find_by_ids_async(&self, ids: &[i32]) -> Result<Vec<ExperienceView>>;
    async fn create_async(&self, input: ExperienceCommand) -> Result<ExperienceView>;
    async fn update_async(&self, id: i32, input: ExperienceCommand) -> Result<Option<ExperienceView>>;
    async fn delete_async(&self, id: i32) -> Result<bool>;
}
