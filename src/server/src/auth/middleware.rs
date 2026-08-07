use std::future::{Future, Ready, ready};
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready};
use actix_web::http::header::{HeaderValue, SET_COOKIE};
use actix_web::{Error, HttpMessage, web};
use modules::auth::{AuthService, Claims};

/// Path prefixes that never need `Claims` and never carry a session worth
/// spending a DB round-trip on. Wrapped at the `App` level, `AuthMiddleware`
/// otherwise runs its full cookie/refresh flow on every request — including
/// the JS/CSS/WASM bundle the browser fetches in parallel on every page
/// load, each racing the same single-use `refresh_token` against the page
/// request that already rotated it. That produced nothing but wasted lookups
/// and misleading "refresh_token invalid or expired" warnings for requests
/// that were never going to read the claims anyway.
const NO_AUTH_PREFIXES: &[&str] = &[
    "/pkg/",
    "/assets/",
    "/public/",
    "/uploads/",
    "/health",
    "/robots.txt",
    "/sitemap.xml",
    "/rss.xml",
];

fn needs_auth_check(path: &str) -> bool {
    !NO_AUTH_PREFIXES.iter().any(|p| path.starts_with(p))
}

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareInner<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareInner {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthMiddlewareInner<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareInner<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<ServiceResponse<B>, Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();

        Box::pin(async move {
            let mut cookie_updates: Vec<String> = Vec::new();

            if !needs_auth_check(req.path()) {
                return service.call(req).await;
            }

            if let Some(auth_svc) = req.app_data::<web::Data<Arc<dyn AuthService>>>().cloned() {
                // Extract cookie values first so we release the borrow on req
                let access_val = req.cookie("access_token").map(|c| c.value().to_owned());
                let refresh_val = req.cookie("refresh_token").map(|c| c.value().to_owned());

                let ip = req
                    .connection_info()
                    .realip_remote_addr()
                    .unwrap_or("unknown")
                    .to_owned();
                let path = req.path().to_owned();

                let secure = req
                    .app_data::<web::Data<leptos::config::LeptosOptions>>()
                    .is_some_and(|o| o.env == leptos::config::Env::PROD);
                let s = if secure { "; Secure" } else { "" };

                // 1. Try access_token
                if let Some(token) = access_val {
                    match auth_svc.validate_access_token(&token) {
                        Ok(claims) => {
                            tracing::debug!(ip, path, user_id = claims.sub, "auth ok");
                            req.extensions_mut().insert(claims);
                        }
                        Err(e) => {
                            tracing::warn!(ip, path, error = %e, "access_token invalid");
                        }
                    }
                }

                // 2. Fall back to refresh_token
                if req.extensions().get::<Claims>().is_none()
                    && let Some(token) = refresh_val
                {
                    match auth_svc.refresh(&token, &ip).await {
                        Ok(result) => {
                            if let Ok(claims) = auth_svc.validate_access_token(&result.access_token)
                            {
                                tracing::info!(ip, path, user_id = claims.sub, "token refreshed");
                                req.extensions_mut().insert(claims);

                                cookie_updates.push(format!(
                                    "access_token={}; HttpOnly; SameSite=Strict; Path=/; Max-Age=900{s}",
                                    result.access_token,
                                ));
                                cookie_updates.push(format!(
                                    "refresh_token={}; HttpOnly; SameSite=Strict; Path=/; Max-Age=604800{s}",
                                    result.refresh_token,
                                ));
                            }
                        }
                        Err(e) => {
                            tracing::warn!(ip, path, error = %e, "refresh_token invalid or expired");

                            // The refresh_token is dead (expired/rotated-away/
                            // session deleted) and never becomes valid again on
                            // its own — without this, the browser keeps resending
                            // it on every single request forever, each one paying
                            // for a DB lookup that's guaranteed to fail. Expire
                            // both cookies so the browser stops offering them.
                            cookie_updates.push(format!(
                                "access_token=; HttpOnly; SameSite=Strict; Path=/; Max-Age=0{s}"
                            ));
                            cookie_updates.push(format!(
                                "refresh_token=; HttpOnly; SameSite=Strict; Path=/; Max-Age=0{s}"
                            ));
                        }
                    }
                }
            }

            let mut res = service.call(req).await?;

            // Attach rotated (or cleared) cookies if a refresh was attempted
            for cookie in &cookie_updates {
                if let Ok(val) = HeaderValue::from_str(cookie) {
                    res.headers_mut().append(SET_COOKIE, val);
                }
            }

            Ok(res)
        })
    }
}

#[cfg(test)]
#[path = "_middleware_test.rs"]
mod tests;
