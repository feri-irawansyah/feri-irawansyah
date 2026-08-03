use anyhow::Result;
use async_trait::async_trait;
use crate::skills::skill_command::SkillCommand;
use crate::skills::skill_view::SkillView;

#[async_trait]
pub trait SkillRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<SkillView>>;
    async fn find_page(&self, page: i64, per_page: i64) -> Result<(Vec<SkillView>, i64)>;
    async fn find_by_id(&self, skill_id: i32) -> Result<Option<SkillView>>;
    async fn create(&self, input: SkillCommand) -> Result<SkillView>;
    async fn update(&self, skill_id: i32, input: SkillCommand) -> Result<Option<SkillView>>;
    async fn delete(&self, skill_id: i32) -> Result<bool>;
}
