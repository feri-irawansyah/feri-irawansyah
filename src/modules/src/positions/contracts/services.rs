use anyhow::Result;
use async_trait::async_trait;
use crate::positions::position_command::PositionCommand;
use crate::positions::position_view::PositionView;

#[async_trait]
pub trait PositionService: Send + Sync {
    async fn list(&self) -> Result<Vec<PositionView>>;
    async fn list_page(&self, page: i64, per_page: i64) -> Result<(Vec<PositionView>, i64)>;
    async fn get(&self, id: i32) -> Result<Option<PositionView>>;
    async fn create(&self, input: PositionCommand) -> Result<PositionView>;
    async fn update(&self, id: i32, input: PositionCommand) -> Result<Option<PositionView>>;
    async fn delete(&self, id: i32) -> Result<bool>;
}
