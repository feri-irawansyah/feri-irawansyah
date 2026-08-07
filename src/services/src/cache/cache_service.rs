use anyhow::Result;
use async_trait::async_trait;
use connectors::cache::CacheStore;
use modules::cache::{CacheKeyInfo, CacheService, CacheStats};
use std::sync::Arc;

use crate::cache::CacheServiceDeps;

pub struct CacheServiceImpl {
    conn: Arc<dyn CacheStore>,
}

impl CacheServiceImpl {
    pub fn new(deps: CacheServiceDeps) -> Self {
        Self { conn: deps.conn }
    }
}

#[async_trait]
impl CacheService for CacheServiceImpl {
    async fn get_stats(&self) -> Result<CacheStats> {
        self.conn.get_stats().await
    }

    async fn get_keys(&self) -> Result<Vec<CacheKeyInfo>> {
        self.conn.get_keys().await
    }

    async fn flush_all(&self) -> Result<()> {
        self.conn.flush_all().await
    }

    async fn delete_key(&self, key: String) -> Result<()> {
        self.conn.delete_key(&key).await
    }

    async fn get_raw(&self, key: &str) -> Option<String> {
        self.conn.get_raw(key).await
    }

    async fn set_raw(&self, key: &str, value: String, ttl_secs: u64) {
        self.conn.set_raw(key, value, ttl_secs).await;
    }
}
