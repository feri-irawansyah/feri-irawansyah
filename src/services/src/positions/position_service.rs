use anyhow::Result;
use async_trait::async_trait;
use modules::positions::{PositionService, PositionView};
use std::sync::Arc;

use crate::positions::PositionServiceDeps;

pub struct PositionServiceImpl {
    repo: Arc<dyn modules::positions::PositionRepository>,
}

impl PositionServiceImpl {
    pub fn new(deps: PositionServiceDeps) -> Self {
        Self { repo: deps.position_repo }
    }
}

#[async_trait]
impl PositionService for PositionServiceImpl {
    async fn list(&self) -> Result<Vec<PositionView>> {
        self.repo.find_all().await
    }

    async fn get(&self, id: i32) -> Result<Option<PositionView>> {
        self.repo.find_by_id(id).await
    }
}
