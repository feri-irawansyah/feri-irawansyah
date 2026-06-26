use anyhow::Result;
use async_trait::async_trait;
use crate::portfolio::portfolio_view::PortfolioView;

#[async_trait]
pub trait PortfolioRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<PortfolioView>>;
    async fn find_featured(&self) -> Result<Vec<PortfolioView>>;
    async fn find_by_slug(&self, slug: &str) -> Result<Option<PortfolioView>>;
}
