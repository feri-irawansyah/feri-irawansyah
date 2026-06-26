use anyhow::Result;
use async_trait::async_trait;
use modules::experiences::{ExperienceService, ExperienceView};
use std::sync::Arc;

use crate::experiences::ExperienceServiceDeps;

pub struct ExperienceServiceImpl {
    repo: Arc<dyn modules::experiences::ExperienceRepository>,
}

impl ExperienceServiceImpl {
    pub fn new(deps: ExperienceServiceDeps) -> Self {
        Self { repo: deps.experience_repo }
    }
}

#[async_trait]
impl ExperienceService for ExperienceServiceImpl {
    async fn list(&self) -> Result<Vec<ExperienceView>> {
        self.repo.find_all().await
    }
}
