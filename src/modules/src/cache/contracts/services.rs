use anyhow::Result;
use async_trait::async_trait;

use crate::cache::{CacheKeyInfo, CacheStats};

#[async_trait]
pub trait CacheService: Send + Sync {
    async fn get_stats(&self) -> Result<CacheStats>;
    async fn get_keys(&self) -> Result<Vec<CacheKeyInfo>>;
    async fn flush_all(&self) -> Result<()>;
    async fn delete_key(&self, key: String) -> Result<()>;
}
