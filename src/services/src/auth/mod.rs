use std::sync::Arc;

pub mod auth_service;
pub use auth_service::AuthServiceImpl;

pub struct AuthServiceDeps {
    pub auth_repo: Arc<dyn modules::auth::AuthRepository>,
    pub jwt_secret: String,
    pub cache: Arc<dyn connectors::cache::CacheStore>,
    /// 32 raw bytes (AES-256-GCM key) — `server/src/main.rs` decodes and
    /// length-checks `MFA_ENC_KEY` at boot so this is guaranteed valid here.
    pub mfa_enc_key: Vec<u8>,
}
