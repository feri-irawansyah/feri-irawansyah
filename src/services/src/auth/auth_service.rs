use aes_gcm::aead::{Aead, KeyInit, OsRng as AesOsRng};
use aes_gcm::{Aes256Gcm, AeadCore, Key, Nonce};
use anyhow::{Result, anyhow};
use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng as ArgonOsRng},
};
use async_trait::async_trait;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use chrono::{Duration, Utc};
use connectors::cache::CacheStore;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use modules::auth::{
    AuthRepository, AuthService, Claims, LoginOutcome, LoginResult, MfaEnrollmentView, UserView,
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use totp_rs::{Algorithm, Secret, TOTP};
use uuid::Uuid;

use crate::auth::AuthServiceDeps;

const MAX_LOGIN_ATTEMPTS: i64 = 5;
const LOGIN_ATTEMPT_WINDOW_SECS: u64 = 15 * 60;

const MAX_MFA_ATTEMPTS: i64 = 5;
const MFA_ATTEMPT_WINDOW_SECS: u64 = 15 * 60;

// TOTP secrets are stored as issuer-less/account-less — the enrollment QR
// carries issuer + account name at generation time (see `enroll_mfa`), but
// re-deriving a `TOTP` just to check a code only needs the algorithm/step
// parameters and the secret itself.
const TOTP_ISSUER: &str = "feri-irawansyah";

const MFA_CHALLENGE_PURPOSE: &str = "mfa_challenge";
const MFA_CHALLENGE_TTL_SECS: i64 = 5 * 60;

// 8 chars, uppercase alphanumeric, ambiguous glyphs (0/O, 1/I/L) excluded —
// see designs/mfa-design.md §4.3.
const RECOVERY_CODE_CHARSET: &[u8] = b"ABCDEFGHJKMNPQRSTUVWXYZ23456789";
const RECOVERY_CODE_COUNT: usize = 10;

/// Claims for the short-lived token handed back from `login()` when a
/// second factor is still needed. Deliberately *not* a `CacheStore` entry:
/// the cache is explicitly allowed to fail open everywhere else in this
/// codebase (see `connectors::cache`'s degrade-on-boot-failure design), but
/// a challenge token that can't be resolved would mean a correct
/// password+TOTP pair still can't log in — for a single-admin site, that's
/// a full lockout, not a degraded feature. Self-contained + signed makes it
/// as available as the JWT signing key already is, with no extra
/// dependency in the critical path.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct MfaChallengeClaims {
    sub: i32,
    purpose: String,
    exp: usize,
}

pub struct AuthServiceImpl {
    repo: Arc<dyn AuthRepository>,
    jwt_secret: String,
    cache: Arc<dyn CacheStore>,
    /// 32-byte AES-256-GCM key, validated at startup (see `AuthServiceDeps`).
    mfa_enc_key: Vec<u8>,
}

impl AuthServiceImpl {
    pub fn new(deps: AuthServiceDeps) -> Self {
        Self {
            repo: deps.auth_repo,
            jwt_secret: deps.jwt_secret,
            cache: deps.cache,
            mfa_enc_key: deps.mfa_enc_key,
        }
    }

    /// Per-IP failed-login tracker, backed by `CacheStore` (Valkey in
    /// production) instead of in-process memory — was a `HashMap<String,
    /// Vec<Instant>>` behind a `Mutex`, which reset on every restart and,
    /// more importantly, only ever saw the requests that landed on *that*
    /// process. Whichever instance handles the next login attempt now sees
    /// the same counter no matter how many instances are running behind a
    /// load balancer.
    ///
    /// Fixed window rather than the old sliding window (exact per-attempt
    /// timestamps) — `incr_with_ttl` only sets the expiry once, on the first
    /// failure, so the lock always clears exactly `LOGIN_ATTEMPT_WINDOW_SECS`
    /// after the *first* failure in a burst rather than sliding forward on
    /// every subsequent one. Standard, simpler, and plenty for brute-force
    /// protection on a single admin login.
    fn login_attempt_key(ip: &str) -> String {
        format!("login-attempts:{ip}").to_string()
    }

    async fn is_locked(&self, ip: &str) -> bool {
        self.cache
            .get_raw(&Self::login_attempt_key(ip))
            .await
            .and_then(|v| v.parse::<i64>().ok())
            .is_some_and(|count| count >= MAX_LOGIN_ATTEMPTS)
    }

    async fn record_login_failure(&self, ip: &str) {
        self.cache
            .incr_with_ttl(&Self::login_attempt_key(ip), LOGIN_ATTEMPT_WINDOW_SECS)
            .await;
    }

    async fn clear_login_failures(&self, ip: &str) {
        let _ = self.cache.delete_key(&Self::login_attempt_key(ip)).await;
    }

    /// Same shape as the login-attempt lock, keyed per-user instead of
    /// per-IP so a TOTP brute-force attempt doesn't share (or exhaust) the
    /// password guess budget, and vice versa.
    fn mfa_attempt_key(user_id: i32) -> String {
        format!("mfa-attempts:{user_id}").to_string()
    }

    async fn is_mfa_locked(&self, user_id: i32) -> bool {
        self.cache
            .get_raw(&Self::mfa_attempt_key(user_id))
            .await
            .and_then(|v| v.parse::<i64>().ok())
            .is_some_and(|count| count >= MAX_MFA_ATTEMPTS)
    }

    async fn record_mfa_failure(&self, user_id: i32) {
        self.cache
            .incr_with_ttl(&Self::mfa_attempt_key(user_id), MFA_ATTEMPT_WINDOW_SECS)
            .await;
    }

    async fn clear_mfa_failures(&self, user_id: i32) {
        let _ = self.cache.delete_key(&Self::mfa_attempt_key(user_id)).await;
    }

    fn create_access_token(
        &self,
        user_id: i32,
        email: &str,
        client_category: i32,
    ) -> Result<String> {
        let exp = (Utc::now() + Duration::minutes(15)).timestamp() as usize;
        let claims = Claims {
            sub: user_id,
            email: email.to_string(),
            client_category,
            exp,
        };
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )?;
        Ok(token)
    }

    /// Shared tail of `login()` (no MFA / MFA already satisfied) and
    /// `verify_mfa()` (MFA just satisfied) — mints the access token and a
    /// fresh, persisted refresh session.
    async fn issue_session(&self, user: &UserView, ip: &str) -> Result<LoginResult> {
        let access_token = self.create_access_token(user.id, &user.email, user.client_category)?;
        let refresh_token = Uuid::new_v4().to_string();
        let expired_at = Utc::now() + Duration::days(7);

        self.repo
            .create_session(user.id, &refresh_token, ip, expired_at)
            .await?;

        Ok(LoginResult {
            access_token,
            refresh_token,
        })
    }

    fn create_mfa_challenge_token(&self, user_id: i32) -> Result<String> {
        let exp = (Utc::now() + Duration::seconds(MFA_CHALLENGE_TTL_SECS)).timestamp() as usize;
        let claims = MfaChallengeClaims {
            sub: user_id,
            purpose: MFA_CHALLENGE_PURPOSE.to_string(),
            exp,
        };
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )?;
        Ok(token)
    }

    fn decode_mfa_challenge_token(&self, token: &str) -> Result<i32> {
        let data = decode::<MfaChallengeClaims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )?;
        if data.claims.purpose != MFA_CHALLENGE_PURPOSE {
            return Err(anyhow!("invalid challenge token"));
        }
        Ok(data.claims.sub)
    }

    fn cipher(&self) -> Aes256Gcm {
        let key = Key::<Aes256Gcm>::from_slice(&self.mfa_enc_key);
        Aes256Gcm::new(key)
    }

    /// AES-256-GCM encrypt, `nonce || ciphertext` base64-encoded together
    /// so there's a single opaque string to persist in `mfa_secret`.
    fn encrypt_secret(&self, plaintext: &[u8]) -> Result<String> {
        let nonce = Aes256Gcm::generate_nonce(&mut AesOsRng);
        let ciphertext = self
            .cipher()
            .encrypt(&nonce, plaintext)
            .map_err(|e| anyhow!("mfa secret encryption failed: {e}"))?;

        let mut combined = nonce.to_vec();
        combined.extend_from_slice(&ciphertext);
        Ok(BASE64.encode(combined))
    }

    fn decrypt_secret(&self, encoded: &str) -> Result<Vec<u8>> {
        let data = BASE64
            .decode(encoded)
            .map_err(|e| anyhow!("mfa secret decode failed: {e}"))?;
        if data.len() < 12 {
            return Err(anyhow!("mfa secret ciphertext malformed"));
        }
        let (nonce_bytes, ciphertext) = data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        self.cipher()
            .decrypt(nonce, ciphertext)
            .map_err(|e| anyhow!("mfa secret decryption failed: {e}"))
    }

    fn verify_totp_code(&self, encrypted_secret: &str, code: &str) -> Result<bool> {
        let secret_bytes = self.decrypt_secret(encrypted_secret)?;
        let totp = TOTP::new(
            Algorithm::SHA1,
            6,
            1,
            30,
            secret_bytes,
            Some(TOTP_ISSUER.to_string()),
            String::new(),
        )
        .map_err(|e| anyhow!("invalid totp secret: {e}"))?;
        Ok(totp.check_current(code.trim()).unwrap_or(false))
    }

    fn hash_recovery_code(code: &str) -> Result<String> {
        let salt = SaltString::generate(&mut ArgonOsRng);
        let hash = Argon2::default()
            .hash_password(code.as_bytes(), &salt)
            .map_err(|e| anyhow!("recovery code hash failed: {e}"))?
            .to_string();
        Ok(hash)
    }

    /// Recovery codes are compared against Argon2 hashes with the same
    /// `PasswordVerifier` used for the login password, so a leaked
    /// `mfa_recovery_codes` column doesn't hand out usable codes directly.
    fn find_matching_recovery_code(user: &UserView, code: &str) -> Option<String> {
        let normalized = code.trim().to_uppercase();
        let hashes = user.mfa_recovery_codes.as_ref()?;
        hashes
            .iter()
            .find(|hash| {
                PasswordHash::new(hash)
                    .ok()
                    .is_some_and(|parsed| {
                        Argon2::default()
                            .verify_password(normalized.as_bytes(), &parsed)
                            .is_ok()
                    })
            })
            .cloned()
    }

    fn generate_recovery_codes() -> Vec<String> {
        let mut rng = rand::thread_rng();
        (0..RECOVERY_CODE_COUNT)
            .map(|_| {
                let raw: String = (0..8)
                    .map(|_| {
                        RECOVERY_CODE_CHARSET[rng.gen_range(0..RECOVERY_CODE_CHARSET.len())]
                            as char
                    })
                    .collect();
                format!("{}-{}", &raw[..4], &raw[4..])
            })
            .collect()
    }
}

#[async_trait]
impl AuthService for AuthServiceImpl {
    async fn login(&self, email: &str, password: &str, ip: &str) -> Result<LoginOutcome> {
        if self.is_locked(ip).await {
            return Err(anyhow!(
                "Terlalu banyak percobaan login gagal. Coba lagi dalam beberapa menit."
            ));
        }

        let user = match self.repo.find_user_by_email(email).await? {
            Some(u) => u,
            None => {
                self.record_login_failure(ip).await;
                return Err(anyhow!("Invalid credentials"));
            }
        };

        let parsed_hash =
            PasswordHash::new(&user.password).map_err(|e| anyhow!("Hash error: {e}"))?;

        if Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_err()
        {
            self.record_login_failure(ip).await;
            return Err(anyhow!("Invalid credentials"));
        }

        self.clear_login_failures(ip).await;

        if user.mfa_enabled == Some(true) {
            let challenge_token = self.create_mfa_challenge_token(user.id)?;
            return Ok(LoginOutcome::MfaRequired { challenge_token });
        }

        self.issue_session(&user, ip)
            .await
            .map(LoginOutcome::Authenticated)
    }

    async fn verify_mfa(&self, challenge_token: &str, code: &str, ip: &str) -> Result<LoginResult> {
        let user_id = self
            .decode_mfa_challenge_token(challenge_token)
            .map_err(|_| {
                anyhow!("Sesi verifikasi tidak valid atau kedaluwarsa, silakan login ulang.")
            })?;

        if self.is_mfa_locked(user_id).await {
            return Err(anyhow!(
                "Terlalu banyak percobaan kode salah. Coba lagi dalam beberapa menit."
            ));
        }

        let user = self
            .repo
            .find_user_by_id(user_id)
            .await?
            .ok_or_else(|| anyhow!("User not found"))?;

        if user.mfa_enabled != Some(true) {
            return Err(anyhow!("MFA tidak aktif untuk akun ini"));
        }
        let Some(encrypted_secret) = user.mfa_secret.as_deref() else {
            return Err(anyhow!("MFA belum lengkap dikonfigurasi"));
        };

        if self.verify_totp_code(encrypted_secret, code).unwrap_or(false) {
            self.clear_mfa_failures(user_id).await;
            return self.issue_session(&user, ip).await;
        }

        if let Some(matched_hash) = Self::find_matching_recovery_code(&user, code) {
            self.repo
                .consume_recovery_code(user_id, &matched_hash)
                .await?;
            self.clear_mfa_failures(user_id).await;
            return self.issue_session(&user, ip).await;
        }

        self.record_mfa_failure(user_id).await;
        Err(anyhow!("Kode MFA tidak valid"))
    }

    async fn refresh(&self, refresh_token: &str, ip: &str) -> Result<LoginResult> {
        let session = self
            .repo
            .find_session_by_token(refresh_token)
            .await?
            .ok_or_else(|| anyhow!("Session not found"))?;

        if session.expired_at < Utc::now() {
            self.repo.delete_session_by_token(refresh_token).await?;
            return Err(anyhow!("Session expired"));
        }

        let user = self
            .repo
            .find_user_by_id(session.user_id)
            .await?
            .ok_or_else(|| anyhow!("User not found"))?;

        // Rotate: invalidate old token, issue a fresh one
        self.repo.delete_session_by_token(refresh_token).await?;
        self.issue_session(&user, ip).await
    }

    async fn logout(&self, refresh_token: &str) -> Result<()> {
        self.repo.delete_session_by_token(refresh_token).await
    }

    fn validate_access_token(&self, token: &str) -> Result<Claims> {
        let data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )?;
        Ok(data.claims)
    }

    async fn enroll_mfa(&self, user_id: i32) -> Result<MfaEnrollmentView> {
        let user = self
            .repo
            .find_user_by_id(user_id)
            .await?
            .ok_or_else(|| anyhow!("User not found"))?;

        let secret = Secret::generate_secret();
        let secret_bytes = secret
            .to_bytes()
            .map_err(|e| anyhow!("secret generation failed: {e}"))?;

        let totp = TOTP::new(
            Algorithm::SHA1,
            6,
            1,
            30,
            secret_bytes.clone(),
            Some(TOTP_ISSUER.to_string()),
            user.email.clone(),
        )
        .map_err(|e| anyhow!("failed to build TOTP: {e}"))?;

        let encrypted = self.encrypt_secret(&secret_bytes)?;
        self.repo.save_mfa_secret(user_id, &encrypted).await?;

        let qr_png_base64 = totp
            .get_qr_base64()
            .map_err(|e| anyhow!("qr generation failed: {e}"))?;

        Ok(MfaEnrollmentView {
            secret_base32: totp.get_secret_base32(),
            qr_data_uri: format!("data:image/png;base64,{qr_png_base64}"),
        })
    }

    async fn confirm_mfa(&self, user_id: i32, code: &str) -> Result<Vec<String>> {
        let user = self
            .repo
            .find_user_by_id(user_id)
            .await?
            .ok_or_else(|| anyhow!("User not found"))?;
        let encrypted_secret = user
            .mfa_secret
            .as_deref()
            .ok_or_else(|| anyhow!("Belum ada enrollment MFA yang pending"))?;

        if !self.verify_totp_code(encrypted_secret, code)? {
            return Err(anyhow!("Kode MFA tidak valid"));
        }

        let recovery_codes = Self::generate_recovery_codes();
        let hashes = recovery_codes
            .iter()
            .map(|rc| Self::hash_recovery_code(rc))
            .collect::<Result<Vec<_>>>()?;

        self.repo.enable_mfa(user_id, hashes).await?;
        Ok(recovery_codes)
    }

    async fn disable_mfa(&self, user_id: i32, code: &str) -> Result<()> {
        let user = self
            .repo
            .find_user_by_id(user_id)
            .await?
            .ok_or_else(|| anyhow!("User not found"))?;

        if user.mfa_enabled != Some(true) {
            return Err(anyhow!("MFA belum aktif untuk akun ini"));
        }
        let Some(encrypted_secret) = user.mfa_secret.as_deref() else {
            return Err(anyhow!("MFA belum lengkap dikonfigurasi"));
        };

        let verified_by_totp = self.verify_totp_code(encrypted_secret, code).unwrap_or(false);
        let verified_by_recovery =
            !verified_by_totp && Self::find_matching_recovery_code(&user, code).is_some();

        if !verified_by_totp && !verified_by_recovery {
            return Err(anyhow!("Kode verifikasi tidak valid"));
        }

        self.repo.disable_mfa(user_id).await
    }
}

#[cfg(test)]
#[path = "_auth_test.rs"]
mod tests;
