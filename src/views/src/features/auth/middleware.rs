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
            let mut refreshed_cookie: Option<String> = None;

            if let Some(auth_svc) = req.app_data::<web::Data<Arc<dyn AuthService>>>().cloned() {
                // Extract cookie values first so we release the borrow on req
                let access_val = req.cookie("access_token").map(|c| c.value().to_owned());
                let refresh_val = req.cookie("refresh_token").map(|c| c.value().to_owned());

                // 1. Try access_token
                if let Some(token) = access_val
                    && let Ok(claims) = auth_svc.validate_access_token(&token)
                {
                    req.extensions_mut().insert(claims);
                }

                // 2. Fall back to refresh_token
                if req.extensions().get::<Claims>().is_none()
                    && let Some(token) = refresh_val
                    && let Ok(new_token) = auth_svc.refresh(&token).await
                    && let Ok(claims) = auth_svc.validate_access_token(&new_token)
                {
                    req.extensions_mut().insert(claims);

                    let secure = req
                        .app_data::<web::Data<leptos::config::LeptosOptions>>()
                        .is_some_and(|o| o.env == leptos::config::Env::PROD);
                    refreshed_cookie = Some(format!(
                        "access_token={}; HttpOnly; SameSite=Strict; Path=/; Max-Age=900{}",
                        new_token,
                        if secure { "; Secure" } else { "" }
                    ));
                }
            }

            let mut res = service.call(req).await?;

            // Attach new access_token cookie if a refresh occurred
            if let Some(cookie) = refreshed_cookie
                && let Ok(val) = HeaderValue::from_str(&cookie)
            {
                res.headers_mut().append(SET_COOKIE, val);
            }

            Ok(res)
        })
    }
}
