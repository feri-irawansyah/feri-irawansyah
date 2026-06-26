use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillView {
    pub id: i32,
    pub name: String,
    pub category: String,
    pub level: i16,
    pub sort_order: i32,
}
