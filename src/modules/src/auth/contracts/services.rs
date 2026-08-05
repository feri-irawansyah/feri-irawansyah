use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::auth::Claims;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResult {
    pub access_token: String,
    pub refresh_token: String,
}

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn login(&self, email: &str, password: &str, ip: &str) -> Result<LoginResult>;
    async fn refresh(&self, refresh_token: &str, ip: &str) -> Result<LoginResult>;
    async fn logout(&self, refresh_token: &str) -> Result<()>;
    fn validate_access_token(&self, token: &str) -> Result<Claims>;
}
