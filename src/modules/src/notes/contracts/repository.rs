use anyhow::Result;
use async_trait::async_trait;
use crate::notes::note_command::NoteCommand;
use crate::notes::note_view::NoteView;

#[async_trait]
pub trait NoteRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<NoteView>>;
    async fn find_recent(&self, limit: i64) -> Result<Vec<NoteView>>;
    async fn find_by_slug(&self, slug: &str) -> Result<Option<NoteView>>;
    async fn find_by_category(&self, category: &str) -> Result<Vec<NoteView>>;
    async fn find_paginated(&self, page: i64, per_page: i64) -> Result<(Vec<NoteView>, i64)>;
    /// Every note regardless of category/enabled — for the admin listing.
    /// Deliberately separate from `find_all`, which filters for the public feed.
    async fn find_all_admin(&self) -> Result<Vec<NoteView>>;
    async fn find_all_admin_page(&self, page: i64, per_page: i64) -> Result<(Vec<NoteView>, i64)>;
    async fn create(&self, input: NoteCommand) -> Result<NoteView>;
    async fn update(&self, id: i32, input: NoteCommand) -> Result<Option<NoteView>>;
    async fn delete(&self, id: i32) -> Result<bool>;
}
