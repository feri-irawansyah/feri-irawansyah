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
        let rows = sqlx::query_as!(
            CertView,
            r#"SELECT id, title, issuer, issued_at, expired_at, credential_id,
                      credential_url, image_src, sort_order, created_at, updated_at
               FROM certifications ORDER BY sort_order, issued_at DESC"#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<CertView>> {
        let row = sqlx::query_as!(
            CertView,
            r#"SELECT id, title, issuer, issued_at, expired_at, credential_id,
                      credential_url, image_src, sort_order, created_at, updated_at
               FROM certifications WHERE id = $1"#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }
}
