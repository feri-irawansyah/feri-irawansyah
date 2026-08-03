use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SkillCommand {
    #[garde(length(min = 1, max = 150))]
    pub title: String,
    #[garde(length(max = 1000))]
    pub description: String,
    #[garde(length(max = 500))]
    pub url_docs: String,
    #[garde(length(max = 500))]
    pub image_src: String,
    #[garde(range(min = 0, max = 100))]
    pub progress: i32,
    #[garde(range(min = 0, max = 5))]
    pub star: i32,
}
