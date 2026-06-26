use anyhow::Result;
use async_trait::async_trait;
use modules::notes::{NoteRepository, NoteView};
use sqlx::PgPool;

pub struct NoteRepositoryImpl {
    pool: PgPool,
}

impl NoteRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl NoteRepository for NoteRepositoryImpl {
    async fn find_all(&self) -> Result<Vec<NoteView>> {
        let rows = sqlx::query_as!(
            NoteView,
            r#"SELECT notes_id, category, title, slug, content, description,
                      hashtag as "hashtag: Vec<String>", published, ip_address, last_update
               FROM notes WHERE published = TRUE ORDER BY last_update DESC"#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    async fn find_by_slug(&self, slug: &str) -> Result<Option<NoteView>> {
        let row = sqlx::query_as!(
            NoteView,
            r#"SELECT notes_id, category, title, slug, content, description,
                      hashtag as "hashtag: Vec<String>", published, ip_address, last_update
               FROM notes WHERE slug = $1"#,
            slug
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    async fn find_by_category(&self, category: &str) -> Result<Vec<NoteView>> {
        let rows = sqlx::query_as!(
            NoteView,
            r#"SELECT notes_id, category, title, slug, content, description,
                      hashtag as "hashtag: Vec<String>", published, ip_address, last_update
               FROM notes WHERE category = $1 AND published = TRUE ORDER BY last_update DESC"#,
            category
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }
}
