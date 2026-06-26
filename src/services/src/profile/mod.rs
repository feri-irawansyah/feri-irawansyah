use std::sync::Arc;

pub mod profile_service;
pub use profile_service::ProfileServiceImpl;

pub struct ProfileServiceDeps {
    pub profile_repo: Arc<dyn modules::profile::ProfileRepository>,
}
