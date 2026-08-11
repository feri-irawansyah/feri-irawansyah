use super::*;
use modules::users::UserRepository;
use sqlx::PgPool;

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../schemas/migrations");

// ── create ────────────────────────────────────────────────────────────────────

#[sqlx::test(migrator = "MIGRATOR")]
async fn create_returns_user_with_correct_fields(pool: PgPool) {
    let repo = UserRepositoryImpl::new(pool);
    let user = repo
        .create_async("new@test.com", "hashed_pw", "New User", 0)
        .await
        .unwrap();
    assert_eq!(user.email, "new@test.com");
    assert_eq!(user.fullname, "New User");
    assert_eq!(user.client_category, 0);
    assert!(user.id > 0);
}

#[sqlx::test(migrator = "MIGRATOR")]
async fn create_duplicate_email_fails(pool: PgPool) {
    let repo = UserRepositoryImpl::new(pool);
    repo.create_async("dup@test.com", "pw", "First", 0).await.unwrap();
    assert!(repo.create_async("dup@test.com", "pw", "Second", 0).await.is_err());
}

// ── find_by_id ────────────────────────────────────────────────────────────────

#[sqlx::test(migrator = "MIGRATOR")]
async fn find_by_id_returns_none_for_unknown(pool: PgPool) {
    let repo = UserRepositoryImpl::new(pool);
    assert!(repo.find_by_id_async(9999).await.unwrap().is_none());
}

#[sqlx::test(migrator = "MIGRATOR")]
async fn find_by_id_finds_created_user(pool: PgPool) {
    let repo = UserRepositoryImpl::new(pool);
    let created = repo.create_async("findid@test.com", "pw", "Find Me", 1).await.unwrap();
    let found = repo.find_by_id_async(created.id).await.unwrap().unwrap();
    assert_eq!(found.id, created.id);
    assert_eq!(found.email, "findid@test.com");
}

// ── find_by_email ─────────────────────────────────────────────────────────────

#[sqlx::test(migrator = "MIGRATOR")]
async fn find_by_email_returns_none_for_unknown(pool: PgPool) {
    let repo = UserRepositoryImpl::new(pool);
    assert!(repo.find_by_email_async("ghost@test.com").await.unwrap().is_none());
}

#[sqlx::test(migrator = "MIGRATOR")]
async fn find_by_email_finds_created_user(pool: PgPool) {
    let repo = UserRepositoryImpl::new(pool);
    repo.create_async("findemail@test.com", "pw", "Find Email", 0).await.unwrap();
    let found = repo.find_by_email_async("findemail@test.com").await.unwrap().unwrap();
    assert_eq!(found.email, "findemail@test.com");
}

// ── find_all ──────────────────────────────────────────────────────────────────

#[sqlx::test(migrator = "MIGRATOR")]
async fn find_all_returns_inserted_users(pool: PgPool) {
    let repo = UserRepositoryImpl::new(pool);
    repo.create_async("a@test.com", "pw", "A", 0).await.unwrap();
    repo.create_async("b@test.com", "pw", "B", 0).await.unwrap();

    let users = repo.find_all_async(10, 0).await.unwrap();
    assert!(users.len() >= 2);
}

#[sqlx::test(migrator = "MIGRATOR")]
async fn find_all_respects_limit(pool: PgPool) {
    let repo = UserRepositoryImpl::new(pool);
    for i in 0..5 {
        repo.create_async(&format!("u{i}@test.com"), "pw", "User", 0).await.unwrap();
    }
    let users = repo.find_all_async(3, 0).await.unwrap();
    assert_eq!(users.len(), 3);
}

#[sqlx::test(migrator = "MIGRATOR")]
async fn find_all_respects_offset(pool: PgPool) {
    let repo = UserRepositoryImpl::new(pool);
    repo.create_async("first@test.com", "pw", "First", 0).await.unwrap();
    repo.create_async("second@test.com", "pw", "Second", 0).await.unwrap();

    let page1 = repo.find_all_async(1, 0).await.unwrap();
    let page2 = repo.find_all_async(1, 1).await.unwrap();
    assert_ne!(page1[0].id, page2[0].id);
}
