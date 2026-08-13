use anyhow::{Result, anyhow};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use async_trait::async_trait;
use chrono::{Duration, Utc};
use connectors::cache::CacheStore;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use modules::auth::{AuthRepository, AuthService, Claims, LoginResult};
use std::sync::Arc;
use uuid::Uuid;

use crate::auth::AuthServiceDeps;

const MAX_LOGIN_ATTEMPTS: i64 = 5;
const LOGIN_ATTEMPT_WINDOW_SECS: u64 = 15 * 60;

pub struct AuthServiceImpl {
    repo: Arc<dyn AuthRepository>,
    jwt_secret: String,
    cache: Arc<dyn CacheStore>,
}

impl AuthServiceImpl {
    pub fn new(deps: AuthServiceDeps) -> Self {
        Self {
            repo: deps.auth_repo,
            jwt_secret: deps.jwt_secret,
            cache: deps.cache,
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
}

#[async_trait]
impl AuthService for AuthServiceImpl {
    async fn login(&self, email: &str, password: &str, ip: &str) -> Result<LoginResult> {
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
        let new_refresh = Uuid::new_v4().to_string();
        let expired_at = Utc::now() + Duration::days(7);
        self.repo
            .create_session(user.id, &new_refresh, ip, expired_at)
            .await?;

        let access_token = self.create_access_token(user.id, &user.email, user.client_category)?;
        Ok(LoginResult {
            access_token,
            refresh_token: new_refresh,
        })
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
}

#[cfg(test)]
#[path = "_auth_test.rs"]
mod tests;
