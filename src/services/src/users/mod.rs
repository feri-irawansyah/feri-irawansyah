use std::sync::Arc;

pub mod user_service;
pub use user_service::UserServiceImpl;

pub struct UserServiceDeps {
    pub user_repo: Arc<dyn modules::users::UserRepository>,
}
