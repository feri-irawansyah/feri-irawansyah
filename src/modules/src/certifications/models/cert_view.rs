use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertView {
    pub id: i32,
    pub title: String,
    pub issuer: String,
    pub issued_at: NaiveDate,
    pub expired_at: Option<NaiveDate>,
    pub credential_id: String,
    pub credential_url: String,
    pub image_src: String,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
