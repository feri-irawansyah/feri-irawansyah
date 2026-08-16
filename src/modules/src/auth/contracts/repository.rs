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

    /// Persists the encrypted secret for a pending (unconfirmed) enrollment.
    /// Does not touch `mfa_enabled`.
    async fn save_mfa_secret(&self, user_id: i32, encrypted_secret: &str) -> Result<()>;
    /// Flips `mfa_enabled` on and stores the recovery-code hashes —
    /// called once, after the first TOTP code is confirmed.
    async fn enable_mfa(&self, user_id: i32, recovery_code_hashes: Vec<String>) -> Result<()>;
    /// Clears `mfa_secret`/`mfa_enabled`/`mfa_recovery_codes` back to their
    /// unenrolled state.
    async fn disable_mfa(&self, user_id: i32) -> Result<()>;
    /// Removes `code_hash` from the account's recovery codes if present.
    /// Returns whether it was found (and thus consumed).
    async fn consume_recovery_code(&self, user_id: i32, code_hash: &str) -> Result<bool>;
}
