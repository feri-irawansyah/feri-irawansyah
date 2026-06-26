use anyhow::Result;
use sqlx::postgres::PgPoolOptions;

pub use sqlx::PgPool;

pub async fn create_pool() -> Result<PgPool> {
    dotenvy::dotenv().ok();

    let url = std::env::var("DATABASE_URL")?;

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&url)
        .await?;

    Ok(pool)
}
