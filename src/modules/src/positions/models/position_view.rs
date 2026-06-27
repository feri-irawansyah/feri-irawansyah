use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PositionView {
    pub id: i32,
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
