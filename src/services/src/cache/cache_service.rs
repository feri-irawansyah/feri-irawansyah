use anyhow::Result;
use async_trait::async_trait;
use connectors::cache::{CacheConn, CacheStore};
use modules::cache::{CacheKeyInfo, CacheService, CacheStats};

use crate::cache::CacheServiceDeps;

pub struct CacheServiceImpl {
    conn: CacheConn,
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
}
