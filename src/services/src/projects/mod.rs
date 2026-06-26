use std::sync::Arc;

pub mod project_service;
pub use project_service::ProjectServiceImpl;

pub struct ProjectServiceDeps {
    pub project_repo: Arc<dyn modules::projects::ProjectRepository>,
}
