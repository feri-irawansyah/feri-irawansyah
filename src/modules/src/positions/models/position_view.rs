use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionView {
    pub id: i32,
    pub company: String,
    pub role: String,
    pub location: String,
    pub employment_type: String,
    pub started_at: NaiveDate,
    pub ended_at: Option<NaiveDate>,
    pub is_current: bool,
    pub description: String,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
