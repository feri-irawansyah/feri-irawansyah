use anyhow::Result;
use async_trait::async_trait;
use crate::skills::skill_view::SkillView;

#[async_trait]
pub trait SkillRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<SkillView>>;
    async fn find_by_id(&self, skill_id: i32) -> Result<Option<SkillView>>;
}
