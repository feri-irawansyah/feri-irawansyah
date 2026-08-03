use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ExperienceView {
    pub id: i32,
    pub title: String,
    pub company: String,
    pub url_docs: String,
    pub image_src: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub last_update: DateTime<Utc>,
}
