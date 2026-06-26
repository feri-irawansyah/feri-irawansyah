use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteView {
    pub notes_id: i32,
    pub category: String,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub description: String,
    pub hashtag: Vec<String>,
    pub published: bool,
    pub ip_address: String,
    pub last_update: DateTime<Utc>,
}
