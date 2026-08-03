use anyhow::Result;
use async_trait::async_trait;
use modules::laboratory::{LaboratoryCommand, LaboratoryRepository, LaboratoryView};
use sqlx::PgPool;

const SELECT_ALL: &str = "SELECT lab_id, category, title, slug, content, description,
    COALESCE(hashtag, '{}') as hashtag,
    enabled,
    COALESCE(ip_address, '') as ip_address,
    last_update
    FROM labolatory";

pub struct LaboratoryRepositoryImpl {
    pool: PgPool,
}

impl LaboratoryRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LaboratoryRepository for LaboratoryRepositoryImpl {
    async fn find_by_slug(&self, slug: &str) -> Result<Option<LaboratoryView>> {
        let row = sqlx::query_as::<_, LaboratoryView>(&format!("{SELECT_ALL} WHERE slug = $1"))
            .bind(slug)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row)
    }

    async fn find_by_category_page(
        &self,
        category: &str,
        page: i64,
        per_page: i64,
    ) -> Result<(Vec<LaboratoryView>, i64)> {
        let total: i64 = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM labolatory WHERE enabled = TRUE AND category = $1",
        )
        .bind(category)
        .fetch_one(&self.pool)
        .await?;
        let offset = (page - 1).max(0) * per_page;
        let rows = sqlx::query_as::<_, LaboratoryView>(&format!(
            "{SELECT_ALL} WHERE enabled = TRUE AND category = $1
             ORDER BY last_update DESC LIMIT $2 OFFSET $3"
        ))
        .bind(category)
        .bind(per_page)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;
        Ok((rows, total))
    }

    async fn find_all_admin(&self) -> Result<Vec<LaboratoryView>> {
        let rows = sqlx::query_as::<_, LaboratoryView>(&format!(
            "{SELECT_ALL} ORDER BY last_update DESC"
        ))
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    async fn find_all_admin_page(&self, page: i64, per_page: i64) -> Result<(Vec<LaboratoryView>, i64)> {
        let total: i64 = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM labolatory")
            .fetch_one(&self.pool)
            .await?;
        let offset = (page - 1).max(0) * per_page;
        let rows = sqlx::query_as::<_, LaboratoryView>(&format!(
            "{SELECT_ALL} ORDER BY last_update DESC LIMIT $1 OFFSET $2"
        ))
        .bind(per_page)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;
        Ok((rows, total))
    }

    async fn create(&self, input: LaboratoryCommand) -> Result<LaboratoryView> {
        let row = sqlx::query_as::<_, LaboratoryView>(
            "INSERT INTO labolatory (category, title, slug, content, description, hashtag, enabled)
             VALUES ($1, $2, $3, $4, $5, $6, $7)
             RETURNING lab_id, category, title, slug, content, description,
                       COALESCE(hashtag, '{}') as hashtag,
                       enabled,
                       COALESCE(ip_address, '') as ip_address,
                       last_update",
        )
        .bind(input.category)
        .bind(input.title)
        .bind(input.slug)
        .bind(input.content)
        .bind(input.description)
        .bind(input.hashtag)
        .bind(input.enabled)
        .fetch_one(&self.pool)
        .await?;
        Ok(row)
    }

    async fn update(&self, id: i32, input: LaboratoryCommand) -> Result<Option<LaboratoryView>> {
        let row = sqlx::query_as::<_, LaboratoryView>(
            "UPDATE labolatory
             SET category = $2, title = $3, slug = $4, content = $5, description = $6,
                 hashtag = $7, enabled = $8, last_update = NOW()
             WHERE lab_id = $1
             RETURNING lab_id, category, title, slug, content, description,
                       COALESCE(hashtag, '{}') as hashtag,
                       enabled,
                       COALESCE(ip_address, '') as ip_address,
                       last_update",
        )
        .bind(id)
        .bind(input.category)
        .bind(input.title)
        .bind(input.slug)
        .bind(input.content)
        .bind(input.description)
        .bind(input.hashtag)
        .bind(input.enabled)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    async fn delete(&self, id: i32) -> Result<bool> {
        let result = sqlx::query("DELETE FROM labolatory WHERE lab_id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}
