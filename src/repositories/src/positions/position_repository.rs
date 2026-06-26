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
        let rows = sqlx::query_as!(
            PositionView,
            r#"SELECT id, company, role, location, employment_type, started_at, ended_at,
                      is_current, description, sort_order, created_at, updated_at
               FROM positions ORDER BY sort_order, started_at DESC"#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<PositionView>> {
        let row = sqlx::query_as!(
            PositionView,
            r#"SELECT id, company, role, location, employment_type, started_at, ended_at,
                      is_current, description, sort_order, created_at, updated_at
               FROM positions WHERE id = $1"#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }
}
