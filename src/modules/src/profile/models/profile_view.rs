use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileView {
    pub id: i32,
    pub name: String,
    pub title: String,
    pub bio: String,
    pub avatar_url: Option<String>,
    pub github_url: Option<String>,
    pub linkedin_url: Option<String>,
    pub website_url: Option<String>,
}
