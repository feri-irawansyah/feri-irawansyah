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
        let rows = sqlx::query_as::<_, SkillView>(
            "SELECT skill_id, title, description, url_docs, image_src, progress, star, last_update
             FROM skills ORDER BY star DESC, progress DESC",
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    async fn find_by_id(&self, skill_id: i32) -> Result<Option<SkillView>> {
        let row = sqlx::query_as::<_, SkillView>(
            "SELECT skill_id, title, description, url_docs, image_src, progress, star, last_update
             FROM skills WHERE skill_id = $1",
        )
        .bind(skill_id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }
}
