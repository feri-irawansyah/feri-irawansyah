use anyhow::Result;
use async_trait::async_trait;
use crate::positions::position_command::PositionCommand;
use crate::positions::position_view::PositionView;

#[async_trait]
pub trait PositionService: Send + Sync {
    async fn find_all_async(&self) -> Result<Vec<PositionView>>;
    async fn find_page_async(&self, page: i64, per_page: i64) -> Result<(Vec<PositionView>, i64)>;
    async fn find_by_id_async(&self, id: i32) -> Result<Option<PositionView>>;
    async fn create_async(&self, input: PositionCommand) -> Result<PositionView>;
    async fn update_async(&self, id: i32, input: PositionCommand) -> Result<Option<PositionView>>;
    async fn delete_async(&self, id: i32) -> Result<bool>;
}
