use anyhow::Result;
use async_trait::async_trait;
use crate::notes::note_command::NoteCommand;
use crate::notes::note_view::NoteView;

#[async_trait]
pub trait NoteService: Send + Sync {
    async fn find_all_async(&self) -> Result<Vec<NoteView>>;
    async fn find_page_async(&self, page: i64, per_page: i64) -> Result<(Vec<NoteView>, i64)>;
    async fn recent_async(&self, limit: i64) -> Result<Vec<NoteView>>;
    async fn find_by_slug_async(&self, slug: &str) -> Result<Option<NoteView>>;
    async fn find_by_category_async(&self, category: &str) -> Result<Vec<NoteView>>;
    async fn search_async(&self, query: &str, page: i64, per_page: i64) -> Result<(Vec<NoteView>, i64)>;
    async fn find_all_admin_async(&self) -> Result<Vec<NoteView>>;
    async fn find_all_admin_page_async(&self, page: i64, per_page: i64) -> Result<(Vec<NoteView>, i64)>;
    async fn create_async(&self, input: NoteCommand) -> Result<NoteView>;
    async fn update_async(&self, id: i32, input: NoteCommand) -> Result<Option<NoteView>>;
    async fn toggle_enabled_async(&self, id: i32, enabled: bool) -> Result<bool>;
    async fn delete_async(&self, id: i32) -> Result<bool>;
}
