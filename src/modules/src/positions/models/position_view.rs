use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Raw row from the `positions` table only — no join. `experience_id` is the
/// foreign key the service layer resolves against `ExperienceRepository`.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PositionRow {
    pub id: i32,
    pub experience_id: i32,
    pub title: String,
    pub address: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub description: Vec<String>,
    pub job_position: String,
    pub job_type: String,
    pub sort_order: i32,
}

/// Position enriched with its parent experience (company) info. Assembled by
/// `PositionServiceImpl`, not queried directly.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionView {
    pub id: i32,
    pub experience_id: i32,
    pub title: String,
    pub company: String,
    pub url_docs: String,
    pub image_src: String,
    pub address: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub description: Vec<String>,
    pub job_position: String,
    pub job_type: String,
    pub sort_order: i32,
}
