use anyhow::Result;
use async_trait::async_trait;
use modules::contact::{ContactReq, ContactService};
use std::sync::Arc;

use crate::contact::ContactServiceDeps;

pub struct ContactServiceImpl {
    repo: Arc<dyn modules::contact::ContactRepository>,
}

impl ContactServiceImpl {
    pub fn new(deps: ContactServiceDeps) -> Self {
        Self { repo: deps.contact_repo }
    }
}

#[async_trait]
impl ContactService for ContactServiceImpl {
    async fn send(&self, req: ContactReq) -> Result<()> {
        self.repo.save(&req).await?;
        Ok(())
    }
}
