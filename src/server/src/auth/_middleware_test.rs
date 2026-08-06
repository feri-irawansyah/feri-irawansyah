use std::sync::Arc;

use actix_web::{App, HttpMessage, HttpResponse, test, web};
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use modules::auth::{AuthService, Claims, LoginResult};

use super::AuthMiddleware;

// ── Mock AuthService ────────────────────────────────────────────────────────
// Only `validate_access_token`/`refresh` are ever called by the middleware —
// `login`/`logout` aren't exercised in this path, so they just error out if
// something unexpectedly calls them.

struct MockAuthService;

#[async_trait]
impl AuthService for MockAuthService {
    async fn login(&self, _email: &str, _password: &str, _ip: &str) -> Result<LoginResult> {
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
async fn invalid_refresh_token_leaves_request_unauthenticated_and_sets_no_cookies() {
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

    assert!(
        !res.headers()
            .get_all(actix_web::http::header::SET_COOKIE)
            .any(|_| true)
    );

    let body: serde_json::Value = test::read_body_json(res).await;
    assert_eq!(body, serde_json::json!({ "claims": false }));
}
