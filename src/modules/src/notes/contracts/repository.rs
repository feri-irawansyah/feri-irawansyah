use anyhow::Result;
use async_trait::async_trait;
use crate::notes::note_view::NoteView;

#[async_trait]
pub trait NoteRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<NoteView>>;
    async fn find_recent(&self, limit: i64) -> Result<Vec<NoteView>>;
    async fn find_by_slug(&self, slug: &str) -> Result<Option<NoteView>>;
    async fn find_by_category(&self, category: &str) -> Result<Vec<NoteView>>;
}
