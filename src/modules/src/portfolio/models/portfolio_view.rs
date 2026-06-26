use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioView {
    pub portfolio_id: i32,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub url_docs: String,
    pub image_src: String,
    pub tech: Vec<i32>,
    pub featured: bool,
    pub sort_order: i32,
    pub last_update: DateTime<Utc>,
}
