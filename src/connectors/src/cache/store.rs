use anyhow::Result;
use async_trait::async_trait;
use modules::cache::{CacheKeyInfo, CacheStats};
use serde::{Serialize, de::DeserializeOwned};

const DEFAULT_TTL_SECS: u64 = 7200;

/// Abstraction over the handful of cache operations the service layer needs.
/// Real callers get this via [`CacheConn`](super::client::CacheConn)
/// (blanket-implemented in [`store_impl`](super::store_impl)); tests can
/// substitute [`MockCacheClient`](super::store_impl::MockCacheClient)
/// instead of standing up a real Valkey instance.
#[async_trait]
pub trait CacheStore: Send + Sync {
    async fn get_raw(&self, key: &str) -> Option<String>;
    async fn set_raw(&self, key: &str, value: String, ttl_secs: u64);
    async fn get_version(&self, domain: &str) -> i64;
    async fn bump_version(&self, domain: &str);

    // ── Admin/monitoring ops, used by the cache admin page ─────────────────
    async fn get_stats(&self) -> Result<CacheStats>;
    async fn get_keys(&self) -> Result<Vec<CacheKeyInfo>>;
    async fn flush_all(&self) -> Result<()>;
    async fn delete_key(&self, key: &str) -> Result<()>;
}

pub async fn get_cached<T: DeserializeOwned>(store: &dyn CacheStore, key: &str) -> Option<T> {
    let raw = store.get_raw(key).await?;
    match serde_json::from_str(&raw) {
        Ok(v) => Some(v),
        Err(err) => {
            tracing::warn!(%err, key, "cache deserialize failed");
            None
        }
    }
}

pub async fn set_cached<T: Serialize>(store: &dyn CacheStore, key: &str, value: &T) {
    let Ok(raw) = serde_json::to_string(value) else {
        return;
    };
    store.set_raw(key, raw, DEFAULT_TTL_SECS).await;
}

/// Current cache "epoch" for a domain. Read-side keys embed this version so
/// a write can invalidate every paginated/filtered variant at once via
/// `bump_cache_version` instead of enumerating and deleting each key.
pub async fn cache_version(store: &dyn CacheStore, domain: &str) -> i64 {
    store.get_version(domain).await
}

pub async fn bump_cache_version(store: &dyn CacheStore, domain: &str) {
    store.bump_version(domain).await;
}

pub fn versioned_key(domain: &str, version: i64, parts: &[&str]) -> String {
    let mut key = format!("{domain}:v{version}");
    for part in parts {
        key.push(':');
        key.push_str(part);
    }
    key
}
