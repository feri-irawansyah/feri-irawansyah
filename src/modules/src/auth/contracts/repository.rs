use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::auth::{SessionView, UserView};

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn find_user_by_email(&self, email: &str) -> Result<Option<UserView>>;
    async fn find_user_by_id(&self, id: i32) -> Result<Option<UserView>>;
    async fn create_session(
        &self,
        user_id: i32,
        token: &str,
        ip: &str,
        expired_at: DateTime<Utc>,
    ) -> Result<SessionView>;
    async fn find_session_by_token(&self, token: &str) -> Result<Option<SessionView>>;
    async fn delete_session_by_token(&self, token: &str) -> Result<()>;
}
