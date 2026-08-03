use anyhow::Result;
use async_trait::async_trait;
use modules::skills::{SkillCommand, SkillRepository, SkillView};
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
             FROM skills ORDER BY last_update",
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    async fn find_page(&self, page: i64, per_page: i64) -> Result<(Vec<SkillView>, i64)> {
        let total: i64 = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM skills")
            .fetch_one(&self.pool)
            .await?;
        let offset = (page - 1).max(0) * per_page;
        let rows = sqlx::query_as::<_, SkillView>(
            "SELECT skill_id, title, description, url_docs, image_src, progress, star, last_update
             FROM skills ORDER BY last_update DESC
             LIMIT $1 OFFSET $2",
        )
        .bind(per_page)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;
        Ok((rows, total))
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

    async fn create(&self, input: SkillCommand) -> Result<SkillView> {
        let row = sqlx::query_as::<_, SkillView>(
            "INSERT INTO skills (title, description, url_docs, image_src, progress, star)
             VALUES ($1, $2, $3, $4, $5, $6)
             RETURNING skill_id, title, description, url_docs, image_src, progress, star, last_update",
        )
        .bind(input.title)
        .bind(input.description)
        .bind(input.url_docs)
        .bind(input.image_src)
        .bind(input.progress)
        .bind(input.star)
        .fetch_one(&self.pool)
        .await?;
        Ok(row)
    }

    async fn update(&self, skill_id: i32, input: SkillCommand) -> Result<Option<SkillView>> {
        let row = sqlx::query_as::<_, SkillView>(
            "UPDATE skills
             SET title = $2, description = $3, url_docs = $4, image_src = $5,
                 progress = $6, star = $7, last_update = NOW()
             WHERE skill_id = $1
             RETURNING skill_id, title, description, url_docs, image_src, progress, star, last_update",
        )
        .bind(skill_id)
        .bind(input.title)
        .bind(input.description)
        .bind(input.url_docs)
        .bind(input.image_src)
        .bind(input.progress)
        .bind(input.star)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    async fn delete(&self, skill_id: i32) -> Result<bool> {
        let result = sqlx::query("DELETE FROM skills WHERE skill_id = $1")
            .bind(skill_id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}
