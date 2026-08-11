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
    async fn find_by_slug_async(&self, slug: &str) -> Result<Option<LaboratoryView>> {
        self.repo.find_by_slug_async(slug).await
    }

    async fn find_by_category_page_async(
        &self,
        category: &str,
        page: i64,
        per_page: i64,
    ) -> Result<(Vec<LaboratoryView>, i64)> {
        self.repo
            .find_by_category_page_async(category, page, per_page)
            .await
    }

    async fn find_all_admin_async(&self) -> Result<Vec<LaboratoryView>> {
        self.repo.find_all_admin_async().await
    }

    async fn find_all_admin_page_async(
        &self,
        page: i64,
        per_page: i64,
    ) -> Result<(Vec<LaboratoryView>, i64)> {
        self.repo.find_all_admin_page_async(page, per_page).await
    }

    async fn create_async(&self, input: LaboratoryCommand) -> Result<LaboratoryView> {
        self.repo.create_async(input).await
    }

    async fn update_async(&self, id: i32, input: LaboratoryCommand) -> Result<Option<LaboratoryView>> {
        self.repo.update_async(id, input).await
    }

    async fn delete_async(&self, id: i32) -> Result<bool> {
        self.repo.delete_async(id).await
    }
}
