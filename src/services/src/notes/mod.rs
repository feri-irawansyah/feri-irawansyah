use std::sync::Arc;

pub mod note_service;
pub use note_service::NoteServiceImpl;

pub struct NoteServiceDeps {
    pub note_repo: Arc<dyn modules::notes::NoteRepository>,
}
