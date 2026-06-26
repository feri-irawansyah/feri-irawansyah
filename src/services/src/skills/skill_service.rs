use anyhow::Result;
use async_trait::async_trait;
use modules::skills::{SkillService, SkillView};
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
}
