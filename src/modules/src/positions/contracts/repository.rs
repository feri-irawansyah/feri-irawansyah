use anyhow::Result;
use async_trait::async_trait;
use crate::positions::position_view::PositionView;

#[async_trait]
pub trait PositionRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<PositionView>>;
    async fn find_by_id(&self, id: i32) -> Result<Option<PositionView>>;
}
