use anyhow::Result;
use async_trait::async_trait;
use modules::cache::{CacheKeyInfo, CacheStats};
use redis::AsyncCommands;
use std::collections::HashMap;
use std::sync::Mutex;

use super::client::CacheConn;
use super::store::CacheStore;

#[async_trait]
impl CacheStore for CacheConn {
    async fn get_raw(&self, key: &str) -> Option<String> {
        let mut conn = self.clone();
        match conn.get(key).await {
            Ok(v) => v,
            Err(err) => {
                tracing::warn!(%err, key, "cache get failed, falling back to db");
                None
            }
        }
    }

    async fn set_raw(&self, key: &str, value: String, ttl_secs: u64) {
        let mut conn = self.clone();
        if let Err(err) = conn.set_ex::<_, _, ()>(key, value, ttl_secs).await {
            tracing::warn!(%err, key, "cache set failed");
        }
    }

    async fn get_version(&self, domain: &str) -> i64 {
        let mut conn = self.clone();
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

    async fn bump_version(&self, domain: &str) {
        let mut conn = self.clone();
        if let Err(err) = conn.incr::<_, _, ()>(format!("{domain}:version"), 1).await {
            tracing::warn!(%err, domain, "cache version bump failed");
        }
    }

    async fn incr_with_ttl(&self, key: &str, ttl_secs: u64) -> i64 {
        let mut conn = self.clone();
        let count: i64 = match conn.incr(key, 1).await {
            Ok(c) => c,
            Err(err) => {
                tracing::warn!(%err, key, "cache incr failed, failing open");
                return 0;
            }
        };
        // NX = only set the expiry if this key doesn't already have one, so
        // a key that's still counting down from its first hit never gets its
        // window pushed back out by later hits — same fixed window either way.
        if let Err(err) = redis::cmd("EXPIRE")
            .arg(key)
            .arg(ttl_secs)
            .arg("NX")
            .query_async::<i64>(&mut conn)
            .await
        {
            tracing::warn!(%err, key, "cache incr_with_ttl: setting expiry failed");
        }
        count
    }

    async fn get_stats(&self) -> Result<CacheStats> {
        fn parse(info: &str, field: &str) -> String {
            for line in info.lines() {
                if line.starts_with('#') {
                    continue;
                }
                if let Some((k, v)) = line.split_once(':')
                    && k.trim() == field
                {
                    return v.trim().trim_end_matches('\r').to_string();
                }
            }
            String::new()
        }

        let mut conn = self.clone();
        let mem_info: String = redis::cmd("INFO")
            .arg("memory")
            .query_async(&mut conn)
            .await?;
        let stats_info: String = redis::cmd("INFO")
            .arg("stats")
            .query_async(&mut conn)
            .await?;
        let clients_info: String = redis::cmd("INFO")
            .arg("clients")
            .query_async(&mut conn)
            .await?;
        let total_keys: i64 = redis::cmd("DBSIZE").query_async(&mut conn).await?;

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

    async fn get_keys(&self) -> Result<Vec<CacheKeyInfo>> {
        let mut conn = self.clone();
        let mut all_keys: Vec<String> = Vec::new();
        let mut cursor = 0u64;
        loop {
            let (next_cursor, keys): (u64, Vec<String>) = redis::cmd("SCAN")
                .arg(cursor)
                .arg("COUNT")
                .arg(200)
                .query_async(&mut conn)
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
                .query_async::<Option<u64>>(&mut conn)
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

    async fn flush_all(&self) -> Result<()> {
        let mut conn = self.clone();
        let _: redis::Value = redis::cmd("FLUSHDB").query_async(&mut conn).await?;
        Ok(())
    }

    async fn delete_key(&self, key: &str) -> Result<()> {
        let mut conn = self.clone();
        let _: i64 = conn.del(key).await?;
        Ok(())
    }
}

/// In-memory [`CacheStore`] for unit tests — no network, no real Valkey
/// instance required.
#[derive(Default)]
pub struct MockCacheClient {
    values: Mutex<HashMap<String, String>>,
    versions: Mutex<HashMap<String, i64>>,
}

impl MockCacheClient {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl CacheStore for MockCacheClient {
    async fn get_raw(&self, key: &str) -> Option<String> {
        self.values.lock().unwrap().get(key).cloned()
    }

    async fn set_raw(&self, key: &str, value: String, _ttl_secs: u64) {
        self.values.lock().unwrap().insert(key.to_string(), value);
    }

    async fn get_version(&self, domain: &str) -> i64 {
        *self.versions.lock().unwrap().get(domain).unwrap_or(&0)
    }

    async fn bump_version(&self, domain: &str) {
        *self
            .versions
            .lock()
            .unwrap()
            .entry(domain.to_string())
            .or_insert(0) += 1;
    }

    async fn incr_with_ttl(&self, key: &str, _ttl_secs: u64) -> i64 {
        // No real expiry semantics here (same simplification `set_raw`
        // already makes) — fine for unit tests, which don't wait out a real
        // window, only assert on the count.
        let mut values = self.values.lock().unwrap();
        let count = values
            .get(key)
            .and_then(|v| v.parse::<i64>().ok())
            .unwrap_or(0)
            + 1;
        values.insert(key.to_string(), count.to_string());
        count
    }

    async fn get_stats(&self) -> Result<CacheStats> {
        let total_keys = self.values.lock().unwrap().len() as i64;
        Ok(CacheStats {
            used_bytes: 0,
            used_human: "0 B".to_string(),
            peak_bytes: 0,
            peak_human: "0 B".to_string(),
            max_bytes: 0,
            fragmentation_ratio: 1.0,
            total_keys,
            connected_clients: 1,
            hits: 0,
            misses: 0,
        })
    }

    async fn get_keys(&self) -> Result<Vec<CacheKeyInfo>> {
        Ok(self
            .values
            .lock()
            .unwrap()
            .keys()
            .map(|key| CacheKeyInfo {
                key: key.clone(),
                ttl_secs: -1,
                mem_bytes: 0,
            })
            .collect())
    }

    async fn flush_all(&self) -> Result<()> {
        self.values.lock().unwrap().clear();
        self.versions.lock().unwrap().clear();
        Ok(())
    }

    async fn delete_key(&self, key: &str) -> Result<()> {
        self.values.lock().unwrap().remove(key);
        Ok(())
    }
}

/// [`CacheStore`] used when Valkey couldn't be reached at boot — see
/// [`super::client::connect_or_degraded`]. Cache is meant to be an
/// optimization, not a hard dependency: reads always miss (falling straight
/// through to the DB, same code path as a normal cache miss) and writes are
/// silent no-ops, so every other service keeps working unmodified. The
/// admin-facing ops report an error instead of pretending to have numbers,
/// since there's no real backing store to report on.
pub struct UnavailableCacheStore;

#[async_trait]
impl CacheStore for UnavailableCacheStore {
    async fn get_raw(&self, _key: &str) -> Option<String> {
        None
    }

    async fn set_raw(&self, _key: &str, _value: String, _ttl_secs: u64) {}

    async fn get_version(&self, _domain: &str) -> i64 {
        0
    }

    async fn bump_version(&self, _domain: &str) {}

    async fn incr_with_ttl(&self, _key: &str, _ttl_secs: u64) -> i64 {
        0
    }

    async fn get_stats(&self) -> Result<CacheStats> {
        Err(anyhow::anyhow!("cache unavailable — valkey unreachable at boot"))
    }

    async fn get_keys(&self) -> Result<Vec<CacheKeyInfo>> {
        Err(anyhow::anyhow!("cache unavailable — valkey unreachable at boot"))
    }

    async fn flush_all(&self) -> Result<()> {
        Err(anyhow::anyhow!("cache unavailable — valkey unreachable at boot"))
    }

    async fn delete_key(&self, _key: &str) -> Result<()> {
        Err(anyhow::anyhow!("cache unavailable — valkey unreachable at boot"))
    }
}

#[cfg(test)]
#[path = "_store_impl_test.rs"]
mod tests;
