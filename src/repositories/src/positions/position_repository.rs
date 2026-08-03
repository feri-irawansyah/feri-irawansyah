use anyhow::Result;
use async_trait::async_trait;
use modules::positions::{PositionCommand, PositionRepository, PositionRow};
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
    async fn find_all(&self) -> Result<Vec<PositionRow>> {
        let rows = sqlx::query_as::<_, PositionRow>(
            "SELECT id, experience_id, title, address, start_date, end_date,
                    description, job_position, job_type, sort_order
             FROM positions
             ORDER BY start_date DESC",
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    async fn find_page(&self, page: i64, per_page: i64) -> Result<(Vec<PositionRow>, i64)> {
        let total: i64 = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM positions")
            .fetch_one(&self.pool)
            .await?;
        let offset = (page - 1).max(0) * per_page;
        let rows = sqlx::query_as::<_, PositionRow>(
            "SELECT id, experience_id, title, address, start_date, end_date,
                    description, job_position, job_type, sort_order
             FROM positions
             ORDER BY start_date DESC
             LIMIT $1 OFFSET $2",
        )
        .bind(per_page)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;
        Ok((rows, total))
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<PositionRow>> {
        let row = sqlx::query_as::<_, PositionRow>(
            "SELECT id, experience_id, title, address, start_date, end_date,
                    description, job_position, job_type, sort_order
             FROM positions
             WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    async fn create(&self, input: PositionCommand) -> Result<PositionRow> {
        let row = sqlx::query_as::<_, PositionRow>(
            "INSERT INTO positions
                (experience_id, title, start_date, end_date, description, address, job_position, job_type, sort_order)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
             RETURNING id, experience_id, title, address, start_date, end_date,
                       description, job_position, job_type, sort_order",
        )
        .bind(input.experience_id)
        .bind(input.title)
        .bind(input.start_date)
        .bind(input.end_date)
        .bind(input.description)
        .bind(input.address)
        .bind(input.job_position)
        .bind(input.job_type)
        .bind(input.sort_order)
        .fetch_one(&self.pool)
        .await?;
        Ok(row)
    }

    async fn update(&self, id: i32, input: PositionCommand) -> Result<Option<PositionRow>> {
        let row = sqlx::query_as::<_, PositionRow>(
            "UPDATE positions
             SET experience_id = $2, title = $3, start_date = $4, end_date = $5,
                 description = $6, address = $7, job_position = $8, job_type = $9, sort_order = $10
             WHERE id = $1
             RETURNING id, experience_id, title, address, start_date, end_date,
                       description, job_position, job_type, sort_order",
        )
        .bind(id)
        .bind(input.experience_id)
        .bind(input.title)
        .bind(input.start_date)
        .bind(input.end_date)
        .bind(input.description)
        .bind(input.address)
        .bind(input.job_position)
        .bind(input.job_type)
        .bind(input.sort_order)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    async fn delete(&self, id: i32) -> Result<bool> {
        let result = sqlx::query("DELETE FROM positions WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}
