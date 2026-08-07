use crate::markdown::{HeadingItem, MarkdownResult, process_localized_cached};
use async_trait::async_trait;
use modules::cache::{CacheKeyInfo, CacheService, CacheStats};
use std::sync::Mutex;

/// Minimal in-memory `CacheService` double — only `get_raw`/`set_raw` matter
/// here, the admin-facing ops just aren't exercised by `process_localized_cached`.
struct MockCacheService {
    store: Mutex<Option<(String, String)>>, // (key, value) — single slot is enough for these tests
}

impl MockCacheService {
    fn seeded(key: &str, value: String) -> Self {
        Self { store: Mutex::new(Some((key.to_string(), value))) }
    }
}

#[async_trait]
impl CacheService for MockCacheService {
    async fn get_stats(&self) -> anyhow::Result<CacheStats> {
        unreachable!("not used by process_localized_cached")
    }
    async fn get_keys(&self) -> anyhow::Result<Vec<CacheKeyInfo>> {
        unreachable!("not used by process_localized_cached")
    }
    async fn flush_all(&self) -> anyhow::Result<()> {
        unreachable!("not used by process_localized_cached")
    }
    async fn delete_key(&self, _key: String) -> anyhow::Result<()> {
        unreachable!("not used by process_localized_cached")
    }

    async fn get_raw(&self, key: &str) -> Option<String> {
        self.store
            .lock()
            .unwrap()
            .as_ref()
            .filter(|(k, _)| k == key)
            .map(|(_, v)| v.clone())
    }

    async fn set_raw(&self, key: &str, value: String, _ttl_secs: u64) {
        *self.store.lock().unwrap() = Some((key.to_string(), value));
    }
}

#[tokio::test]
async fn cache_hit_returns_cached_result_without_fetching() {
    let cached = MarkdownResult {
        html: "<p>cached</p>".to_string(),
        headings: vec![HeadingItem { level: 2, text: "Heading".to_string(), id: "heading".to_string() }],
    };
    let raw = serde_json::to_string(&cached).unwrap();
    let cache = MockCacheService::seeded("k", raw);

    // A garbage URL would make the miss-path's real `process_localized` fetch
    // fail (or hang) — since this returns the seeded value instead, the hit
    // path never touches `url` at all.
    let result = process_localized_cached(&cache, "k", "http://example.invalid/x.md", "id", 60)
        .await
        .unwrap();

    assert_eq!(result, cached);
}

#[tokio::test]
async fn corrupted_cache_entry_falls_through_to_fetch_and_errors_on_bad_url() {
    let cache = MockCacheService::seeded("k", "not valid json".to_string());

    // Garbage stored value must NOT be trusted as-is — falls through to a
    // real fetch, which then fails against this deliberately invalid URL.
    let result = process_localized_cached(&cache, "k", "http://example.invalid/x.md", "id", 60).await;

    assert!(result.is_err());
}

