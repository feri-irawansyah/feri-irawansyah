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
    async fn list(&self) -> Result<Vec<PortfolioView>> {
        self.repo.find_all().await
    }

    async fn list_page(&self, page: i64, per_page: i64) -> Result<(Vec<PortfolioView>, i64)> {
        self.repo.find_page(page, per_page).await
    }

    async fn featured(&self) -> Result<Vec<PortfolioView>> {
        self.repo.find_featured().await
    }

    async fn get_by_slug(&self, slug: &str) -> Result<Option<PortfolioView>> {
        self.repo.find_by_slug(slug).await
    }

    async fn create(&self, input: PortfolioCommand) -> Result<PortfolioView> {
        self.repo.create(input).await
    }

    async fn update(&self, id: i32, input: PortfolioCommand) -> Result<Option<PortfolioView>> {
        self.repo.update(id, input).await
    }

    async fn delete(&self, id: i32) -> Result<bool> {
        self.repo.delete(id).await
    }
}
