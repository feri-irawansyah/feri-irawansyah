use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// Cached OAuth2 access token for the service account, shared across
/// requests within this process. Resets on restart — acceptable, a fresh
/// token is only a couple hundred ms away.
static TOKEN_CACHE: Mutex<Option<(String, Instant)>> = Mutex::new(None);

#[derive(Serialize)]
struct Claims {
    iss: String,
    scope: String,
    aud: String,
    exp: u64,
    iat: u64,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: u64,
}

/// Signs a JWT with the service account's private key and exchanges it for a
/// short-lived OAuth2 access token (the "JWT bearer" flow) — Google's
/// server-to-server auth path, no user/browser involved.
pub async fn get_access_token() -> Result<String, String> {
    if let Some((token, expiry)) = TOKEN_CACHE.lock().unwrap().as_ref()
        && Instant::now() < *expiry
    {
        return Ok(token.clone());
    }

    let email = std::env::var("GA_SERVICE_ACCOUNT_EMAIL")
        .map_err(|_| "GA_SERVICE_ACCOUNT_EMAIL belum di-set di .env".to_string())?;
    let private_key_pem = std::env::var("GA_SERVICE_ACCOUNT_PRIVATE_KEY")
        .map_err(|_| "GA_SERVICE_ACCOUNT_PRIVATE_KEY belum di-set di .env".to_string())?
        .replace("\\n", "\n");

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs();
    let claims = Claims {
        iss: email,
        scope: "https://www.googleapis.com/auth/analytics.readonly".to_string(),
        aud: "https://oauth2.googleapis.com/token".to_string(),
        exp: now + 3600,
        iat: now,
    };

    let key = EncodingKey::from_rsa_pem(private_key_pem.as_bytes())
        .map_err(|e| format!("GA_SERVICE_ACCOUNT_PRIVATE_KEY tidak valid: {e}"))?;
    let jwt = encode(&Header::new(Algorithm::RS256), &claims, &key)
        .map_err(|e| format!("Gagal sign JWT: {e}"))?;

    let client = reqwest::Client::new();
    let resp = client
        .post("https://oauth2.googleapis.com/token")
        .form(&[
            ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
            ("assertion", jwt.as_str()),
        ])
        .send()
        .await
        .map_err(|e| format!("Gagal menghubungi Google OAuth2: {e}"))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("Google OAuth2 menolak ({status}): {body}"));
    }

    let token_resp: TokenResponse = resp
        .json()
        .await
        .map_err(|e| format!("Gagal membaca response OAuth2: {e}"))?;

    let expiry = Instant::now() + Duration::from_secs(token_resp.expires_in.saturating_sub(60));
    *TOKEN_CACHE.lock().unwrap() = Some((token_resp.access_token.clone(), expiry));

    Ok(token_resp.access_token)
}
