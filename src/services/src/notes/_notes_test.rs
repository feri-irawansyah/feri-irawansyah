use super::*;
use chrono::Utc;
use modules::notes::{NoteCommand, NoteRepository, NoteView};
use std::sync::Mutex;

// ── Mock repository ───────────────────────────────────────────────────────────
struct MockNoteRepo {
    notes: Mutex<Vec<NoteView>>,
    next_id: Mutex<i32>,
}

impl MockNoteRepo {
    fn new(notes: Vec<NoteView>) -> Self {
        let next = (notes.len() as i32) + 1;
        Self {
            notes: Mutex::new(notes),
            next_id: Mutex::new(next),
        }
    }

    fn empty() -> Self {
        Self {
            notes: Mutex::new(vec![]),
            next_id: Mutex::new(1),
        }
    }
}

#[async_trait::async_trait]
impl NoteRepository for MockNoteRepo {
    async fn find_all_async(&self) -> anyhow::Result<Vec<NoteView>> {
        Ok(self
            .notes
            .lock()
            .unwrap()
            .iter()
            .filter(|n| n.enabled)
            .cloned()
            .collect())
    }

    async fn find_recent_async(&self, limit: i64) -> anyhow::Result<Vec<NoteView>> {
        Ok(self
            .notes
            .lock()
            .unwrap()
            .iter()
            .filter(|n| n.enabled)
            .rev()
            .take(limit as usize)
            .cloned()
            .collect())
    }

    async fn find_by_slug_async(&self, slug: &str) -> anyhow::Result<Option<NoteView>> {
        Ok(self
            .notes
            .lock()
            .unwrap()
            .iter()
            .find(|n| n.slug == slug)
            .cloned())
    }

    async fn find_by_category_async(&self, category: &str) -> anyhow::Result<Vec<NoteView>> {
        Ok(self
            .notes
            .lock()
            .unwrap()
            .iter()
            .filter(|n| n.category == category && n.enabled)
            .cloned()
            .collect())
    }

    async fn find_paginated_async(
        &self,
        page: i64,
        per_page: i64,
    ) -> anyhow::Result<(Vec<NoteView>, i64)> {
        let all: Vec<_> = self
            .notes
            .lock()
            .unwrap()
            .iter()
            .filter(|n| n.enabled)
            .cloned()
            .collect();
        let total = all.len() as i64;
        let offset = ((page - 1) * per_page) as usize;
        let items: Vec<_> = all
            .into_iter()
            .skip(offset)
            .take(per_page as usize)
            .collect();
        Ok((items, total))
    }

    async fn search_async(
        &self,
        query: &str,
        page: i64,
        per_page: i64,
    ) -> anyhow::Result<(Vec<NoteView>, i64)> {
        // Good-enough stand-in for `tsv @@ websearch_to_tsquery`: a
        // case-insensitive substring match over the same columns the real
        // generated column indexes.
        let q = query.to_lowercase();
        let all: Vec<_> = self
            .notes
            .lock()
            .unwrap()
            .iter()
            .filter(|n| {
                n.enabled
                    && (n.title.to_lowercase().contains(&q)
                        || n.description.to_lowercase().contains(&q)
                        || n.category.to_lowercase().contains(&q)
                        || n.hashtag.iter().any(|h| h.to_lowercase().contains(&q)))
            })
            .cloned()
            .collect();
        let total = all.len() as i64;
        let offset = ((page - 1).max(0) * per_page) as usize;
        let items = all
            .into_iter()
            .skip(offset)
            .take(per_page as usize)
            .collect();
        Ok((items, total))
    }

    async fn find_all_admin_async(&self) -> anyhow::Result<Vec<NoteView>> {
        Ok(self.notes.lock().unwrap().clone())
    }

    async fn find_all_admin_page_async(
        &self,
        page: i64,
        per_page: i64,
    ) -> anyhow::Result<(Vec<NoteView>, i64)> {
        let all = self.notes.lock().unwrap().clone();
        let total = all.len() as i64;
        let offset = ((page - 1) * per_page) as usize;
        let items: Vec<_> = all
            .into_iter()
            .skip(offset)
            .take(per_page as usize)
            .collect();
        Ok((items, total))
    }

    async fn create_async(&self, input: NoteCommand) -> anyhow::Result<NoteView> {
        let mut id_lock = self.next_id.lock().unwrap();
        let id = *id_lock;
        *id_lock += 1;
        let note = NoteView {
            notes_id: id,
            category: input.category,
            title: input.title,
            slug: input.slug,
            content: input.content,
            description: input.description,
            hashtag: input.hashtag,
            enabled: input.enabled,
            ip_address: "127.0.0.1".to_string(),
            last_update: Utc::now(),
        };
        self.notes.lock().unwrap().push(note.clone());
        Ok(note)
    }

    async fn update_async(&self, id: i32, input: NoteCommand) -> anyhow::Result<Option<NoteView>> {
        let mut notes = self.notes.lock().unwrap();
        if let Some(note) = notes.iter_mut().find(|n| n.notes_id == id) {
            note.title = input.title;
            note.slug = input.slug;
            note.content = input.content;
            note.description = input.description;
            note.category = input.category;
            note.hashtag = input.hashtag;
            note.enabled = input.enabled;
            Ok(Some(note.clone()))
        } else {
            Ok(None)
        }
    }

    async fn delete_async(&self, id: i32) -> anyhow::Result<bool> {
        let mut notes = self.notes.lock().unwrap();
        let before = notes.len();
        notes.retain(|n| n.notes_id != id);
        Ok(notes.len() < before)
    }

    async fn toggle_enabled_async(&self, id: i32, enabled: bool) -> anyhow::Result<bool> {
        let mut notes = self.notes.lock().unwrap();
        if let Some(note) = notes.iter_mut().find(|n| n.notes_id == id) {
            note.enabled = enabled;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────
fn make_svc(repo: MockNoteRepo) -> NoteServiceImpl {
    NoteServiceImpl::new(crate::notes::NoteServiceDeps {
        note_repo: Arc::new(repo),
        cache: Arc::new(connectors::cache::MockCacheClient::new())
            as Arc<dyn connectors::cache::CacheStore>,
    })
}

fn make_note(id: i32, slug: &str, category: &str, enabled: bool) -> NoteView {
    NoteView {
        notes_id: id,
        category: category.to_string(),
        title: format!("Note {id}"),
        slug: slug.to_string(),
        content: "content".to_string(),
        description: "desc".to_string(),
        hashtag: vec![],
        enabled,
        ip_address: "127.0.0.1".to_string(),
        last_update: Utc::now(),
    }
}

fn make_cmd(slug: &str, category: &str) -> NoteCommand {
    NoteCommand {
        category: category.to_string(),
        title: format!("Title {slug}"),
        slug: slug.to_string(),
        content: "content".to_string(),
        description: "desc".to_string(),
        hashtag: vec!["rust".to_string()],
        enabled: true,
    }
}

// ── list ─────────────────────────────────────────────────────────────────────
#[tokio::test]
async fn list_returns_only_enabled() {
    let notes = vec![
        make_note(1, "enabled-note", "rust", true),
        make_note(2, "disabled-note", "rust", false),
    ];
    let svc = make_svc(MockNoteRepo::new(notes));
    let result = svc.find_all_async().await.unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].slug, "enabled-note");
}

// ── recent ────────────────────────────────────────────────────────────────────
#[tokio::test]
async fn recent_respects_limit() {
    let notes = (1..=10)
        .map(|i| make_note(i, &format!("note-{i}"), "rust", true))
        .collect();
    let svc = make_svc(MockNoteRepo::new(notes));
    let result = svc.recent_async(3).await.unwrap();
    assert_eq!(result.len(), 3);
}

#[tokio::test]
async fn recent_excludes_disabled() {
    let notes = vec![
        make_note(1, "pub", "rust", true),
        make_note(2, "priv", "rust", false),
    ];
    let svc = make_svc(MockNoteRepo::new(notes));
    let result = svc.recent_async(10).await.unwrap();
    assert!(result.iter().all(|n| n.enabled));
}

// ── get_by_slug ───────────────────────────────────────────────────────────────
#[tokio::test]
async fn get_by_slug_found() {
    let notes = vec![make_note(1, "my-post", "rust", true)];
    let svc = make_svc(MockNoteRepo::new(notes));
    let result = svc.find_by_slug_async("my-post").await.unwrap();
    assert!(result.is_some());
    assert_eq!(result.unwrap().slug, "my-post");
}

#[tokio::test]
async fn get_by_slug_not_found() {
    let svc = make_svc(MockNoteRepo::empty());
    assert!(svc.find_by_slug_async("nope").await.unwrap().is_none());
}

// ── by_category ───────────────────────────────────────────────────────────────
#[tokio::test]
async fn by_category_filters_correctly() {
    let notes = vec![
        make_note(1, "rust-post", "rust", true),
        make_note(2, "go-post", "go", true),
        make_note(3, "another-rust", "rust", true),
    ];
    let svc = make_svc(MockNoteRepo::new(notes));
    let result = svc.find_by_category_async("rust").await.unwrap();
    assert_eq!(result.len(), 2);
    assert!(result.iter().all(|n| n.category == "rust"));
}

// ── search ───────────────────────────────────────────────────────────────────
#[tokio::test]
async fn search_matches_title() {
    let notes = vec![
        make_note(1, "rust-async", "rust", true),
        make_note(2, "go-channels", "go", true),
    ];
    let svc = make_svc(MockNoteRepo::new(notes));
    let (result, total) = svc.search_async("Note 1", 1, 10).await.unwrap();
    assert_eq!(total, 1);
    assert_eq!(result[0].slug, "rust-async");
}

#[tokio::test]
async fn search_blank_query_returns_empty_without_hitting_repo() {
    let svc = make_svc(MockNoteRepo::empty());
    let (result, total) = svc.search_async("   ", 1, 10).await.unwrap();
    assert!(result.is_empty());
    assert_eq!(total, 0);
}

#[tokio::test]
async fn search_excludes_disabled_notes() {
    let notes = vec![
        make_note(1, "visible", "rust", true),
        make_note(2, "hidden", "rust", false),
    ];
    let svc = make_svc(MockNoteRepo::new(notes));
    let (result, _) = svc.search_async("Note", 1, 10).await.unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].slug, "visible");
}

#[tokio::test]
async fn search_paginates_results() {
    let notes = (1..=5)
        .map(|i| make_note(i, &format!("note-{i}"), "rust", true))
        .collect();
    let svc = make_svc(MockNoteRepo::new(notes));
    let (page1, total) = svc.search_async("Note", 1, 2).await.unwrap();
    assert_eq!(total, 5);
    assert_eq!(page1.len(), 2);
}

// ── create ────────────────────────────────────────────────────────────────────
#[tokio::test]
async fn create_stores_and_returns_note() {
    let svc = make_svc(MockNoteRepo::empty());
    let cmd = make_cmd("new-post", "rust");
    let result = svc.create_async(cmd).await.unwrap();
    assert_eq!(result.slug, "new-post");
    assert_eq!(result.category, "rust");
    assert_eq!(result.hashtag, vec!["rust"]);
    assert!(result.enabled);
}

// ── update ────────────────────────────────────────────────────────────────────
#[tokio::test]
async fn update_modifies_existing_note() {
    let notes = vec![make_note(1, "old-slug", "rust", true)];
    let svc = make_svc(MockNoteRepo::new(notes));
    let mut cmd = make_cmd("new-slug", "leptos");
    cmd.title = "Updated Title".to_string();
    let result = svc.update_async(1, cmd).await.unwrap();
    assert!(result.is_some());
    let updated = result.unwrap();
    assert_eq!(updated.slug, "new-slug");
    assert_eq!(updated.category, "leptos");
}

#[tokio::test]
async fn update_returns_none_for_missing_id() {
    let svc = make_svc(MockNoteRepo::empty());
    let result = svc
        .update_async(99, make_cmd("slug", "rust"))
        .await
        .unwrap();
    assert!(result.is_none());
}

// ── delete ────────────────────────────────────────────────────────────────────
#[tokio::test]
async fn delete_existing_returns_true() {
    let notes = vec![make_note(1, "to-delete", "rust", true)];
    let svc = make_svc(MockNoteRepo::new(notes));
    assert!(svc.delete_async(1).await.unwrap());
    assert!(svc.find_all_async().await.unwrap().is_empty());
}

#[tokio::test]
async fn delete_missing_returns_false() {
    let svc = make_svc(MockNoteRepo::empty());
    assert!(!svc.delete_async(99).await.unwrap());
}

// ── list_page ─────────────────────────────────────────────────────────────────
#[tokio::test]
async fn list_page_paginates_correctly() {
    let notes = (1..=10)
        .map(|i| make_note(i, &format!("note-{i}"), "rust", true))
        .collect();
    let svc = make_svc(MockNoteRepo::new(notes));
    let (page1, total) = svc.find_page_async(1, 4).await.unwrap();
    assert_eq!(total, 10);
    assert_eq!(page1.len(), 4);
    let (page3, _) = svc.find_page_async(3, 4).await.unwrap();
    assert_eq!(page3.len(), 2);
}
