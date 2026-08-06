use anyhow::{Result, anyhow};
use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use async_trait::async_trait;
use modules::users::{UserRepository, UserService, UserView};
use std::sync::Arc;

use crate::users::UserServiceDeps;

pub struct UserServiceImpl {
    repo: Arc<dyn UserRepository>,
}

impl UserServiceImpl {
    pub fn new(deps: UserServiceDeps) -> Self {
        Self {
            repo: deps.user_repo,
        }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn create(
        &self,
        email: &str,
        password: &str,
        fullname: &str,
        client_category: i32,
    ) -> Result<UserView> {
        let salt = SaltString::generate(&mut OsRng);
        let hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow!("Hash error: {e}"))?
            .to_string();
        self.repo
            .create(email, &hash, fullname, client_category)
            .await
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<UserView>> {
        self.repo.find_by_id(id).await
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<UserView>> {
        self.repo.find_by_email(email).await
    }

    async fn list(&self, limit: i64, offset: i64) -> Result<Vec<UserView>> {
        self.repo.list(limit, offset).await
    }
}

#[cfg(test)]
#[path = "_users_test.rs"]
mod tests;
