use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserView {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub fullname: String,
    pub client_category: i32,
    /// AES-256-GCM ciphertext of the TOTP secret, base64-encoded. `None`
    /// means MFA was never enrolled for this account.
    pub mfa_secret: Option<String>,
    /// `Some(true)` once enrollment is confirmed. `None`/`Some(false)` both
    /// read as "not enrolled" — `disable_mfa` resets this to `None` rather
    /// than leaving a half-enrolled `mfa_secret` behind.
    pub mfa_enabled: Option<bool>,
    /// Argon2 hashes of unused one-time recovery codes.
    pub mfa_recovery_codes: Option<Vec<String>>,
}
