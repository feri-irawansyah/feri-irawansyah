use std::sync::Arc;

pub mod cert_service;
pub use cert_service::CertServiceImpl;

pub struct CertServiceDeps {
    pub cert_repo: Arc<dyn modules::certifications::CertRepository>,
}
