use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CertView {
    pub id: i32,
    pub title: String,
    pub url_docs: String,
    pub image_src: String,
    pub description: String,
    pub tech: Vec<i32>,
    pub start_date: NaiveDate,
    pub last_update: DateTime<Utc>,
}
