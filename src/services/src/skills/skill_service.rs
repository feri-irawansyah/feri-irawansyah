use anyhow::Result;
use async_trait::async_trait;
use modules::skills::{SkillCommand, SkillService, SkillView};
use std::sync::Arc;

use crate::skills::SkillServiceDeps;

pub struct SkillServiceImpl {
    repo: Arc<dyn modules::skills::SkillRepository>,
}

impl SkillServiceImpl {
    pub fn new(deps: SkillServiceDeps) -> Self {
        Self {
            repo: deps.skill_repo,
        }
    }
}

#[async_trait]
impl SkillService for SkillServiceImpl {
    async fn list(&self) -> Result<Vec<SkillView>> {
        self.repo.find_all().await
    }

    async fn list_page(&self, page: i64, per_page: i64) -> Result<(Vec<SkillView>, i64)> {
        self.repo.find_page(page, per_page).await
    }

    async fn create(&self, input: SkillCommand) -> Result<SkillView> {
        self.repo.create(input).await
    }

    async fn update(&self, skill_id: i32, input: SkillCommand) -> Result<Option<SkillView>> {
        self.repo.update(skill_id, input).await
    }

    async fn delete(&self, skill_id: i32) -> Result<bool> {
        self.repo.delete(skill_id).await
    }
}
