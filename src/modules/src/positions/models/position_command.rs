use chrono::NaiveDate;
use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct PositionCommand {
    #[garde(skip)]
    pub experience_id: i32,
    #[garde(length(min = 1, max = 150))]
    pub title: String,
    #[garde(skip)]
    pub start_date: NaiveDate,
    #[garde(skip)]
    pub end_date: Option<NaiveDate>,
    #[garde(skip)]
    pub description: Vec<String>,
    #[garde(length(max = 300))]
    pub address: String,
    #[garde(length(min = 1, max = 50))]
    pub job_position: String,
    #[garde(length(min = 1, max = 50))]
    pub job_type: String,
    #[garde(skip)]
    pub sort_order: i32,
}
