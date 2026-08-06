use anyhow::Result;
use async_trait::async_trait;
use modules::certifications::{CertRepository, CertView};
use sqlx::PgPool;

pub struct CertRepositoryImpl {
    pool: PgPool,
}

impl CertRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CertRepository for CertRepositoryImpl {
    async fn find_all(&self) -> Result<Vec<CertView>> {
        let rows = sqlx::query_as::<_, CertView>(
            "SELECT id, title, url_docs, image_src, description, tech, start_date, last_update
             FROM certifications ORDER BY start_date DESC",
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<CertView>> {
        let row = sqlx::query_as::<_, CertView>(
            "SELECT id, title, url_docs, image_src, description, tech, start_date, last_update
             FROM certifications WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }
}
