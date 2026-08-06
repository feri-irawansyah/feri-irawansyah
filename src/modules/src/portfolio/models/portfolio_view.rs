use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PortfolioView {
    pub portfolio_id: i32,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub url_docs: String,
    pub image_src: String,
    pub tech: Vec<i32>,
    pub pined: bool,
    pub sort_order: i32,
    pub details: String,
    pub last_update: DateTime<Utc>,
}
