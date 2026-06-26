use anyhow::Result;
use async_trait::async_trait;
use modules::experiences::{ExperienceRepository, ExperienceView};
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
        let rows = sqlx::query_as!(
            ExperienceView,
            r#"SELECT id, company, role, started_at, ended_at, description, is_current, sort_order
               FROM experiences ORDER BY sort_order, started_at DESC"#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<ExperienceView>> {
        let row = sqlx::query_as!(
            ExperienceView,
            r#"SELECT id, company, role, started_at, ended_at, description, is_current, sort_order
               FROM experiences WHERE id = $1"#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }
}
