use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::auth::{Claims, MfaEnrollmentView};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResult {
    pub access_token: String,
    pub refresh_token: String,
}

/// `login` can't hand back a session in one step once MFA is enrolled — it
/// needs a second factor first. `MfaRequired` carries a short-lived
/// challenge token (not a session) that `verify_mfa` exchanges for the real
/// thing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoginOutcome {
    Authenticated(LoginResult),
    MfaRequired { challenge_token: String },
}

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn login(&self, email: &str, password: &str, ip: &str) -> Result<LoginOutcome>;
    async fn verify_mfa(&self, challenge_token: &str, code: &str, ip: &str) -> Result<LoginResult>;
    async fn refresh(&self, refresh_token: &str, ip: &str) -> Result<LoginResult>;
    async fn logout(&self, refresh_token: &str) -> Result<()>;
    fn validate_access_token(&self, token: &str) -> Result<Claims>;

    /// Start enrollment: generates a new secret and stores it encrypted,
    /// but `mfa_enabled` stays unset until `confirm_mfa` proves the user
    /// actually captured it.
    async fn enroll_mfa(&self, user_id: i32) -> Result<MfaEnrollmentView>;
    /// Verifies the first code against the pending secret, turns MFA on,
    /// and returns the plaintext recovery codes — the only time they're
    /// ever visible.
    async fn confirm_mfa(&self, user_id: i32, code: &str) -> Result<Vec<String>>;
    /// Requires a valid current TOTP or recovery code before turning MFA
    /// back off, so a hijacked session alone can't disable it.
    async fn disable_mfa(&self, user_id: i32, code: &str) -> Result<()>;
}
