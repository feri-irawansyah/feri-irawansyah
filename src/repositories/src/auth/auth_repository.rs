use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use modules::auth::{AuthRepository, SessionView, UserView};
use sqlx::PgPool;

#[cfg(test)]
#[path = "_auth_test.rs"]
mod tests;

pub struct AuthRepositoryImpl {
    pool: PgPool,
}

impl AuthRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuthRepository for AuthRepositoryImpl {
    async fn find_user_by_email(&self, email: &str) -> Result<Option<UserView>> {
        let row = sqlx::query_as::<_, UserView>(
            "SELECT id, email, password, fullname, client_category,
                    mfa_secret, mfa_enabled, mfa_recovery_codes
             FROM users
             WHERE email = $1 AND disable_login = FALSE",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    async fn find_user_by_id(&self, id: i32) -> Result<Option<UserView>> {
        let row = sqlx::query_as::<_, UserView>(
            "SELECT id, email, password, fullname, client_category,
                    mfa_secret, mfa_enabled, mfa_recovery_codes
             FROM users WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    async fn create_session(
        &self,
        user_id: i32,
        token: &str,
        ip: &str,
        expired_at: DateTime<Utc>,
    ) -> Result<SessionView> {
        let row = sqlx::query_as::<_, SessionView>(
            "INSERT INTO auth_sessions (user_id, token, ip_address, expired_at)
             VALUES ($1, $2, $3, $4)
             RETURNING id, user_id, token, expired_at",
        )
        .bind(user_id)
        .bind(token)
        .bind(ip)
        .bind(expired_at)
        .fetch_one(&self.pool)
        .await?;
        Ok(row)
    }

    async fn find_session_by_token(&self, token: &str) -> Result<Option<SessionView>> {
        let row = sqlx::query_as::<_, SessionView>(
            "SELECT id, user_id, token, expired_at FROM auth_sessions WHERE token = $1",
        )
        .bind(token)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    async fn delete_session_by_token(&self, token: &str) -> Result<()> {
        sqlx::query("DELETE FROM auth_sessions WHERE token = $1")
            .bind(token)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn save_mfa_secret(&self, user_id: i32, encrypted_secret: &str) -> Result<()> {
        sqlx::query("UPDATE users SET mfa_secret = $2 WHERE id = $1")
            .bind(user_id)
            .bind(encrypted_secret)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn enable_mfa(&self, user_id: i32, recovery_code_hashes: Vec<String>) -> Result<()> {
        sqlx::query(
            "UPDATE users SET mfa_enabled = TRUE, mfa_recovery_codes = $2 WHERE id = $1",
        )
        .bind(user_id)
        .bind(recovery_code_hashes)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn disable_mfa(&self, user_id: i32) -> Result<()> {
        sqlx::query(
            "UPDATE users
             SET mfa_secret = NULL, mfa_enabled = NULL, mfa_recovery_codes = NULL
             WHERE id = $1",
        )
        .bind(user_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn consume_recovery_code(&self, user_id: i32, code_hash: &str) -> Result<bool> {
        // array_remove is a no-op if the hash isn't present, so the WHERE
        // guard is what tells us whether it actually matched anything.
        let result = sqlx::query(
            "UPDATE users
             SET mfa_recovery_codes = array_remove(mfa_recovery_codes, $2)
             WHERE id = $1 AND mfa_recovery_codes @> ARRAY[$2]::text[]",
        )
        .bind(user_id)
        .bind(code_hash)
        .execute(&self.pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }
}
