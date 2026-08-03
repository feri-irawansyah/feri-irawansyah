use chrono::NaiveDate;
use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ExperienceCommand {
    #[garde(length(min = 1, max = 150))]
    pub title: String,
    #[garde(length(min = 1, max = 150))]
    pub company: String,
    #[garde(length(max = 500))]
    pub url_docs: String,
    #[garde(length(max = 500))]
    pub image_src: String,
    #[garde(skip)]
    pub start_date: NaiveDate,
    #[garde(skip)]
    pub end_date: Option<NaiveDate>,
}
