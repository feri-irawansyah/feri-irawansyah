use super::*;
use modules::users::{UserRepository, UserView};
use std::sync::Mutex;

// ── Mock repository ───────────────────────────────────────────────────────────

struct MockUserRepo {
    users: Mutex<Vec<UserView>>,
    next_id: Mutex<i32>,
}

impl MockUserRepo {
    fn new(users: Vec<UserView>) -> Self {
        let next_id = (users.len() as i32) + 1;
        Self {
            users: Mutex::new(users),
            next_id: Mutex::new(next_id),
        }
    }

    fn empty() -> Self {
        Self { users: Mutex::new(vec![]), next_id: Mutex::new(1) }
    }
}

#[async_trait::async_trait]
impl UserRepository for MockUserRepo {
    async fn create(
        &self,
        email: &str,
        password_hash: &str,
        fullname: &str,
        client_category: i32,
    ) -> anyhow::Result<UserView> {
        let mut id_lock = self.next_id.lock().unwrap();
        let id = *id_lock;
        *id_lock += 1;
        let u = blank_user(id, email, password_hash, fullname, client_category);
        self.users.lock().unwrap().push(u.clone());
        Ok(u)
    }

    async fn find_by_id(&self, id: i32) -> anyhow::Result<Option<UserView>> {
        Ok(self.users.lock().unwrap().iter().find(|u| u.id == id).cloned())
    }

    async fn find_by_email(&self, email: &str) -> anyhow::Result<Option<UserView>> {
        Ok(self.users.lock().unwrap().iter().find(|u| u.email == email).cloned())
    }

    async fn list(&self, limit: i64, offset: i64) -> anyhow::Result<Vec<UserView>> {
        let users = self.users.lock().unwrap();
        Ok(users.iter().skip(offset as usize).take(limit as usize).cloned().collect())
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn blank_user(id: i32, email: &str, password: &str, fullname: &str, client_category: i32) -> UserView {
    use chrono::Utc;
    UserView {
        id,
        email: email.to_string(),
        password: password.to_string(),
        fullname: fullname.to_string(),
        mobile_phone: String::new(),
        picture: None,
        google_id: String::new(),
        client_category,
        activate_code: String::new(),
        otp_generated_link: String::new(),
        otp_generated_link_date: Utc::now(),
        count_resend_activation: 0,
        activate_time: None,
        disable_login: false,
        reset_password_key: String::new(),
        reset_password_flag: false,
        reset_password_date: None,
        last_login: None,
        register_date: Utc::now(),
        updated_at: Utc::now(),
    }
}

fn make_svc(repo: MockUserRepo) -> UserServiceImpl {
    UserServiceImpl::new(crate::users::UserServiceDeps { user_repo: Arc::new(repo) })
}

fn seed_users(n: usize) -> Vec<UserView> {
    (1..=n)
        .map(|i| blank_user(i as i32, &format!("user{i}@test.com"), "hash", &format!("User {i}"), 0))
        .collect()
}

// ── create ────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn create_hashes_password_not_plaintext() {
    let svc = make_svc(MockUserRepo::empty());
    let result = svc.create("new@test.com", "plaintext", "New User", 0).await.unwrap();
    assert_ne!(result.password, "plaintext", "password must be hashed");
    assert!(result.password.starts_with("$argon2"), "must be argon2 hash");
}

#[tokio::test]
async fn create_stores_correct_metadata() {
    let svc = make_svc(MockUserRepo::empty());
    let result = svc.create("meta@test.com", "pass", "Full Name", 1).await.unwrap();
    assert_eq!(result.email, "meta@test.com");
    assert_eq!(result.fullname, "Full Name");
    assert_eq!(result.client_category, 1);
}

// ── find_by_id ────────────────────────────────────────────────────────────────

#[tokio::test]
async fn find_by_id_returns_user_when_found() {
    let users = seed_users(3);
    let svc = make_svc(MockUserRepo::new(users));
    let found = svc.find_by_id(2).await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().id, 2);
}

#[tokio::test]
async fn find_by_id_returns_none_when_missing() {
    let svc = make_svc(MockUserRepo::empty());
    assert!(svc.find_by_id(99).await.unwrap().is_none());
}

// ── find_by_email ─────────────────────────────────────────────────────────────

#[tokio::test]
async fn find_by_email_returns_user_when_found() {
    let users = seed_users(3);
    let svc = make_svc(MockUserRepo::new(users));
    let found = svc.find_by_email("user2@test.com").await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().email, "user2@test.com");
}

#[tokio::test]
async fn find_by_email_returns_none_when_missing() {
    let svc = make_svc(MockUserRepo::empty());
    assert!(svc.find_by_email("ghost@test.com").await.unwrap().is_none());
}

// ── list ─────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn list_respects_limit() {
    let users = seed_users(10);
    let svc = make_svc(MockUserRepo::new(users));
    let page = svc.list(3, 0).await.unwrap();
    assert_eq!(page.len(), 3);
}

#[tokio::test]
async fn list_respects_offset() {
    let users = seed_users(5);
    let svc = make_svc(MockUserRepo::new(users));
    let page = svc.list(10, 3).await.unwrap();
    assert_eq!(page.len(), 2);
    assert_eq!(page[0].id, 4);
}

#[tokio::test]
async fn list_empty_repo_returns_empty() {
    let svc = make_svc(MockUserRepo::empty());
    assert!(svc.list(20, 0).await.unwrap().is_empty());
}
