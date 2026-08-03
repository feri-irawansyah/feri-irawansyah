use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct NoteCommand {
    #[garde(length(min = 1, max = 50))]
    pub category: String,
    #[garde(length(min = 1, max = 200))]
    pub title: String,
    #[garde(length(min = 1, max = 200))]
    pub slug: String,
    #[garde(length(max = 500))]
    pub content: String,
    #[garde(length(max = 500))]
    pub description: String,
    #[garde(skip)]
    pub hashtag: Vec<String>,
    #[garde(skip)]
    pub enabled: bool,
}
