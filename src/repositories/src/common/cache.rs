use anyhow::Result;
use modules::cache::{CacheKeyInfo, CacheStats};
use redis::AsyncCommands;
use serde::{Serialize, de::DeserializeOwned};

pub use redis::aio::MultiplexedConnection as CacheConn;

const DEFAULT_TTL_SECS: u64 = 7200;

// pub async fn create_cache_client() -> Result<CacheConn> {
//     dotenvy::dotenv().ok();

//     let url = std::env::var("VALKEY_URL")?;
//     let client = redis::Client::open(url)?;
//     let manager = client.get_connection_manager().await?;

//     Ok(manager)
// }

pub async fn create_cache_client() -> anyhow::Result<CacheConn> {
    dotenvy::dotenv().ok();

    let url = std::env::var("VALKEY_URL")?;
    tracing::debug!(url, "connecting to valkey");

    let client = redis::Client::open(url)?;
    tracing::debug!("valkey client created");

    let mut conn = client.get_multiplexed_async_connection().await?;
    tracing::debug!("valkey connection established");

    let pong: String = redis::cmd("PING").query_async(&mut conn).await?;
    tracing::debug!(pong, "valkey ping ok");

    Ok(conn)
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
    match conn
        .get::<_, Option<i64>>(format!("{domain}:version"))
        .await
    {
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

pub async fn get_stats(conn: &mut CacheConn) -> Result<CacheStats> {
    fn parse(info: &str, field: &str) -> String {
        for line in info.lines() {
            if line.starts_with('#') {
                continue;
            }
            if let Some((k, v)) = line.split_once(':') {
                if k.trim() == field {
                    return v.trim().trim_end_matches('\r').to_string();
                }
            }
        }
        String::new()
    }

    let mem_info: String = redis::cmd("INFO").arg("memory").query_async(conn).await?;
    let stats_info: String = redis::cmd("INFO").arg("stats").query_async(conn).await?;
    let clients_info: String = redis::cmd("INFO").arg("clients").query_async(conn).await?;
    let total_keys: i64 = redis::cmd("DBSIZE").query_async(conn).await?;

    Ok(CacheStats {
        used_bytes: parse(&mem_info, "used_memory").parse().unwrap_or(0),
        used_human: parse(&mem_info, "used_memory_human"),
        peak_bytes: parse(&mem_info, "used_memory_peak").parse().unwrap_or(0),
        peak_human: parse(&mem_info, "used_memory_peak_human"),
        max_bytes: parse(&mem_info, "maxmemory").parse().unwrap_or(0),
        fragmentation_ratio: parse(&mem_info, "mem_fragmentation_ratio")
            .parse()
            .unwrap_or(1.0),
        total_keys,
        connected_clients: parse(&clients_info, "connected_clients")
            .parse()
            .unwrap_or(0),
        hits: parse(&stats_info, "keyspace_hits").parse().unwrap_or(0),
        misses: parse(&stats_info, "keyspace_misses").parse().unwrap_or(0),
    })
}

pub async fn get_keys(conn: &mut CacheConn) -> Result<Vec<CacheKeyInfo>> {
    let mut all_keys: Vec<String> = Vec::new();
    let mut cursor = 0u64;
    loop {
        let (next_cursor, keys): (u64, Vec<String>) = redis::cmd("SCAN")
            .arg(cursor)
            .arg("COUNT")
            .arg(200)
            .query_async(conn)
            .await?;
        all_keys.extend(keys);
        cursor = next_cursor;
        if cursor == 0 || all_keys.len() >= 500 {
            break;
        }
    }

    let mut result: Vec<CacheKeyInfo> = Vec::new();
    for key in all_keys.into_iter().take(500) {
        let ttl: i64 = conn.ttl(&key).await.unwrap_or(-1);
        let mem: u64 = redis::cmd("MEMORY")
            .arg("USAGE")
            .arg(&key)
            .query_async::<Option<u64>>(conn)
            .await
            .unwrap_or(None)
            .unwrap_or(0);
        result.push(CacheKeyInfo {
            key,
            ttl_secs: ttl,
            mem_bytes: mem,
        });
    }

    result.sort_by(|a, b| b.mem_bytes.cmp(&a.mem_bytes));
    Ok(result)
}

pub async fn flush_all(conn: &mut CacheConn) -> Result<()> {
    let _: redis::Value = redis::cmd("FLUSHDB").query_async(conn).await?;
    Ok(())
}

pub async fn delete_key(conn: &mut CacheConn, key: &str) -> Result<()> {
    let _: i64 = conn.del(key).await?;
    Ok(())
}
