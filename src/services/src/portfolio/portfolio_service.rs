use anyhow::Result;
use async_trait::async_trait;
use modules::portfolio::{PortfolioService, PortfolioView};
use std::sync::Arc;

use crate::portfolio::PortfolioServiceDeps;

pub struct PortfolioServiceImpl {
    repo: Arc<dyn modules::portfolio::PortfolioRepository>,
}

impl PortfolioServiceImpl {
    pub fn new(deps: PortfolioServiceDeps) -> Self {
        Self { repo: deps.portfolio_repo }
    }
}

#[async_trait]
impl PortfolioService for PortfolioServiceImpl {
    async fn list(&self) -> Result<Vec<PortfolioView>> {
        self.repo.find_all().await
    }

    async fn featured(&self) -> Result<Vec<PortfolioView>> {
        self.repo.find_featured().await
    }

    async fn get_by_slug(&self, slug: &str) -> Result<Option<PortfolioView>> {
        self.repo.find_by_slug(slug).await
    }
}
