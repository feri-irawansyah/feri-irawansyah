use anyhow::Result;
use async_trait::async_trait;
use modules::notes::{NoteService, NoteView};
use std::sync::Arc;

use crate::notes::NoteServiceDeps;

pub struct NoteServiceImpl {
    repo: Arc<dyn modules::notes::NoteRepository>,
}

impl NoteServiceImpl {
    pub fn new(deps: NoteServiceDeps) -> Self {
        Self { repo: deps.note_repo }
    }
}

#[async_trait]
impl NoteService for NoteServiceImpl {
    async fn list(&self) -> Result<Vec<NoteView>> {
        self.repo.find_all().await
    }

    async fn list_page(&self, page: i64, per_page: i64) -> Result<(Vec<NoteView>, i64)> {
        self.repo.find_paginated(page, per_page).await
    }

    async fn recent(&self, limit: i64) -> Result<Vec<NoteView>> {
        self.repo.find_recent(limit).await
    }

    async fn get_by_slug(&self, slug: &str) -> Result<Option<NoteView>> {
        self.repo.find_by_slug(slug).await
    }
}
