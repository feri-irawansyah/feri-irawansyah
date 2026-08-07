use super::*;

#[tokio::test]
async fn unavailable_cache_always_misses_on_read() {
    let store = UnavailableCacheStore;
    assert!(store.get_raw("any").await.is_none());
}

#[tokio::test]
async fn unavailable_cache_write_is_a_silent_no_op() {
    let store = UnavailableCacheStore;
    // Must not panic, and a get right after must still be a miss — there's
    // no real storage behind it to have written to.
    store.set_raw("k", "v".to_string(), 60).await;
    assert!(store.get_raw("k").await.is_none());
}

#[tokio::test]
async fn unavailable_cache_version_defaults_to_zero_and_bump_is_a_no_op() {
    let store = UnavailableCacheStore;
    assert_eq!(store.get_version("notes").await, 0);
    store.bump_version("notes").await;
    assert_eq!(store.get_version("notes").await, 0);
}

#[tokio::test]
async fn unavailable_cache_admin_ops_report_errors_not_fake_success() {
    let store = UnavailableCacheStore;
    assert!(store.get_stats().await.is_err());
    assert!(store.get_keys().await.is_err());
    assert!(store.flush_all().await.is_err());
    assert!(store.delete_key("k").await.is_err());
}

#[tokio::test]
async fn unavailable_cache_incr_with_ttl_fails_open_at_zero() {
    // A rate limiter reading this as "count" must never see it as "locked" —
    // 0 always stays under any real threshold.
    let store = UnavailableCacheStore;
    assert_eq!(store.incr_with_ttl("login-attempts:1.2.3.4", 900).await, 0);
}

#[tokio::test]
async fn mock_cache_incr_with_ttl_counts_up_from_one() {
    let store = MockCacheClient::new();
    assert_eq!(store.incr_with_ttl("k", 60).await, 1);
    assert_eq!(store.incr_with_ttl("k", 60).await, 2);
    assert_eq!(store.incr_with_ttl("k", 60).await, 3);
}

#[tokio::test]
async fn mock_cache_incr_with_ttl_keys_are_independent() {
    let store = MockCacheClient::new();
    store.incr_with_ttl("a", 60).await;
    store.incr_with_ttl("a", 60).await;
    assert_eq!(store.incr_with_ttl("b", 60).await, 1);
    assert_eq!(store.get_raw("a").await.as_deref(), Some("2"));
}
