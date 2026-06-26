use anyhow::Result;
use async_trait::async_trait;
use crate::skills::skill_view::SkillView;

#[async_trait]
pub trait SkillService: Send + Sync {
    async fn list(&self) -> Result<Vec<SkillView>>;
    async fn grouped(&self) -> Result<Vec<(String, Vec<SkillView>)>>;
}
