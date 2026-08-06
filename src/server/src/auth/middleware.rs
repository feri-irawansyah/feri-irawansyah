use std::future::{Future, Ready, ready};
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready};
use actix_web::http::header::{HeaderValue, SET_COOKIE};
use actix_web::{Error, HttpMessage, web};
use modules::auth::{AuthService, Claims};

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
            let mut refreshed_cookies: Vec<String> = Vec::new();

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

                                let secure = req
                                    .app_data::<web::Data<leptos::config::LeptosOptions>>()
                                    .is_some_and(|o| o.env == leptos::config::Env::PROD);
                                let s = if secure { "; Secure" } else { "" };
                                refreshed_cookies.push(format!(
                                    "access_token={}; HttpOnly; SameSite=Strict; Path=/; Max-Age=900{s}",
                                    result.access_token,
                                ));
                                refreshed_cookies.push(format!(
                                    "refresh_token={}; HttpOnly; SameSite=Strict; Path=/; Max-Age=604800{s}",
                                    result.refresh_token,
                                ));
                            }
                        }
                        Err(e) => {
                            tracing::warn!(ip, path, error = %e, "refresh_token invalid or expired");
                        }
                    }
                }
            }

            let mut res = service.call(req).await?;

            // Attach rotated cookies if a refresh occurred
            for cookie in &refreshed_cookies {
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
