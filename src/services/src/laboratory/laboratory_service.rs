use anyhow::Result;
use async_trait::async_trait;
use modules::laboratory::{LaboratoryCommand, LaboratoryService, LaboratoryView};
use std::sync::Arc;

use crate::laboratory::LaboratoryServiceDeps;

pub struct LaboratoryServiceImpl {
    repo: Arc<dyn modules::laboratory::LaboratoryRepository>,
}

impl LaboratoryServiceImpl {
    pub fn new(deps: LaboratoryServiceDeps) -> Self {
        Self {
            repo: deps.laboratory_repo,
        }
    }
}

#[async_trait]
impl LaboratoryService for LaboratoryServiceImpl {
    async fn get_by_slug(&self, slug: &str) -> Result<Option<LaboratoryView>> {
        self.repo.find_by_slug(slug).await
    }

    async fn by_category_page(
        &self,
        category: &str,
        page: i64,
        per_page: i64,
    ) -> Result<(Vec<LaboratoryView>, i64)> {
        self.repo
            .find_by_category_page(category, page, per_page)
            .await
    }

    async fn list_admin(&self) -> Result<Vec<LaboratoryView>> {
        self.repo.find_all_admin().await
    }

    async fn list_admin_page(
        &self,
        page: i64,
        per_page: i64,
    ) -> Result<(Vec<LaboratoryView>, i64)> {
        self.repo.find_all_admin_page(page, per_page).await
    }

    async fn create(&self, input: LaboratoryCommand) -> Result<LaboratoryView> {
        self.repo.create(input).await
    }

    async fn update(&self, id: i32, input: LaboratoryCommand) -> Result<Option<LaboratoryView>> {
        self.repo.update(id, input).await
    }

    async fn delete(&self, id: i32) -> Result<bool> {
        self.repo.delete(id).await
    }
}
