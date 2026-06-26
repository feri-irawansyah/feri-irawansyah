use anyhow::Result;
use async_trait::async_trait;
use crate::certifications::cert_view::CertView;

#[async_trait]
pub trait CertService: Send + Sync {
    async fn list(&self) -> Result<Vec<CertView>>;
}
