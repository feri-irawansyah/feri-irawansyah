use anyhow::Result;
use async_trait::async_trait;
use connectors::cache::{self, CacheConn};
use modules::notes::{NoteCommand, NoteService, NoteView};
use std::sync::Arc;

use crate::notes::NoteServiceDeps;

const DOMAIN: &str = "notes";

pub struct NoteServiceImpl {
    repo: Arc<dyn modules::notes::NoteRepository>,
    cache: CacheConn,
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
    async fn list(&self) -> Result<Vec<NoteView>> {
        let version = cache::cache_version(&self.cache, DOMAIN).await;
        let key = cache::versioned_key(DOMAIN, version, &["all"]);
        if let Some(cached) = cache::get_cached::<Vec<NoteView>>(&self.cache, &key).await {
            return Ok(cached);
        }
        let rows = self.repo.find_all().await?;
        cache::set_cached(&self.cache, &key, &rows).await;
        Ok(rows)
    }

    async fn list_page(&self, page: i64, per_page: i64) -> Result<(Vec<NoteView>, i64)> {
        let version = cache::cache_version(&self.cache, DOMAIN).await;
        let key = cache::versioned_key(
            DOMAIN,
            version,
            &["page", &page.to_string(), &per_page.to_string()],
        );
        if let Some(cached) = cache::get_cached::<(Vec<NoteView>, i64)>(&self.cache, &key).await {
            return Ok(cached);
        }
        let result = self.repo.find_paginated(page, per_page).await?;
        cache::set_cached(&self.cache, &key, &result).await;
        Ok(result)
    }

    async fn recent(&self, limit: i64) -> Result<Vec<NoteView>> {
        let version = cache::cache_version(&self.cache, DOMAIN).await;
        let key = cache::versioned_key(DOMAIN, version, &["recent", &limit.to_string()]);
        if let Some(cached) = cache::get_cached::<Vec<NoteView>>(&self.cache, &key).await {
            return Ok(cached);
        }
        let rows = self.repo.find_recent(limit).await?;
        cache::set_cached(&self.cache, &key, &rows).await;
        Ok(rows)
    }

    async fn get_by_slug(&self, slug: &str) -> Result<Option<NoteView>> {
        let version = cache::cache_version(&self.cache, DOMAIN).await;
        let key = cache::versioned_key(DOMAIN, version, &["slug", slug]);
        if let Some(cached) = cache::get_cached::<Option<NoteView>>(&self.cache, &key).await {
            return Ok(cached);
        }
        let row = self.repo.find_by_slug(slug).await?;
        cache::set_cached(&self.cache, &key, &row).await;
        Ok(row)
    }

    async fn by_category(&self, category: &str) -> Result<Vec<NoteView>> {
        let version = cache::cache_version(&self.cache, DOMAIN).await;
        let key = cache::versioned_key(DOMAIN, version, &["cat", category]);
        if let Some(cached) = cache::get_cached::<Vec<NoteView>>(&self.cache, &key).await {
            return Ok(cached);
        }
        let rows = self.repo.find_by_category(category).await?;
        cache::set_cached(&self.cache, &key, &rows).await;
        Ok(rows)
    }

    async fn list_admin(&self) -> Result<Vec<NoteView>> {
        self.repo.find_all_admin().await
    }

    async fn list_admin_page(&self, page: i64, per_page: i64) -> Result<(Vec<NoteView>, i64)> {
        self.repo.find_all_admin_page(page, per_page).await
    }

    async fn create(&self, input: NoteCommand) -> Result<NoteView> {
        let result = self.repo.create(input).await?;
        cache::bump_cache_version(&self.cache, DOMAIN).await;
        Ok(result)
    }

    async fn update(&self, id: i32, input: NoteCommand) -> Result<Option<NoteView>> {
        let result = self.repo.update(id, input).await?;
        cache::bump_cache_version(&self.cache, DOMAIN).await;
        Ok(result)
    }

    async fn toggle_enabled(&self, id: i32, enabled: bool) -> Result<bool> {
        let result = self.repo.toggle_enabled(id, enabled).await?;
        cache::bump_cache_version(&self.cache, DOMAIN).await;
        Ok(result)
    }

    async fn delete(&self, id: i32) -> Result<bool> {
        let result = self.repo.delete(id).await?;
        cache::bump_cache_version(&self.cache, DOMAIN).await;
        Ok(result)
    }
}

#[cfg(test)]
#[path = "_notes_test.rs"]
mod tests;
