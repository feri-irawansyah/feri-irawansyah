use anyhow::Result;
use async_trait::async_trait;

use crate::users::UserView;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(
        &self,
        email: &str,
        password_hash: &str,
        fullname: &str,
        client_category: i32,
    ) -> Result<UserView>;
    async fn find_by_id(&self, id: i32) -> Result<Option<UserView>>;
    async fn find_by_email(&self, email: &str) -> Result<Option<UserView>>;
    async fn list(&self, limit: i64, offset: i64) -> Result<Vec<UserView>>;
}
