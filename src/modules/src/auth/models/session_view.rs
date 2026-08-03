use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SessionView {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
    pub expired_at: DateTime<Utc>,
}
