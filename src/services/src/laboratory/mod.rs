use std::sync::Arc;

pub mod laboratory_service;
pub use laboratory_service::LaboratoryServiceImpl;

pub struct LaboratoryServiceDeps {
    pub laboratory_repo: Arc<dyn modules::laboratory::LaboratoryRepository>,
}
