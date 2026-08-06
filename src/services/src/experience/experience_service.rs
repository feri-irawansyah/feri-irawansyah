use anyhow::Result;
use async_trait::async_trait;
use modules::experience::{ExperienceCommand, ExperienceService, ExperienceView};
use std::sync::Arc;

use crate::experience::ExperienceServiceDeps;

pub struct ExperienceServiceImpl {
    repo: Arc<dyn modules::experience::ExperienceRepository>,
}

impl ExperienceServiceImpl {
    pub fn new(deps: ExperienceServiceDeps) -> Self {
        Self {
            repo: deps.experience_repo,
        }
    }
}

#[async_trait]
impl ExperienceService for ExperienceServiceImpl {
    async fn list(&self) -> Result<Vec<ExperienceView>> {
        self.repo.find_all().await
    }

    async fn list_page(&self, page: i64, per_page: i64) -> Result<(Vec<ExperienceView>, i64)> {
        self.repo.find_page(page, per_page).await
    }

    async fn get_by_ids(&self, ids: &[i32]) -> Result<Vec<ExperienceView>> {
        self.repo.find_by_ids(ids).await
    }

    async fn create(&self, input: ExperienceCommand) -> Result<ExperienceView> {
        self.repo.create(input).await
    }

    async fn update(&self, id: i32, input: ExperienceCommand) -> Result<Option<ExperienceView>> {
        self.repo.update(id, input).await
    }

    async fn delete(&self, id: i32) -> Result<bool> {
        self.repo.delete(id).await
    }
}
