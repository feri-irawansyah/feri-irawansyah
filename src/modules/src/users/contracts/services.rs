use anyhow::Result;
use async_trait::async_trait;

use crate::users::UserView;

#[async_trait]
pub trait UserService: Send + Sync {
    async fn create_async(
        &self,
        email: &str,
        password: &str,
        fullname: &str,
        client_category: i32,
    ) -> Result<UserView>;
    async fn find_by_id_async(&self, id: i32) -> Result<Option<UserView>>;
    async fn find_by_email_async(&self, email: &str) -> Result<Option<UserView>>;
    async fn find_all_async(&self, limit: i64, offset: i64) -> Result<Vec<UserView>>;
}
