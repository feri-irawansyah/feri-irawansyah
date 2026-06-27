use anyhow::Result;
use async_trait::async_trait;
use modules::portfolio::{PortfolioRepository, PortfolioView};
use sqlx::PgPool;

pub struct PortfolioRepositoryImpl {
    pool: PgPool,
}

impl PortfolioRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PortfolioRepository for PortfolioRepositoryImpl {
    async fn find_all(&self) -> Result<Vec<PortfolioView>> {
        let rows = sqlx::query_as::<_, PortfolioView>(
            "SELECT portfolio_id, title, slug, description, url_docs, image_src,
                    tech, pined, sort_order, details, last_update
             FROM portfolio ORDER BY sort_order, last_update DESC",
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    async fn find_featured(&self) -> Result<Vec<PortfolioView>> {
        let rows = sqlx::query_as::<_, PortfolioView>(
            "SELECT portfolio_id, title, slug, description, url_docs, image_src,
                    tech, pined, sort_order, details, last_update
             FROM portfolio WHERE pined = TRUE ORDER BY sort_order",
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    async fn find_by_slug(&self, slug: &str) -> Result<Option<PortfolioView>> {
        let row = sqlx::query_as::<_, PortfolioView>(
            "SELECT portfolio_id, title, slug, description, url_docs, image_src,
                    tech, pined, sort_order, details, last_update
             FROM portfolio WHERE slug = $1",
        )
        .bind(slug)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }
}
