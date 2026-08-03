use anyhow::Result;
use async_trait::async_trait;
use modules::portfolio::{PortfolioCommand, PortfolioRepository, PortfolioView};
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

    async fn find_page(&self, page: i64, per_page: i64) -> Result<(Vec<PortfolioView>, i64)> {
        let total: i64 = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM portfolio")
            .fetch_one(&self.pool)
            .await?;
        let offset = (page - 1).max(0) * per_page;
        let rows = sqlx::query_as::<_, PortfolioView>(
            "SELECT portfolio_id, title, slug, description, url_docs, image_src,
                    tech, pined, sort_order, details, last_update
             FROM portfolio ORDER BY sort_order, last_update DESC
             LIMIT $1 OFFSET $2",
        )
        .bind(per_page)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;
        Ok((rows, total))
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

    async fn create(&self, input: PortfolioCommand) -> Result<PortfolioView> {
        let row = sqlx::query_as::<_, PortfolioView>(
            "INSERT INTO portfolio
                (title, slug, description, url_docs, image_src, tech, pined, sort_order, details)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
             RETURNING portfolio_id, title, slug, description, url_docs, image_src,
                       tech, pined, sort_order, details, last_update",
        )
        .bind(input.title)
        .bind(input.slug)
        .bind(input.description)
        .bind(input.url_docs)
        .bind(input.image_src)
        .bind(input.tech)
        .bind(input.pined)
        .bind(input.sort_order)
        .bind(input.details)
        .fetch_one(&self.pool)
        .await?;
        Ok(row)
    }

    async fn update(&self, id: i32, input: PortfolioCommand) -> Result<Option<PortfolioView>> {
        let row = sqlx::query_as::<_, PortfolioView>(
            "UPDATE portfolio
             SET title = $2, slug = $3, description = $4, url_docs = $5, image_src = $6,
                 tech = $7, pined = $8, sort_order = $9, details = $10, last_update = NOW()
             WHERE portfolio_id = $1
             RETURNING portfolio_id, title, slug, description, url_docs, image_src,
                       tech, pined, sort_order, details, last_update",
        )
        .bind(id)
        .bind(input.title)
        .bind(input.slug)
        .bind(input.description)
        .bind(input.url_docs)
        .bind(input.image_src)
        .bind(input.tech)
        .bind(input.pined)
        .bind(input.sort_order)
        .bind(input.details)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    async fn delete(&self, id: i32) -> Result<bool> {
        let result = sqlx::query("DELETE FROM portfolio WHERE portfolio_id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}
