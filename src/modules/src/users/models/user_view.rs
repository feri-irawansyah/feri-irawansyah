use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// client_category: 0 = visitor, 1 = admin
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserView {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub fullname: String,
    pub mobile_phone: String,
    pub picture: Option<String>,
    pub google_id: String,
    pub client_category: i32,
    pub activate_code: String,
    pub otp_generated_link: String,
    pub otp_generated_link_date: DateTime<Utc>,
    pub count_resend_activation: i32,
    pub activate_time: Option<DateTime<Utc>>,
    pub disable_login: bool,
    pub reset_password_key: String,
    pub reset_password_flag: bool,
    pub reset_password_date: Option<DateTime<Utc>>,
    pub last_login: Option<DateTime<Utc>>,
    pub register_date: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// `Some(true)` once MFA enrollment is confirmed; `None` means never
    /// enrolled. See `modules::auth::UserView` for the enrollment fields
    /// themselves — this domain only needs the on/off state for display.
    pub mfa_enabled: Option<bool>,
}
