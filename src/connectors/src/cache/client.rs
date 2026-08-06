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
