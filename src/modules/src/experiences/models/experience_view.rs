use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperienceView {
    pub id: i32,
    pub company: String,
    pub role: String,
    pub started_at: NaiveDate,
    pub ended_at: Option<NaiveDate>,
    pub description: String,
    pub is_current: bool,
    pub sort_order: i32,
}
