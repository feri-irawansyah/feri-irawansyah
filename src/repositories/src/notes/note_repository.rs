use anyhow::Result;
use async_trait::async_trait;
use modules::notes::{NoteCommand, NoteRepository, NoteView};
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
    async fn find_all_async(&self) -> Result<Vec<NoteView>> {
        let rows = sqlx::query_as::<_, NoteView>(
            "SELECT notes_id, category, title, slug, content, description,
                    COALESCE(hashtag, '{}') as hashtag,
                    enabled,
                    COALESCE(ip_address, '') as ip_address,
                    last_update
             FROM notes WHERE enabled = TRUE ORDER BY last_update DESC",
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    async fn find_recent_async(&self, limit: i64) -> Result<Vec<NoteView>> {
        let rows = sqlx::query_as::<_, NoteView>(
            "SELECT notes_id, category, title, slug, content, description,
                    COALESCE(hashtag, '{}') as hashtag,
                    enabled,
                    COALESCE(ip_address, '') as ip_address,
                    last_update
             FROM notes WHERE enabled = TRUE ORDER BY last_update DESC LIMIT $1",
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    async fn find_by_slug_async(&self, slug: &str) -> Result<Option<NoteView>> {
        let row = sqlx::query_as::<_, NoteView>(
            "SELECT notes_id, category, title, slug, content, description,
                    COALESCE(hashtag, '{}') as hashtag,
                    enabled,
                    COALESCE(ip_address, '') as ip_address,
                    last_update
             FROM notes WHERE slug = $1",
        )
        .bind(slug)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    async fn find_by_category_async(&self, category: &str) -> Result<Vec<NoteView>> {
        let rows = sqlx::query_as::<_, NoteView>(
            "SELECT notes_id, category, title, slug, content, description,
                    COALESCE(hashtag, '{}') as hashtag,
                    enabled,
                    COALESCE(ip_address, '') as ip_address,
                    last_update
             FROM notes WHERE category = $1 ORDER BY last_update DESC",
        )
        .bind(category)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    async fn find_paginated_async(&self, page: i64, per_page: i64) -> Result<(Vec<NoteView>, i64)> {
        let total: i64 =
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM notes WHERE enabled = TRUE")
                .fetch_one(&self.pool)
                .await?;
        let offset = (page - 1).max(0) * per_page;
        let rows = sqlx::query_as::<_, NoteView>(
            "SELECT notes_id, category, title, slug, content, description,
                    COALESCE(hashtag, '{}') as hashtag,
                    enabled,
                    COALESCE(ip_address, '') as ip_address,
                    last_update
             FROM notes WHERE enabled = TRUE ORDER BY last_update DESC LIMIT $1 OFFSET $2",
        )
        .bind(per_page)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;
        Ok((rows, total))
    }

    async fn search_async(&self, query: &str, page: i64, per_page: i64) -> Result<(Vec<NoteView>, i64)> {
        let offset = (page - 1).max(0) * per_page;
        let total: i64 = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM notes
             WHERE enabled = TRUE AND tsv @@ websearch_to_tsquery('simple', $1)",
        )
        .bind(query)
        .fetch_one(&self.pool)
        .await?;
        let rows = sqlx::query_as::<_, NoteView>(
            "SELECT notes_id, category, title, slug, content, description,
                    COALESCE(hashtag, '{}') as hashtag,
                    enabled,
                    COALESCE(ip_address, '') as ip_address,
                    last_update
             FROM notes
             WHERE enabled = TRUE AND tsv @@ websearch_to_tsquery('simple', $1)
             ORDER BY ts_rank(tsv, websearch_to_tsquery('simple', $1)) DESC, last_update DESC
             LIMIT $2 OFFSET $3",
        )
        .bind(query)
        .bind(per_page)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;
        Ok((rows, total))
    }

    async fn find_all_admin_async(&self) -> Result<Vec<NoteView>> {
        let rows = sqlx::query_as::<_, NoteView>(
            "SELECT notes_id, category, title, slug, content, description,
                    COALESCE(hashtag, '{}') as hashtag,
                    enabled,
                    COALESCE(ip_address, '') as ip_address,
                    last_update
             FROM notes ORDER BY last_update DESC",
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    async fn find_all_admin_page_async(&self, page: i64, per_page: i64) -> Result<(Vec<NoteView>, i64)> {
        let total: i64 = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM notes")
            .fetch_one(&self.pool)
            .await?;
        let offset = (page - 1).max(0) * per_page;
        let rows = sqlx::query_as::<_, NoteView>(
            "SELECT notes_id, category, title, slug, content, description,
                    COALESCE(hashtag, '{}') as hashtag,
                    enabled,
                    COALESCE(ip_address, '') as ip_address,
                    last_update
             FROM notes ORDER BY last_update DESC
             LIMIT $1 OFFSET $2",
        )
        .bind(per_page)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;
        Ok((rows, total))
    }

    async fn create_async(&self, input: NoteCommand) -> Result<NoteView> {
        let row = sqlx::query_as::<_, NoteView>(
            "INSERT INTO notes (category, title, slug, content, description, hashtag, enabled)
             VALUES ($1, $2, $3, $4, $5, $6, $7)
             RETURNING notes_id, category, title, slug, content, description,
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

    async fn update_async(&self, id: i32, input: NoteCommand) -> Result<Option<NoteView>> {
        let row = sqlx::query_as::<_, NoteView>(
            "UPDATE notes
             SET category = $2, title = $3, slug = $4, content = $5, description = $6,
                 hashtag = $7, enabled = $8, last_update = NOW()
             WHERE notes_id = $1
             RETURNING notes_id, category, title, slug, content, description,
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

    async fn toggle_enabled_async(&self, id: i32, enabled: bool) -> Result<bool> {
        let result =
            sqlx::query("UPDATE notes SET enabled = $2, last_update = NOW() WHERE notes_id = $1")
                .bind(id)
                .bind(enabled)
                .execute(&self.pool)
                .await?;
        Ok(result.rows_affected() > 0)
    }

    async fn delete_async(&self, id: i32) -> Result<bool> {
        let result = sqlx::query("DELETE FROM notes WHERE notes_id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}
