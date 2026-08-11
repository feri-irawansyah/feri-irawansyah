use anyhow::Result;
use async_trait::async_trait;
use connectors::cache::{self, CacheStore};
use modules::notes::{NoteCommand, NoteService, NoteView};
use std::sync::Arc;

use crate::notes::NoteServiceDeps;

const DOMAIN: &str = "notes";

pub struct NoteServiceImpl {
    repo: Arc<dyn modules::notes::NoteRepository>,
    cache: Arc<dyn CacheStore>,
}

impl NoteServiceImpl {
    pub fn new(deps: NoteServiceDeps) -> Self {
        Self {
            repo: deps.note_repo,
            cache: deps.cache,
        }
    }
}

#[async_trait]
impl NoteService for NoteServiceImpl {
    async fn find_all_async(&self) -> Result<Vec<NoteView>> {
        let version = cache::cache_version(self.cache.as_ref(), DOMAIN).await;
        let key = cache::versioned_key(DOMAIN, version, &["all"]);
        if let Some(cached) = cache::get_cached::<Vec<NoteView>>(self.cache.as_ref(), &key).await {
            return Ok(cached);
        }
        let rows = self.repo.find_all_async().await?;
        cache::set_cached(self.cache.as_ref(), &key, &rows).await;
        Ok(rows)
    }

    async fn find_page_async(&self, page: i64, per_page: i64) -> Result<(Vec<NoteView>, i64)> {
        let version = cache::cache_version(self.cache.as_ref(), DOMAIN).await;
        let key = cache::versioned_key(
            DOMAIN,
            version,
            &["page", &page.to_string(), &per_page.to_string()],
        );
        if let Some(cached) =
            cache::get_cached::<(Vec<NoteView>, i64)>(self.cache.as_ref(), &key).await
        {
            return Ok(cached);
        }
        let result = self.repo.find_paginated_async(page, per_page).await?;
        cache::set_cached(self.cache.as_ref(), &key, &result).await;
        Ok(result)
    }

    async fn recent_async(&self, limit: i64) -> Result<Vec<NoteView>> {
        let version = cache::cache_version(self.cache.as_ref(), DOMAIN).await;
        let key = cache::versioned_key(DOMAIN, version, &["recent", &limit.to_string()]);
        if let Some(cached) = cache::get_cached::<Vec<NoteView>>(self.cache.as_ref(), &key).await {
            return Ok(cached);
        }
        let rows = self.repo.find_recent_async(limit).await?;
        cache::set_cached(self.cache.as_ref(), &key, &rows).await;
        Ok(rows)
    }

    async fn find_by_slug_async(&self, slug: &str) -> Result<Option<NoteView>> {
        let version = cache::cache_version(self.cache.as_ref(), DOMAIN).await;
        let key = cache::versioned_key(DOMAIN, version, &["slug", &cache::hash_part(slug)]);
        if let Some(cached) = cache::get_cached::<Option<NoteView>>(self.cache.as_ref(), &key).await
        {
            return Ok(cached);
        }
        let row = self.repo.find_by_slug_async(slug).await?;
        cache::set_cached(self.cache.as_ref(), &key, &row).await;
        Ok(row)
    }

    async fn find_by_category_async(&self, category: &str) -> Result<Vec<NoteView>> {
        let version = cache::cache_version(self.cache.as_ref(), DOMAIN).await;
        let key = cache::versioned_key(DOMAIN, version, &["cat", category]);
        if let Some(cached) = cache::get_cached::<Vec<NoteView>>(self.cache.as_ref(), &key).await {
            return Ok(cached);
        }
        let rows = self.repo.find_by_category_async(category).await?;
        cache::set_cached(self.cache.as_ref(), &key, &rows).await;
        Ok(rows)
    }

    async fn search_async(&self, query: &str, page: i64, per_page: i64) -> Result<(Vec<NoteView>, i64)> {
        let query = query.trim();
        if query.is_empty() {
            return Ok((Vec::new(), 0));
        }

        let version = cache::cache_version(self.cache.as_ref(), DOMAIN).await;
        let key = cache::versioned_key(
            DOMAIN,
            version,
            &[
                "search",
                &cache::hash_part(query),
                &page.to_string(),
                &per_page.to_string(),
            ],
        );
        if let Some(cached) =
            cache::get_cached::<(Vec<NoteView>, i64)>(self.cache.as_ref(), &key).await
        {
            return Ok(cached);
        }
        let result = self.repo.search_async(query, page, per_page).await?;
        cache::set_cached(self.cache.as_ref(), &key, &result).await;
        Ok(result)
    }

    async fn find_all_admin_async(&self) -> Result<Vec<NoteView>> {
        self.repo.find_all_admin_async().await
    }

    async fn find_all_admin_page_async(&self, page: i64, per_page: i64) -> Result<(Vec<NoteView>, i64)> {
        self.repo.find_all_admin_page_async(page, per_page).await
    }

    async fn create_async(&self, input: NoteCommand) -> Result<NoteView> {
        let result = self.repo.create_async(input).await?;
        cache::bump_cache_version(self.cache.as_ref(), DOMAIN).await;
        Ok(result)
    }

    async fn update_async(&self, id: i32, input: NoteCommand) -> Result<Option<NoteView>> {
        let result = self.repo.update_async(id, input).await?;
        cache::bump_cache_version(self.cache.as_ref(), DOMAIN).await;
        Ok(result)
    }

    async fn toggle_enabled_async(&self, id: i32, enabled: bool) -> Result<bool> {
        let result = self.repo.toggle_enabled_async(id, enabled).await?;
        cache::bump_cache_version(self.cache.as_ref(), DOMAIN).await;
        Ok(result)
    }

    async fn delete_async(&self, id: i32) -> Result<bool> {
        let result = self.repo.delete_async(id).await?;
        cache::bump_cache_version(self.cache.as_ref(), DOMAIN).await;
        Ok(result)
    }
}

#[cfg(test)]
#[path = "_notes_test.rs"]
mod tests;
