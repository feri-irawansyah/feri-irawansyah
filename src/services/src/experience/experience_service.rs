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
    async fn find_all_async(&self) -> Result<Vec<ExperienceView>> {
        self.repo.find_all_async().await
    }

    async fn find_page_async(&self, page: i64, per_page: i64) -> Result<(Vec<ExperienceView>, i64)> {
        self.repo.find_page_async(page, per_page).await
    }

    async fn find_by_ids_async(&self, ids: &[i32]) -> Result<Vec<ExperienceView>> {
        self.repo.find_by_ids_async(ids).await
    }

    async fn create_async(&self, input: ExperienceCommand) -> Result<ExperienceView> {
        self.repo.create_async(input).await
    }

    async fn update_async(&self, id: i32, input: ExperienceCommand) -> Result<Option<ExperienceView>> {
        self.repo.update_async(id, input).await
    }

    async fn delete_async(&self, id: i32) -> Result<bool> {
        self.repo.delete_async(id).await
    }
}
