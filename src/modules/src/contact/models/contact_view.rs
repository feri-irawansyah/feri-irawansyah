use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactView {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub subject: String,
    pub body: String,
    pub read: bool,
    pub created_at: DateTime<Utc>,
}
