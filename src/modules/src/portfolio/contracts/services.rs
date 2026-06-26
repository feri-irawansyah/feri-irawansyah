use anyhow::Result;
use async_trait::async_trait;
use crate::portfolio::portfolio_view::PortfolioView;

#[async_trait]
pub trait PortfolioService: Send + Sync {
    async fn list(&self) -> Result<Vec<PortfolioView>>;
    async fn featured(&self) -> Result<Vec<PortfolioView>>;
    async fn get_by_slug(&self, slug: &str) -> Result<Option<PortfolioView>>;
}
