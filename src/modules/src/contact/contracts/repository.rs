use anyhow::Result;
use async_trait::async_trait;
use crate::contact::contact_req::ContactReq;
use crate::contact::contact_view::ContactView;

#[async_trait]
pub trait ContactRepository: Send + Sync {
    async fn save(&self, req: &ContactReq) -> Result<ContactView>;
    async fn find_all(&self) -> Result<Vec<ContactView>>;
}
