use std::sync::Arc;

pub mod auth_service;
pub use auth_service::AuthServiceImpl;

pub struct AuthServiceDeps {
    pub auth_repo: Arc<dyn modules::auth::AuthRepository>,
    pub jwt_secret: String,
    pub cache: Arc<dyn connectors::cache::CacheStore>,
}
