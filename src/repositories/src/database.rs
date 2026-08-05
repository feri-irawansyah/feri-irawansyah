use std::time::Duration;

use anyhow::Result;
use sqlx::postgres::PgPoolOptions;

pub use sqlx::PgPool;

fn env_u32(key: &str, default: u32) -> u32 {
    std::env::var(key)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

fn env_u64(key: &str, default: u64) -> u64 {
    std::env::var(key)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

pub async fn health_check(pool: &PgPool) -> Result<()> {
    sqlx::query("SELECT 1").execute(pool).await?;
    Ok(())
}

pub async fn create_pool() -> Result<PgPool> {
    dotenvy::dotenv().ok();

    let url = std::env::var("DATABASE_URL")?;

    let pool = PgPoolOptions::new()
        .max_connections(env_u32("DB_MAX_CONNECTIONS", 10))
        .min_connections(env_u32("DB_MIN_CONNECTIONS", 1))
        .acquire_timeout(Duration::from_secs(env_u64("DB_ACQUIRE_TIMEOUT_SECS", 5)))
        .idle_timeout(Duration::from_secs(env_u64("DB_IDLE_TIMEOUT_SECS", 600)))
        .max_lifetime(Duration::from_secs(env_u64("DB_MAX_LIFETIME_SECS", 1800)))
        .connect(&url)
        .await?;

    Ok(pool)
}
