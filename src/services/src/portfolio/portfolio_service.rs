use anyhow::Result;
use async_trait::async_trait;
use modules::portfolio::{PortfolioCommand, PortfolioService, PortfolioView};
use std::sync::Arc;

use crate::portfolio::PortfolioServiceDeps;

pub struct PortfolioServiceImpl {
    repo: Arc<dyn modules::portfolio::PortfolioRepository>,
}

impl PortfolioServiceImpl {
    pub fn new(deps: PortfolioServiceDeps) -> Self {
        Self {
            repo: deps.portfolio_repo,
        }
    }
}

#[async_trait]
impl PortfolioService for PortfolioServiceImpl {
    async fn find_all_async(&self) -> Result<Vec<PortfolioView>> {
        self.repo.find_all_async().await
    }

    async fn find_page_async(&self, page: i64, per_page: i64) -> Result<(Vec<PortfolioView>, i64)> {
        self.repo.find_page_async(page, per_page).await
    }

    async fn find_featured_async(&self) -> Result<Vec<PortfolioView>> {
        self.repo.find_featured_async().await
    }

    async fn find_by_slug_async(&self, slug: &str) -> Result<Option<PortfolioView>> {
        self.repo.find_by_slug_async(slug).await
    }

    async fn create_async(&self, input: PortfolioCommand) -> Result<PortfolioView> {
        self.repo.create_async(input).await
    }

    async fn update_async(&self, id: i32, input: PortfolioCommand) -> Result<Option<PortfolioView>> {
        self.repo.update_async(id, input).await
    }

    async fn delete_async(&self, id: i32) -> Result<bool> {
        self.repo.delete_async(id).await
    }
}
