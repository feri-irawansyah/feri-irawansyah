use std::sync::Arc;

pub mod portfolio_service;
pub use portfolio_service::PortfolioServiceImpl;

pub struct PortfolioServiceDeps {
    pub portfolio_repo: Arc<dyn modules::portfolio::PortfolioRepository>,
}
