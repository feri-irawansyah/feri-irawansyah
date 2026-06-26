use std::sync::Arc;

pub mod position_service;
pub use position_service::PositionServiceImpl;

pub struct PositionServiceDeps {
    pub position_repo: Arc<dyn modules::positions::PositionRepository>,
}
