use anyhow::Result;
use async_trait::async_trait;
use crate::portfolio::portfolio_command::PortfolioCommand;
use crate::portfolio::portfolio_view::PortfolioView;

#[async_trait]
pub trait PortfolioService: Send + Sync {
    async fn list(&self) -> Result<Vec<PortfolioView>>;
    async fn list_page(&self, page: i64, per_page: i64) -> Result<(Vec<PortfolioView>, i64)>;
    async fn featured(&self) -> Result<Vec<PortfolioView>>;
    async fn get_by_slug(&self, slug: &str) -> Result<Option<PortfolioView>>;
    async fn create(&self, input: PortfolioCommand) -> Result<PortfolioView>;
    async fn update(&self, id: i32, input: PortfolioCommand) -> Result<Option<PortfolioView>>;
    async fn delete(&self, id: i32) -> Result<bool>;
}
