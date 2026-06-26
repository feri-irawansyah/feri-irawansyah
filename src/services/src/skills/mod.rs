use std::sync::Arc;

pub mod skill_service;
pub use skill_service::SkillServiceImpl;

pub struct SkillServiceDeps {
    pub skill_repo: Arc<dyn modules::skills::SkillRepository>,
}
