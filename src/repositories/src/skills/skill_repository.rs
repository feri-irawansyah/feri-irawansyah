use anyhow::Result;
use async_trait::async_trait;
use modules::skills::{SkillRepository, SkillView};
use sqlx::PgPool;

pub struct SkillRepositoryImpl {
    pool: PgPool,
}

impl SkillRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SkillRepository for SkillRepositoryImpl {
    async fn find_all(&self) -> Result<Vec<SkillView>> {
        let rows = sqlx::query_as!(
            SkillView,
            r#"SELECT id, name, category, level, sort_order
               FROM skills ORDER BY category, sort_order"#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    async fn find_by_category(&self, category: &str) -> Result<Vec<SkillView>> {
        let rows = sqlx::query_as!(
            SkillView,
            r#"SELECT id, name, category, level, sort_order
               FROM skills WHERE category = $1 ORDER BY sort_order"#,
            category
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }
}
