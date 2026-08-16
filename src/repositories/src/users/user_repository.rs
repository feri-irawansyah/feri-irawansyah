use anyhow::Result;
use async_trait::async_trait;
use modules::users::{UserRepository, UserView};
use sqlx::PgPool;

#[cfg(test)]
#[path = "_users_test.rs"]
mod tests;

const SELECT_ALL: &str = "SELECT id, email, password, fullname, mobile_phone, picture,
    google_id, client_category, activate_code, otp_generated_link,
    otp_generated_link_date, count_resend_activation, activate_time,
    disable_login, reset_password_key, reset_password_flag,
    reset_password_date, last_login, register_date, updated_at, mfa_enabled
    FROM users";

pub struct UserRepositoryImpl {
    pool: PgPool,
}

impl UserRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create_async(
        &self,
        email: &str,
        password_hash: &str,
        fullname: &str,
        client_category: i32,
    ) -> Result<UserView> {
        let row = sqlx::query_as::<_, UserView>(
            "INSERT INTO users (email, password, fullname, client_category, disable_login)
             VALUES ($1, $2, $3, $4, FALSE)
             RETURNING id, email, password, fullname, mobile_phone, picture,
             google_id, client_category, activate_code, otp_generated_link,
             otp_generated_link_date, count_resend_activation, activate_time,
             disable_login, reset_password_key, reset_password_flag,
             reset_password_date, last_login, register_date, updated_at, mfa_enabled",
        )
        .bind(email)
        .bind(password_hash)
        .bind(fullname)
        .bind(client_category)
        .fetch_one(&self.pool)
        .await?;
        Ok(row)
    }

    async fn find_by_id_async(&self, id: i32) -> Result<Option<UserView>> {
        let row = sqlx::query_as::<_, UserView>(&format!("{SELECT_ALL} WHERE id = $1"))
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row)
    }

    async fn find_by_email_async(&self, email: &str) -> Result<Option<UserView>> {
        let row = sqlx::query_as::<_, UserView>(&format!("{SELECT_ALL} WHERE email = $1"))
            .bind(email)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row)
    }

    async fn find_all_async(&self, limit: i64, offset: i64) -> Result<Vec<UserView>> {
        let rows = sqlx::query_as::<_, UserView>(&format!(
            "{SELECT_ALL} ORDER BY id ASC LIMIT $1 OFFSET $2"
        ))
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }
}
