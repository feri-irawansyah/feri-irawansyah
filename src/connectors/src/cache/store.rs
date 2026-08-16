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

    /// Atomically increments `key` and returns the new count, setting `key`
    /// to expire in `ttl_secs` the first time it's created (subsequent calls
    /// don't push the expiry back out — same key keeps counting down to a
    /// single fixed window rather than resetting on every hit). Built for
    /// rate limiting (e.g. failed-login counters — see `AuthServiceImpl`),
    /// but generically useful anywhere a distributed counter-with-expiry is
    /// needed. On a cache error, returns 0 (fails open: a rate limiter that
    /// can lock everyone out because the cache hiccuped is worse than one
    /// that occasionally under-counts).
    async fn incr_with_ttl(&self, key: &str, ttl_secs: u64) -> i64;

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

/// Shrinks an unbounded key part (e.g. a note slug) down to a fixed-length
/// hex digest, for use as a `versioned_key` part. Use this instead of the
/// raw value whenever the source string's length isn't bounded by app logic
/// (slugs mirror titles and can run long) — the raw value is still what's
/// looked up against, so the hash only needs to be deterministic, not secure.
/// FNV-1a is used over `DefaultHasher` because its algorithm is fixed by us,
/// not an unspecified std implementation detail.
pub fn hash_part(input: &str) -> String {
    const FNV_OFFSET: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;

    let mut hash = FNV_OFFSET;
    for byte in input.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    format!("{hash:016x}").to_string()
}

#[cfg(test)]
#[path = "_store_test.rs"]
mod tests;
