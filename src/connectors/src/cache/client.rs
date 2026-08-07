use std::sync::Arc;

use super::store::CacheStore;
use super::store_impl::UnavailableCacheStore;

pub use redis::aio::MultiplexedConnection as CacheConn;

pub async fn create_cache_client() -> anyhow::Result<CacheConn> {
    dotenvy::dotenv().ok();

    let url = std::env::var("VALKEY_URL")?;
    tracing::debug!(url, "connecting to valkey");

    let client = redis::Client::open(url.as_str())?;
    tracing::debug!("valkey client created");

    let mut conn = client.get_multiplexed_async_connection().await?;
    tracing::debug!("valkey connection established");

    let pong: String = redis::cmd("PING").query_async(&mut conn).await?;
    tracing::info!(pong, "valkey connected");

    Ok(conn)
}

/// Connects to Valkey but never fails boot — a cache is meant to be an
/// optimization, not a hard dependency for the app to run at all. On
/// failure, logs loudly (this needs eyes — it's not a routine per-request
/// cache miss) and hands back [`UnavailableCacheStore`], which makes every
/// cache read a miss (straight through to the DB) until the process is
/// restarted with Valkey reachable again.
pub async fn connect_or_degraded() -> Arc<dyn CacheStore> {
    match create_cache_client().await {
        Ok(conn) => Arc::new(conn),
        Err(err) => {
            tracing::error!(
                %err,
                "valkey unreachable at boot — starting anyway with cache disabled \
                 (every read falls through to the DB; admin cache page will report \
                 errors until this is fixed and the process is restarted)"
            );
            Arc::new(UnavailableCacheStore)
        }
    }
}
