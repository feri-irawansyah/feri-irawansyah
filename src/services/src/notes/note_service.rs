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

    async fn get_by_slug(&self, slug: &str) -> Result<Option<NoteView>> {
        self.repo.find_by_slug(slug).await
    }
}
