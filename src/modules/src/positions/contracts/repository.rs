use anyhow::Result;
use async_trait::async_trait;
use crate::positions::position_command::PositionCommand;
use crate::positions::position_view::PositionRow;

#[async_trait]
pub trait PositionRepository: Send + Sync {
    async fn find_all_async(&self) -> Result<Vec<PositionRow>>;
    async fn find_page_async(&self, page: i64, per_page: i64) -> Result<(Vec<PositionRow>, i64)>;
    async fn find_by_id_async(&self, id: i32) -> Result<Option<PositionRow>>;
    async fn create_async(&self, input: PositionCommand) -> Result<PositionRow>;
    async fn update_async(&self, id: i32, input: PositionCommand) -> Result<Option<PositionRow>>;
    async fn delete_async(&self, id: i32) -> Result<bool>;
}
