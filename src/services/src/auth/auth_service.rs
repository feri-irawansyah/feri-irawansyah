use anyhow::{Result, anyhow};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use async_trait::async_trait;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use modules::auth::{AuthRepository, AuthService, Claims, LoginResult};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration as StdDuration, Instant};
use uuid::Uuid;

use crate::auth::AuthServiceDeps;

const MAX_LOGIN_ATTEMPTS: usize = 5;
const LOGIN_ATTEMPT_WINDOW: StdDuration = StdDuration::from_secs(15 * 60);

/// In-memory per-IP failed-login tracker. Resets on process restart — an
/// acceptable tradeoff here (this guards a single personal admin login, not
/// a large multi-instance service), and avoids a DB round-trip on every hit.
#[derive(Default)]
struct LoginAttempts {
    failures: Mutex<HashMap<String, Vec<Instant>>>,
}

impl LoginAttempts {
    fn is_locked(&self, key: &str) -> bool {
        let mut map = self.failures.lock().unwrap();
        match map.get_mut(key) {
            Some(times) => {
                let now = Instant::now();
                times.retain(|t| now.duration_since(*t) < LOGIN_ATTEMPT_WINDOW);
                times.len() >= MAX_LOGIN_ATTEMPTS
            }
            None => false,
        }
    }

    fn record_failure(&self, key: &str) {
        let mut map = self.failures.lock().unwrap();
        let now = Instant::now();
        let entry = map.entry(key.to_string()).or_default();
        entry.retain(|t| now.duration_since(*t) < LOGIN_ATTEMPT_WINDOW);
        entry.push(now);
    }

    fn clear(&self, key: &str) {
        self.failures.lock().unwrap().remove(key);
    }
}

pub struct AuthServiceImpl {
    repo: Arc<dyn AuthRepository>,
    jwt_secret: String,
    login_attempts: LoginAttempts,
}

impl AuthServiceImpl {
    pub fn new(deps: AuthServiceDeps) -> Self {
        Self {
            repo: deps.auth_repo,
            jwt_secret: deps.jwt_secret,
            login_attempts: LoginAttempts::default(),
        }
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
        if self.login_attempts.is_locked(ip) {
            return Err(anyhow!(
                "Terlalu banyak percobaan login gagal. Coba lagi dalam beberapa menit."
            ));
        }

        let user = match self.repo.find_user_by_email(email).await? {
            Some(u) => u,
            None => {
                self.login_attempts.record_failure(ip);
                return Err(anyhow!("Invalid credentials"));
            }
        };

        let parsed_hash =
            PasswordHash::new(&user.password).map_err(|e| anyhow!("Hash error: {e}"))?;

        if Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_err()
        {
            self.login_attempts.record_failure(ip);
            return Err(anyhow!("Invalid credentials"));
        }

        self.login_attempts.clear(ip);

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
