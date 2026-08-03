use anyhow::Result;
use async_trait::async_trait;
use crate::portfolio::portfolio_command::PortfolioCommand;
use crate::portfolio::portfolio_view::PortfolioView;

#[async_trait]
pub trait PortfolioRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<PortfolioView>>;
    async fn find_page(&self, page: i64, per_page: i64) -> Result<(Vec<PortfolioView>, i64)>;
    async fn find_featured(&self) -> Result<Vec<PortfolioView>>;
    async fn find_by_slug(&self, slug: &str) -> Result<Option<PortfolioView>>;
    async fn create(&self, input: PortfolioCommand) -> Result<PortfolioView>;
    async fn update(&self, id: i32, input: PortfolioCommand) -> Result<Option<PortfolioView>>;
    async fn delete(&self, id: i32) -> Result<bool>;
}
