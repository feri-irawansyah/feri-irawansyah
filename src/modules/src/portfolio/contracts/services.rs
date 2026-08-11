use anyhow::Result;
use async_trait::async_trait;
use crate::portfolio::portfolio_command::PortfolioCommand;
use crate::portfolio::portfolio_view::PortfolioView;

#[async_trait]
pub trait PortfolioService: Send + Sync {
    async fn find_all_async(&self) -> Result<Vec<PortfolioView>>;
    async fn find_page_async(&self, page: i64, per_page: i64) -> Result<(Vec<PortfolioView>, i64)>;
    async fn find_featured_async(&self) -> Result<Vec<PortfolioView>>;
    async fn find_by_slug_async(&self, slug: &str) -> Result<Option<PortfolioView>>;
    async fn create_async(&self, input: PortfolioCommand) -> Result<PortfolioView>;
    async fn update_async(&self, id: i32, input: PortfolioCommand) -> Result<Option<PortfolioView>>;
    async fn delete_async(&self, id: i32) -> Result<bool>;
}
