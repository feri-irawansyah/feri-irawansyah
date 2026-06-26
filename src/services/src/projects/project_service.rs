use anyhow::Result;
use async_trait::async_trait;
use modules::projects::{ProjectService, ProjectView};
use std::sync::Arc;

use crate::projects::ProjectServiceDeps;

pub struct ProjectServiceImpl {
    repo: Arc<dyn modules::projects::ProjectRepository>,
}

impl ProjectServiceImpl {
    pub fn new(deps: ProjectServiceDeps) -> Self {
        Self { repo: deps.project_repo }
    }
}

#[async_trait]
impl ProjectService for ProjectServiceImpl {
    async fn list(&self) -> Result<Vec<ProjectView>> {
        self.repo.find_all().await
    }

    async fn featured(&self) -> Result<Vec<ProjectView>> {
        self.repo.find_featured().await
    }

    async fn get_by_slug(&self, slug: &str) -> Result<Option<ProjectView>> {
        self.repo.find_by_slug(slug).await
    }
}
