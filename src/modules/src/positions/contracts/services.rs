use anyhow::Result;
use async_trait::async_trait;
use crate::positions::position_view::PositionView;

#[async_trait]
pub trait PositionService: Send + Sync {
    async fn list(&self) -> Result<Vec<PositionView>>;
    async fn get(&self, id: i32) -> Result<Option<PositionView>>;
}
