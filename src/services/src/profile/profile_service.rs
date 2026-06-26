use anyhow::Result;
use async_trait::async_trait;
use modules::profile::{ProfileService, ProfileView};
use std::sync::Arc;

use crate::profile::ProfileServiceDeps;

pub struct ProfileServiceImpl {
    repo: Arc<dyn modules::profile::ProfileRepository>,
}

impl ProfileServiceImpl {
    pub fn new(deps: ProfileServiceDeps) -> Self {
        Self { repo: deps.profile_repo }
    }
}

#[async_trait]
impl ProfileService for ProfileServiceImpl {
    async fn get(&self) -> Result<Option<ProfileView>> {
        self.repo.find().await
    }
}
