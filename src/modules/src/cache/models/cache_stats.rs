use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CacheStats {
    pub used_bytes: u64,
    pub used_human: String,
    pub peak_bytes: u64,
    pub peak_human: String,
    pub max_bytes: u64,
    pub fragmentation_ratio: f64,
    pub total_keys: i64,
    pub connected_clients: i64,
    pub hits: u64,
    pub misses: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CacheKeyInfo {
    pub key: String,
    pub ttl_secs: i64,
    pub mem_bytes: u64,
}
