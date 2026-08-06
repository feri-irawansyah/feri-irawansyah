use anyhow::Result;
use async_trait::async_trait;
use connectors::cache::CacheConn;
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
        let mut conn = self.conn.clone();
        connectors::cache::get_stats(&mut conn).await
    }

    async fn get_keys(&self) -> Result<Vec<CacheKeyInfo>> {
        let mut conn = self.conn.clone();
        connectors::cache::get_keys(&mut conn).await
    }

    async fn flush_all(&self) -> Result<()> {
        let mut conn = self.conn.clone();
        connectors::cache::flush_all(&mut conn).await
    }

    async fn delete_key(&self, key: String) -> Result<()> {
        let mut conn = self.conn.clone();
        connectors::cache::delete_key(&mut conn, &key).await
    }
}
