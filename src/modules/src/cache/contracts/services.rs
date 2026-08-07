use anyhow::Result;
use async_trait::async_trait;

use crate::cache::{CacheKeyInfo, CacheStats};

#[async_trait]
pub trait CacheService: Send + Sync {
    async fn get_stats(&self) -> Result<CacheStats>;
    async fn get_keys(&self) -> Result<Vec<CacheKeyInfo>>;
    async fn flush_all(&self) -> Result<()>;
    async fn delete_key(&self, key: String) -> Result<()>;

    /// Generic read/write for callers that just need a cache slot, not the
    /// admin-facing stats/management ops above — e.g. `views::markdown`
    /// caching rendered content fetched from GitHub, keeping `views` off
    /// `connectors` directly (pages only ever reach cache/DB through a
    /// service trait object).
    async fn get_raw(&self, key: &str) -> Option<String>;
    async fn set_raw(&self, key: &str, value: String, ttl_secs: u64);
}
