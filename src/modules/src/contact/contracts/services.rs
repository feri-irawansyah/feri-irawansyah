use anyhow::Result;
use async_trait::async_trait;
use crate::contact::contact_req::ContactReq;

#[async_trait]
pub trait ContactService: Send + Sync {
    async fn send(&self, req: ContactReq) -> Result<()>;
}
