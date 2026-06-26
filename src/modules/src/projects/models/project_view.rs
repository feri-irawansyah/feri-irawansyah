use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectView {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub description: String,
    pub tech_stack: Vec<String>,
    pub repo_url: Option<String>,
    pub demo_url: Option<String>,
    pub image_url: Option<String>,
    pub featured: bool,
    pub sort_order: i32,
}
