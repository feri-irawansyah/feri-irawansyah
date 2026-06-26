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
        Self { repo: deps.skill_repo }
    }
}

#[async_trait]
impl SkillService for SkillServiceImpl {
    async fn list(&self) -> Result<Vec<SkillView>> {
        self.repo.find_all().await
    }

    async fn grouped(&self) -> Result<Vec<(String, Vec<SkillView>)>> {
        let all = self.repo.find_all().await?;
        let mut map: std::collections::BTreeMap<String, Vec<SkillView>> = Default::default();
        for s in all {
            map.entry(s.category.clone()).or_default().push(s);
        }
        Ok(map.into_iter().collect())
    }
}
