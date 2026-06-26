use std::sync::Arc;

pub mod experience_service;
pub use experience_service::ExperienceServiceImpl;

pub struct ExperienceServiceDeps {
    pub experience_repo: Arc<dyn modules::experiences::ExperienceRepository>,
}
