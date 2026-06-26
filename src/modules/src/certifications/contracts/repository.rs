use anyhow::Result;
use async_trait::async_trait;
use crate::certifications::cert_view::CertView;

#[async_trait]
pub trait CertRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<CertView>>;
    async fn find_by_id(&self, id: i32) -> Result<Option<CertView>>;
}
