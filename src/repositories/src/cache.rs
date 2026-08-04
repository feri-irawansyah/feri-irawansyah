use anyhow::Result;
use redis::AsyncCommands;
use serde::{Serialize, de::DeserializeOwned};

pub use redis::aio::ConnectionManager as CacheConn;

const DEFAULT_TTL_SECS: u64 = 600;

pub async fn create_cache_client() -> Result<CacheConn> {
    dotenvy::dotenv().ok();

    let url = std::env::var("VALKEY_URL")?;
    let client = redis::Client::open(url)?;
    let manager = client.get_connection_manager().await?;

    Ok(manager)
}

/// Cache-aside read: any Valkey error or miss falls through to `None` so the
/// caller always has a working path to Postgres. Never let a cache failure
/// fail the request.
pub async fn get_cached<T: DeserializeOwned>(conn: &CacheConn, key: &str) -> Option<T> {
    let mut conn = conn.clone();
    let raw: Option<String> = match conn.get(key).await {
        Ok(v) => v,
        Err(err) => {
            tracing::warn!(%err, key, "cache get failed, falling back to db");
            return None;
        }
    };

    raw.and_then(|s| match serde_json::from_str(&s) {
        Ok(v) => Some(v),
        Err(err) => {
            tracing::warn!(%err, key, "cache deserialize failed");
            None
        }
    })
}

pub async fn set_cached<T: Serialize>(conn: &CacheConn, key: &str, value: &T) {
    let mut conn = conn.clone();
    let Ok(raw) = serde_json::to_string(value) else {
        return;
    };
    if let Err(err) = conn.set_ex::<_, _, ()>(key, raw, DEFAULT_TTL_SECS).await {
        tracing::warn!(%err, key, "cache set failed");
    }
}

/// Current cache "epoch" for a domain. Read-side keys embed this version so
/// a write can invalidate every paginated/filtered variant at once via
/// `bump_cache_version` instead of enumerating and deleting each key.
pub async fn cache_version(conn: &CacheConn, domain: &str) -> i64 {
    let mut conn = conn.clone();
    match conn.get::<_, Option<i64>>(format!("{domain}:version")).await {
        Ok(Some(v)) => v,
        Ok(None) => 0,
        Err(err) => {
            tracing::warn!(%err, domain, "cache version read failed");
            0
        }
    }
}

pub async fn bump_cache_version(conn: &CacheConn, domain: &str) {
    let mut conn = conn.clone();
    if let Err(err) = conn.incr::<_, _, ()>(format!("{domain}:version"), 1).await {
        tracing::warn!(%err, domain, "cache version bump failed");
    }
}

pub fn versioned_key(domain: &str, version: i64, parts: &[&str]) -> String {
    let mut key = format!("{domain}:v{version}");
    for part in parts {
        key.push(':');
        key.push_str(part);
    }
    key
}
