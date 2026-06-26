use anyhow::Result;
use async_trait::async_trait;
use modules::certifications::{CertService, CertView};
use std::sync::Arc;

use crate::certifications::CertServiceDeps;

pub struct CertServiceImpl {
    repo: Arc<dyn modules::certifications::CertRepository>,
}

impl CertServiceImpl {
    pub fn new(deps: CertServiceDeps) -> Self {
        Self { repo: deps.cert_repo }
    }
}

#[async_trait]
impl CertService for CertServiceImpl {
    async fn list(&self) -> Result<Vec<CertView>> {
        self.repo.find_all().await
    }
}
