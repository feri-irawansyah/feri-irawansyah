use super::*;
use chrono::Utc;
use modules::auth::AuthRepository;
use sqlx::PgPool;

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../schemas/migrations");

// Insert a user with disable_login = FALSE and return its id
async fn insert_user(pool: &PgPool, email: &str) -> i32 {
    sqlx::query_scalar(
        "INSERT INTO users (email, password, fullname, disable_login)
         VALUES ($1, 'hashed_pw', 'Test User', FALSE) RETURNING id",
    )
    .bind(email)
    .fetch_one(pool)
    .await
    .unwrap()
}

// ── find_user_by_email ────────────────────────────────────────────────────────
#[sqlx::test(migrator = "MIGRATOR")]
async fn find_user_by_email_returns_none_for_unknown(pool: PgPool) {
    let repo = AuthRepositoryImpl::new(pool);
    assert!(
        repo.find_user_by_email("ghost@test.com")
            .await
            .unwrap()
            .is_none()
    );
}

#[sqlx::test(migrator = "MIGRATOR")]
async fn find_user_by_email_finds_existing(pool: PgPool) {
    insert_user(&pool, "admin@test.com").await;
    let repo = AuthRepositoryImpl::new(pool);
    let user = repo
        .find_user_by_email("admin@test.com")
        .await
        .unwrap()
        .unwrap();
    assert_eq!(user.email, "admin@test.com");
}

#[sqlx::test(migrator = "MIGRATOR")]
async fn find_user_by_email_ignores_disabled_login(pool: PgPool) {
    // default disable_login = TRUE
    sqlx::query("INSERT INTO users (email, password, fullname) VALUES ($1, 'pw', 'Disabled')")
        .bind("disabled@test.com")
        .execute(&pool)
        .await
        .unwrap();
    let repo = AuthRepositoryImpl::new(pool);
    assert!(
        repo.find_user_by_email("disabled@test.com")
            .await
            .unwrap()
            .is_none()
    );
}

// ── find_user_by_id ───────────────────────────────────────────────────────────
#[sqlx::test(migrator = "MIGRATOR")]
async fn find_user_by_id_returns_none_for_unknown(pool: PgPool) {
    let repo = AuthRepositoryImpl::new(pool);
    assert!(repo.find_user_by_id(9999).await.unwrap().is_none());
}

#[sqlx::test(migrator = "MIGRATOR")]
async fn find_user_by_id_finds_existing(pool: PgPool) {
    let id = insert_user(&pool, "byid@test.com").await;
    let repo = AuthRepositoryImpl::new(pool);
    let user = repo.find_user_by_id(id).await.unwrap().unwrap();
    assert_eq!(user.id, id);
}

// ── create_session ────────────────────────────────────────────────────────────
#[sqlx::test(migrator = "MIGRATOR")]
async fn create_session_stores_correct_fields(pool: PgPool) {
    let user_id = insert_user(&pool, "session@test.com").await;
    let repo = AuthRepositoryImpl::new(pool);
    let expired_at = Utc::now() + chrono::Duration::days(7);

    let session = repo
        .create_session(user_id, "tok-abc-123", "127.0.0.1", expired_at)
        .await
        .unwrap();

    assert_eq!(session.token, "tok-abc-123");
    assert_eq!(session.user_id, user_id);
}

// ── find_session_by_token ─────────────────────────────────────────────────────
#[sqlx::test(migrator = "MIGRATOR")]
async fn find_session_by_token_returns_none_for_unknown(pool: PgPool) {
    let repo = AuthRepositoryImpl::new(pool);
    assert!(
        repo.find_session_by_token("nonexistent")
            .await
            .unwrap()
            .is_none()
    );
}

#[sqlx::test(migrator = "MIGRATOR")]
async fn find_session_by_token_finds_created_session(pool: PgPool) {
    let user_id = insert_user(&pool, "find@test.com").await;
    let repo = AuthRepositoryImpl::new(pool);
    let expired_at = Utc::now() + chrono::Duration::days(7);

    repo.create_session(user_id, "find-tok", "1.2.3.4", expired_at)
        .await
        .unwrap();
    let found = repo
        .find_session_by_token("find-tok")
        .await
        .unwrap()
        .unwrap();
    assert_eq!(found.user_id, user_id);
}

// ── delete_session_by_token ───────────────────────────────────────────────────
#[sqlx::test(migrator = "MIGRATOR")]
async fn delete_session_removes_it(pool: PgPool) {
    let user_id = insert_user(&pool, "del@test.com").await;
    let repo = AuthRepositoryImpl::new(pool);
    let expired_at = Utc::now() + chrono::Duration::days(7);

    repo.create_session(user_id, "del-tok", "1.2.3.4", expired_at)
        .await
        .unwrap();
    repo.delete_session_by_token("del-tok").await.unwrap();
    assert!(
        repo.find_session_by_token("del-tok")
            .await
            .unwrap()
            .is_none()
    );
}

#[sqlx::test(migrator = "MIGRATOR")]
async fn delete_nonexistent_session_is_ok(pool: PgPool) {
    let repo = AuthRepositoryImpl::new(pool);
    assert!(repo.delete_session_by_token("ghost-tok").await.is_ok());
}
