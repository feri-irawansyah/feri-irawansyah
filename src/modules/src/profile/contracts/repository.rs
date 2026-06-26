use anyhow::Result;
use async_trait::async_trait;
use crate::profile::profile_view::ProfileView;

#[async_trait]
pub trait ProfileRepository: Send + Sync {
    async fn find(&self) -> Result<Option<ProfileView>>;
    async fn upsert(&self, view: &ProfileView) -> Result<ProfileView>;
}
