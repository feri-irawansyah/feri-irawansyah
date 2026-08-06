use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SkillView {
    pub skill_id: i32,
    pub title: String,
    pub description: String,
    pub url_docs: String,
    pub image_src: String,
    pub progress: i32,
    pub star: i32,
    pub last_update: DateTime<Utc>,
}
