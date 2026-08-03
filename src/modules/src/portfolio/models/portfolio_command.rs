use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct PortfolioCommand {
    #[garde(length(min = 1, max = 150))]
    pub title: String,
    #[garde(length(min = 1, max = 150))]
    pub slug: String,
    #[garde(length(max = 2000))]
    pub description: String,
    #[garde(length(max = 500))]
    pub url_docs: String,
    #[garde(length(max = 500))]
    pub image_src: String,
    #[garde(skip)]
    pub tech: Vec<i32>,
    #[garde(skip)]
    pub pined: bool,
    #[garde(skip)]
    pub sort_order: i32,
    #[garde(length(max = 20000))]
    pub details: String,
}
