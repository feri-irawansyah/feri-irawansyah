use anyhow::Result;
use async_trait::async_trait;
use modules::positions::{PositionRepository, PositionView};
use sqlx::PgPool;

pub struct PositionRepositoryImpl {
    pool: PgPool,
}

impl PositionRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PositionRepository for PositionRepositoryImpl {
    async fn find_all(&self) -> Result<Vec<PositionView>> {
        let rows = sqlx::query_as::<_, PositionView>(
            "SELECT p.id, p.title,
                    e.company, e.url_docs, e.image_src,
                    p.address, p.start_date, p.end_date,
                    p.description, p.job_position, p.job_type, p.sort_order
             FROM positions p
             JOIN experience e ON e.id = p.experience_id
             ORDER BY p.start_date DESC",
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<PositionView>> {
        let row = sqlx::query_as::<_, PositionView>(
            "SELECT p.id, p.title,
                    e.company, e.url_docs, e.image_src,
                    p.address, p.start_date, p.end_date,
                    p.description, p.job_position, p.job_type, p.sort_order
             FROM positions p
             JOIN experience e ON e.id = p.experience_id
             WHERE p.id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }
}
