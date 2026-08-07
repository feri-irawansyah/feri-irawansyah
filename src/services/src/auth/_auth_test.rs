use super::*;
use chrono::Utc;
use connectors::cache::MockCacheClient;
use modules::auth::{AuthRepository, SessionView, UserView};
use std::sync::Mutex;

// ── Mock repository ───────────────────────────────────────────────────────────

struct MockAuthRepo {
    users: Mutex<Vec<UserView>>,
    sessions: Mutex<Vec<SessionView>>,
    next_id: Mutex<i32>,
}

impl MockAuthRepo {
    fn new(users: Vec<UserView>) -> Self {
        Self {
            users: Mutex::new(users),
            sessions: Mutex::new(vec![]),
            next_id: Mutex::new(1),
        }
    }

    fn with_session(self, s: SessionView) -> Self {
        self.sessions.lock().unwrap().push(s);
        self
    }
}

#[async_trait::async_trait]
impl AuthRepository for MockAuthRepo {
    async fn find_user_by_email(&self, email: &str) -> anyhow::Result<Option<UserView>> {
        Ok(self
            .users
            .lock()
            .unwrap()
            .iter()
            .find(|u| u.email == email)
            .cloned())
    }

    async fn find_user_by_id(&self, id: i32) -> anyhow::Result<Option<UserView>> {
        Ok(self
            .users
            .lock()
            .unwrap()
            .iter()
            .find(|u| u.id == id)
            .cloned())
    }

    async fn create_session(
        &self,
        user_id: i32,
        token: &str,
        _ip: &str,
        expired_at: chrono::DateTime<Utc>,
    ) -> anyhow::Result<SessionView> {
        let mut id_lock = self.next_id.lock().unwrap();
        let id = *id_lock;
        *id_lock += 1;
        let s = SessionView {
            id,
            user_id,
            token: token.to_string(),
            expired_at,
        };
        self.sessions.lock().unwrap().push(s.clone());
        Ok(s)
    }

    async fn find_session_by_token(&self, token: &str) -> anyhow::Result<Option<SessionView>> {
        Ok(self
            .sessions
            .lock()
            .unwrap()
            .iter()
            .find(|s| s.token == token)
            .cloned())
    }

    async fn delete_session_by_token(&self, token: &str) -> anyhow::Result<()> {
        self.sessions.lock().unwrap().retain(|s| s.token != token);
        Ok(())
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn hash_password(password: &str) -> String {
    use argon2::PasswordHasher;
    use argon2::password_hash::{SaltString, rand_core::OsRng};
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

fn test_user() -> UserView {
    UserView {
        id: 1,
        email: "admin@test.com".to_string(),
        password: hash_password("correct_password"),
        fullname: "Admin Test".to_string(),
        client_category: 1,
    }
}

fn make_svc(repo: MockAuthRepo) -> AuthServiceImpl {
    AuthServiceImpl::new(crate::auth::AuthServiceDeps {
        auth_repo: Arc::new(repo),
        jwt_secret: "test_secret_key_32chars_padding!!".to_string(),
        cache: Arc::new(MockCacheClient::new()),
    })
}

fn future_session(token: &str) -> SessionView {
    SessionView {
        id: 1,
        user_id: 1,
        token: token.to_string(),
        expired_at: Utc::now() + Duration::days(7),
    }
}

// ── login rate limiter (cache-backed) ───────────────────────────────────────

#[tokio::test]
async fn rate_limiter_not_locked_initially() {
    let svc = make_svc(MockAuthRepo::new(vec![]));
    assert!(!svc.is_locked("1.2.3.4").await);
}

#[tokio::test]
async fn rate_limiter_locks_after_max_failures() {
    let svc = make_svc(MockAuthRepo::new(vec![]));
    for _ in 0..MAX_LOGIN_ATTEMPTS {
        svc.record_login_failure("1.2.3.4").await;
    }
    assert!(svc.is_locked("1.2.3.4").await);
}

#[tokio::test]
async fn rate_limiter_clear_unlocks() {
    let svc = make_svc(MockAuthRepo::new(vec![]));
    for _ in 0..MAX_LOGIN_ATTEMPTS {
        svc.record_login_failure("1.2.3.4").await;
    }
    svc.clear_login_failures("1.2.3.4").await;
    assert!(!svc.is_locked("1.2.3.4").await);
}

#[tokio::test]
async fn rate_limiter_ips_are_independent() {
    let svc = make_svc(MockAuthRepo::new(vec![]));
    for _ in 0..MAX_LOGIN_ATTEMPTS {
        svc.record_login_failure("1.2.3.4").await;
    }
    assert!(svc.is_locked("1.2.3.4").await);
    assert!(!svc.is_locked("9.9.9.9").await);
}

// ── validate_access_token ─────────────────────────────────────────────────────

#[test]
fn validate_token_valid() {
    let svc = make_svc(MockAuthRepo::new(vec![]));
    let token = svc.create_access_token(1, "admin@test.com", 1).unwrap();
    let claims = svc.validate_access_token(&token).unwrap();
    assert_eq!(claims.sub, 1);
    assert_eq!(claims.email, "admin@test.com");
    assert_eq!(claims.client_category, 1);
}

#[test]
fn validate_token_wrong_secret_rejected() {
    let svc = make_svc(MockAuthRepo::new(vec![]));
    let token = svc.create_access_token(1, "admin@test.com", 1).unwrap();
    let svc2 = AuthServiceImpl::new(crate::auth::AuthServiceDeps {
        auth_repo: Arc::new(MockAuthRepo::new(vec![])),
        jwt_secret: "wrong_secret_key_32chars_padding!".to_string(),
        cache: Arc::new(MockCacheClient::new()),
    });
    assert!(svc2.validate_access_token(&token).is_err());
}

#[test]
fn validate_token_malformed_rejected() {
    let svc = make_svc(MockAuthRepo::new(vec![]));
    assert!(svc.validate_access_token("not.a.jwt").is_err());
    assert!(svc.validate_access_token("").is_err());
}

#[test]
fn validate_token_expired_rejected() {
    let expired_claims = Claims {
        sub: 1,
        email: "admin@test.com".to_string(),
        client_category: 1,
        exp: (Utc::now() - Duration::minutes(5)).timestamp() as usize,
    };
    let token = encode(
        &Header::default(),
        &expired_claims,
        &EncodingKey::from_secret(b"test_secret_key_32chars_padding!!"),
    )
    .unwrap();
    let svc = make_svc(MockAuthRepo::new(vec![]));
    assert!(svc.validate_access_token(&token).is_err());
}

// ── login ─────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn login_success_returns_tokens_with_correct_claims() {
    let svc = make_svc(MockAuthRepo::new(vec![test_user()]));
    let result = svc
        .login("admin@test.com", "correct_password", "1.2.3.4")
        .await
        .unwrap();
    assert!(!result.access_token.is_empty());
    assert!(!result.refresh_token.is_empty());
    let claims = svc.validate_access_token(&result.access_token).unwrap();
    assert_eq!(claims.sub, 1);
    assert_eq!(claims.client_category, 1);
}

#[tokio::test]
async fn login_wrong_password_fails() {
    let svc = make_svc(MockAuthRepo::new(vec![test_user()]));
    assert!(
        svc.login("admin@test.com", "wrong_password", "1.2.3.4")
            .await
            .is_err()
    );
}

#[tokio::test]
async fn login_unknown_email_fails() {
    let svc = make_svc(MockAuthRepo::new(vec![]));
    assert!(
        svc.login("ghost@test.com", "password", "1.2.3.4")
            .await
            .is_err()
    );
}

#[tokio::test]
async fn login_locked_after_max_failures() {
    let svc = make_svc(MockAuthRepo::new(vec![test_user()]));
    for _ in 0..MAX_LOGIN_ATTEMPTS {
        let _ = svc.login("admin@test.com", "wrong", "1.2.3.4").await;
    }
    let err = svc
        .login("admin@test.com", "correct_password", "1.2.3.4")
        .await
        .unwrap_err();
    assert!(err.to_string().contains("Terlalu banyak"));
}

#[tokio::test]
async fn login_success_clears_failure_counter() {
    let svc = make_svc(MockAuthRepo::new(vec![test_user()]));
    for _ in 0..(MAX_LOGIN_ATTEMPTS - 1) {
        let _ = svc.login("admin@test.com", "wrong", "1.2.3.4").await;
    }
    assert!(
        svc.login("admin@test.com", "correct_password", "1.2.3.4")
            .await
            .is_ok()
    );
    assert!(!svc.is_locked("1.2.3.4").await);
}

// ── refresh ───────────────────────────────────────────────────────────────────

#[tokio::test]
async fn refresh_valid_session_returns_new_tokens() {
    let repo = MockAuthRepo::new(vec![test_user()]).with_session(future_session("refresh_tok"));
    let svc = make_svc(repo);
    let result = svc.refresh("refresh_tok", "1.2.3.4").await.unwrap();
    let claims = svc.validate_access_token(&result.access_token).unwrap();
    assert_eq!(claims.sub, 1);
    assert!(!result.refresh_token.is_empty());
    assert_ne!(result.refresh_token, "refresh_tok");
}

#[tokio::test]
async fn refresh_rotates_token_old_invalidated() {
    let repo = MockAuthRepo::new(vec![test_user()]).with_session(future_session("old_tok"));
    let svc = make_svc(repo);
    let result = svc.refresh("old_tok", "1.2.3.4").await.unwrap();
    // old token must be gone
    assert!(svc.refresh("old_tok", "1.2.3.4").await.is_err());
    // new token must work
    assert!(svc.refresh(&result.refresh_token, "1.2.3.4").await.is_ok());
}

#[tokio::test]
async fn refresh_expired_session_fails() {
    let expired = SessionView {
        id: 1,
        user_id: 1,
        token: "expired_tok".to_string(),
        expired_at: Utc::now() - Duration::days(1),
    };
    let repo = MockAuthRepo::new(vec![test_user()]).with_session(expired);
    let svc = make_svc(repo);
    let err = svc.refresh("expired_tok", "1.2.3.4").await.unwrap_err();
    assert!(err.to_string().contains("expired"));
}

#[tokio::test]
async fn refresh_unknown_token_fails() {
    let svc = make_svc(MockAuthRepo::new(vec![]));
    assert!(svc.refresh("nonexistent", "1.2.3.4").await.is_err());
}

// ── logout ────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn logout_invalidates_session() {
    let repo = MockAuthRepo::new(vec![test_user()]).with_session(future_session("logout_tok"));
    let svc = make_svc(repo);
    svc.logout("logout_tok").await.unwrap();
    assert!(svc.refresh("logout_tok", "1.2.3.4").await.is_err());
}
