use anyhow::Result;
use async_trait::async_trait;
use crate::skills::skill_command::SkillCommand;
use crate::skills::skill_view::SkillView;

#[async_trait]
pub trait SkillService: Send + Sync {
    async fn list(&self) -> Result<Vec<SkillView>>;
    async fn list_page(&self, page: i64, per_page: i64) -> Result<(Vec<SkillView>, i64)>;
    async fn create(&self, input: SkillCommand) -> Result<SkillView>;
    async fn update(&self, skill_id: i32, input: SkillCommand) -> Result<Option<SkillView>>;
    async fn delete(&self, skill_id: i32) -> Result<bool>;
}
