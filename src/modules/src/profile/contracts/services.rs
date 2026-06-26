use anyhow::Result;
use async_trait::async_trait;
use crate::profile::profile_view::ProfileView;

#[async_trait]
pub trait ProfileService: Send + Sync {
    async fn get(&self) -> Result<Option<ProfileView>>;
}
