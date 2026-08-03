use anyhow::Result;
use async_trait::async_trait;
use modules::experience::{ExperienceCommand, ExperienceRepository, ExperienceView};
use sqlx::PgPool;

pub struct ExperienceRepositoryImpl {
    pool: PgPool,
}

impl ExperienceRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ExperienceRepository for ExperienceRepositoryImpl {
    async fn find_all(&self) -> Result<Vec<ExperienceView>> {
        let rows = sqlx::query_as::<_, ExperienceView>(
            "SELECT id, title, company, url_docs, image_src, start_date, end_date, last_update
             FROM experience
             ORDER BY start_date DESC",
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    async fn find_page(&self, page: i64, per_page: i64) -> Result<(Vec<ExperienceView>, i64)> {
        let total: i64 = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM experience")
            .fetch_one(&self.pool)
            .await?;
        let offset = (page - 1).max(0) * per_page;
        let rows = sqlx::query_as::<_, ExperienceView>(
            "SELECT id, title, company, url_docs, image_src, start_date, end_date, last_update
             FROM experience
             ORDER BY start_date DESC
             LIMIT $1 OFFSET $2",
        )
        .bind(per_page)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;
        Ok((rows, total))
    }

    async fn find_by_ids(&self, ids: &[i32]) -> Result<Vec<ExperienceView>> {
        let rows = sqlx::query_as::<_, ExperienceView>(
            "SELECT id, title, company, url_docs, image_src, start_date, end_date, last_update
             FROM experience
             WHERE id = ANY($1)",
        )
        .bind(ids)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    async fn create(&self, input: ExperienceCommand) -> Result<ExperienceView> {
        let row = sqlx::query_as::<_, ExperienceView>(
            "INSERT INTO experience (title, company, url_docs, image_src, start_date, end_date)
             VALUES ($1, $2, $3, $4, $5, $6)
             RETURNING id, title, company, url_docs, image_src, start_date, end_date, last_update",
        )
        .bind(input.title)
        .bind(input.company)
        .bind(input.url_docs)
        .bind(input.image_src)
        .bind(input.start_date)
        .bind(input.end_date)
        .fetch_one(&self.pool)
        .await?;
        Ok(row)
    }

    async fn update(&self, id: i32, input: ExperienceCommand) -> Result<Option<ExperienceView>> {
        let row = sqlx::query_as::<_, ExperienceView>(
            "UPDATE experience
             SET title = $2, company = $3, url_docs = $4, image_src = $5,
                 start_date = $6, end_date = $7, last_update = NOW()
             WHERE id = $1
             RETURNING id, title, company, url_docs, image_src, start_date, end_date, last_update",
        )
        .bind(id)
        .bind(input.title)
        .bind(input.company)
        .bind(input.url_docs)
        .bind(input.image_src)
        .bind(input.start_date)
        .bind(input.end_date)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    async fn delete(&self, id: i32) -> Result<bool> {
        let result = sqlx::query("DELETE FROM experience WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}
