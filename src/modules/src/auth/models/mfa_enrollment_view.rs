use serde::{Deserialize, Serialize};

/// Returned by `AuthService::enroll_mfa` — everything the client needs to
/// scan into an authenticator app plus the manual-entry fallback.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MfaEnrollmentView {
    pub secret_base32: String,
    pub qr_data_uri: String,
}
