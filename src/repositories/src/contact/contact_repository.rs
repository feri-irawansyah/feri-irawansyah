use anyhow::Result;
use async_trait::async_trait;
use modules::contact::{ContactRepository, ContactReq, ContactView};
use sqlx::PgPool;

pub struct ContactRepositoryImpl {
    pool: PgPool,
}

impl ContactRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ContactRepository for ContactRepositoryImpl {
    async fn save(&self, req: &ContactReq) -> Result<ContactView> {
        let row = sqlx::query_as!(
            ContactView,
            r#"INSERT INTO contact_messages (name, email, subject, body)
               VALUES ($1, $2, $3, $4)
               RETURNING id, name, email, subject, body, read, created_at"#,
            req.name, req.email, req.subject, req.body
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(row)
    }

    async fn find_all(&self) -> Result<Vec<ContactView>> {
        let rows = sqlx::query_as!(
            ContactView,
            r#"SELECT id, name, email, subject, body, read, created_at
               FROM contact_messages ORDER BY created_at DESC"#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }
}
