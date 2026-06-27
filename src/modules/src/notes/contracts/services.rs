use anyhow::Result;
use async_trait::async_trait;
use crate::notes::note_view::NoteView;

#[async_trait]
pub trait NoteService: Send + Sync {
    async fn list(&self) -> Result<Vec<NoteView>>;
    async fn list_page(&self, page: i64, per_page: i64) -> Result<(Vec<NoteView>, i64)>;
    async fn recent(&self, limit: i64) -> Result<Vec<NoteView>>;
    async fn get_by_slug(&self, slug: &str) -> Result<Option<NoteView>>;
}
