use std::sync::Arc;

pub mod contact_service;
pub use contact_service::ContactServiceImpl;

pub struct ContactServiceDeps {
    pub contact_repo: Arc<dyn modules::contact::ContactRepository>,
}
