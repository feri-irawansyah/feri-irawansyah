use std::sync::Arc;

use actix_web::{App, HttpMessage, HttpResponse, test, web};
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use modules::auth::{AuthService, Claims, LoginOutcome, LoginResult, MfaEnrollmentView};

use super::{AuthMiddleware, needs_auth_check};

// ── Mock AuthService ────────────────────────────────────────────────────────
// Only `validate_access_token`/`refresh` are ever called by the middleware —
// everything else (login/logout/MFA enrollment) isn't exercised in this
// path, so those just error out if something unexpectedly calls them.

struct MockAuthService;

#[async_trait]
impl AuthService for MockAuthService {
    async fn login(&self, _email: &str, _password: &str, _ip: &str) -> Result<LoginOutcome> {
        Err(anyhow!("not used by AuthMiddleware"))
    }

    async fn verify_mfa(&self, _challenge_token: &str, _code: &str, _ip: &str) -> Result<LoginResult> {
        Err(anyhow!("not used by AuthMiddleware"))
    }

    async fn refresh(&self, refresh_token: &str, _ip: &str) -> Result<LoginResult> {
        if refresh_token == "valid-refresh" {
            Ok(LoginResult {
                access_token: "refreshed-access".to_string(),
                refresh_token: "refreshed-refresh".to_string(),
            })
        } else {
            Err(anyhow!("invalid or expired refresh token"))
        }
    }

    async fn logout(&self, _refresh_token: &str) -> Result<()> {
        Err(anyhow!("not used by AuthMiddleware"))
    }

    fn validate_access_token(&self, token: &str) -> Result<Claims> {
        match token {
            "valid-access" | "refreshed-access" => Ok(Claims {
                sub: 42,
                email: "test@example.com".to_string(),
                client_category: 1,
                exp: 9_999_999_999,
            }),
            _ => Err(anyhow!("invalid access token")),
        }
    }

    async fn enroll_mfa(&self, _user_id: i32) -> Result<MfaEnrollmentView> {
        Err(anyhow!("not used by AuthMiddleware"))
    }

    async fn confirm_mfa(&self, _user_id: i32, _code: &str) -> Result<Vec<String>> {
        Err(anyhow!("not used by AuthMiddleware"))
    }

    async fn disable_mfa(&self, _user_id: i32, _code: &str) -> Result<()> {
        Err(anyhow!("not used by AuthMiddleware"))
    }
}

/// Downstream handler standing in for a real route — reports back whatever
/// the middleware attached to request extensions, so tests can assert on it
/// without needing a real page/service behind the middleware.
async fn probe(req: actix_web::HttpRequest) -> HttpResponse {
    match req.extensions().get::<Claims>() {
        Some(c) => HttpResponse::Ok().json(serde_json::json!({ "sub": c.sub })),
        None => HttpResponse::Ok().json(serde_json::json!({ "claims": false })),
    }
}

fn auth_svc() -> web::Data<Arc<dyn AuthService>> {
    web::Data::new(Arc::new(MockAuthService) as Arc<dyn AuthService>)
}

// ── tests ────────────────────────────────────────────────────────────────────
// These two are plain sync tests, but the glob-imported `actix_web::test`
// module shadows the `test` name that bare `#[test]` resolves through, so
// they need the fully-qualified std attribute to not be mistaken for
// `#[actix_web::test]` (which demands an async fn).

#[::std::prelude::v1::test]
fn needs_auth_check_skips_static_and_infra_paths() {
    assert!(!needs_auth_check("/pkg/feri-irawansyah.wasm"));
    assert!(!needs_auth_check("/pkg/feri-irawansyah.js"));
    assert!(!needs_auth_check("/pkg/feri-irawansyah.css"));
    assert!(!needs_auth_check("/assets/logo.png"));
    assert!(!needs_auth_check("/public/favicon.ico"));
    assert!(!needs_auth_check("/uploads/note-cover.png"));
    assert!(!needs_auth_check("/health"));
    assert!(!needs_auth_check("/robots.txt"));
    assert!(!needs_auth_check("/sitemap.xml"));
    assert!(!needs_auth_check("/rss.xml"));
}

#[::std::prelude::v1::test]
fn needs_auth_check_still_checks_pages_and_server_fns() {
    assert!(needs_auth_check("/"));
    assert!(needs_auth_check("/admin/cache"));
    assert!(needs_auth_check("/notes/some-post"));
    assert!(needs_auth_check("/api/login"));
}

#[actix_web::test]
async fn static_asset_path_bypasses_refresh_even_with_valid_refresh_token() {
    // Regression test for the log-spam/wasted-DB-lookup bug: static asset
    // requests riding along with a still-valid refresh_token used to run the
    // full refresh flow (and could race the page request that already
    // rotated the same single-use token). They should now skip it entirely
    // — no claims attached, no cookies rotated, mock never consulted.
    let app = test::init_service(
        App::new()
            .wrap(AuthMiddleware)
            .app_data(auth_svc())
            .route("/pkg/{filename:.*}", web::get().to(probe)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/pkg/feri-irawansyah.wasm")
        .cookie(actix_web::cookie::Cookie::new("access_token", "expired"))
        .cookie(actix_web::cookie::Cookie::new(
            "refresh_token",
            "valid-refresh",
        ))
        .to_request();
    let res = test::call_service(&app, req).await;

    assert!(
        !res.headers()
            .get_all(actix_web::http::header::SET_COOKIE)
            .any(|_| true)
    );

    let body: serde_json::Value = test::read_body_json(res).await;
    assert_eq!(body, serde_json::json!({ "claims": false }));
}

#[actix_web::test]
async fn no_cookies_lets_request_through_without_claims() {
    let app = test::init_service(
        App::new()
            .wrap(AuthMiddleware)
            .app_data(auth_svc())
            .route("/probe", web::get().to(probe)),
    )
    .await;

    let req = test::TestRequest::get().uri("/probe").to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    assert_eq!(body, serde_json::json!({ "claims": false }));
}

#[actix_web::test]
async fn valid_access_token_attaches_claims() {
    let app = test::init_service(
        App::new()
            .wrap(AuthMiddleware)
            .app_data(auth_svc())
            .route("/probe", web::get().to(probe)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/probe")
        .cookie(actix_web::cookie::Cookie::new("access_token", "valid-access"))
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    assert_eq!(body, serde_json::json!({ "sub": 42 }));
}

#[actix_web::test]
async fn invalid_access_token_without_refresh_leaves_request_unauthenticated() {
    let app = test::init_service(
        App::new()
            .wrap(AuthMiddleware)
            .app_data(auth_svc())
            .route("/probe", web::get().to(probe)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/probe")
        .cookie(actix_web::cookie::Cookie::new("access_token", "garbage"))
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    assert_eq!(body, serde_json::json!({ "claims": false }));
}

#[actix_web::test]
async fn expired_access_token_falls_back_to_refresh_and_rotates_cookies() {
    let app = test::init_service(
        App::new()
            .wrap(AuthMiddleware)
            .app_data(auth_svc())
            .route("/probe", web::get().to(probe)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/probe")
        .cookie(actix_web::cookie::Cookie::new("access_token", "expired"))
        .cookie(actix_web::cookie::Cookie::new(
            "refresh_token",
            "valid-refresh",
        ))
        .to_request();
    let res = test::call_service(&app, req).await;

    let set_cookies: Vec<_> = res
        .headers()
        .get_all(actix_web::http::header::SET_COOKIE)
        .map(|v| v.to_str().unwrap().to_string())
        .collect();
    assert!(set_cookies.iter().any(|c| c.starts_with("access_token=refreshed-access")));
    assert!(set_cookies.iter().any(|c| c.starts_with("refresh_token=refreshed-refresh")));

    let body: serde_json::Value = test::read_body_json(res).await;
    assert_eq!(body, serde_json::json!({ "sub": 42 }));
}

#[actix_web::test]
async fn invalid_refresh_token_clears_both_cookies() {
    // Regression test for the "dead refresh_token spams the log forever"
    // bug: a refresh_token that fails validation never becomes valid on
    // retry, so the browser must be told to stop resending it instead of
    // silently doing nothing (which used to leave `Set-Cookie` empty here).
    let app = test::init_service(
        App::new()
            .wrap(AuthMiddleware)
            .app_data(auth_svc())
            .route("/probe", web::get().to(probe)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/probe")
        .cookie(actix_web::cookie::Cookie::new("access_token", "expired"))
        .cookie(actix_web::cookie::Cookie::new("refresh_token", "garbage"))
        .to_request();
    let res = test::call_service(&app, req).await;

    let set_cookies: Vec<_> = res
        .headers()
        .get_all(actix_web::http::header::SET_COOKIE)
        .map(|v| v.to_str().unwrap().to_string())
        .collect();
    assert!(
        set_cookies
            .iter()
            .any(|c| c.starts_with("access_token=;") && c.contains("Max-Age=0"))
    );
    assert!(
        set_cookies
            .iter()
            .any(|c| c.starts_with("refresh_token=;") && c.contains("Max-Age=0"))
    );

    let body: serde_json::Value = test::read_body_json(res).await;
    assert_eq!(body, serde_json::json!({ "claims": false }));
}
