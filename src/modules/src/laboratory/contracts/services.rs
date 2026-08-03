use anyhow::Result;
use async_trait::async_trait;
use crate::laboratory::laboratory_command::LaboratoryCommand;
use crate::laboratory::laboratory_view::LaboratoryView;

#[async_trait]
pub trait LaboratoryService: Send + Sync {
    async fn get_by_slug(&self, slug: &str) -> Result<Option<LaboratoryView>>;
    async fn by_category_page(
        &self,
        category: &str,
        page: i64,
        per_page: i64,
    ) -> Result<(Vec<LaboratoryView>, i64)>;
    async fn list_admin(&self) -> Result<Vec<LaboratoryView>>;
    async fn list_admin_page(&self, page: i64, per_page: i64) -> Result<(Vec<LaboratoryView>, i64)>;
    async fn create(&self, input: LaboratoryCommand) -> Result<LaboratoryView>;
    async fn update(&self, id: i32, input: LaboratoryCommand) -> Result<Option<LaboratoryView>>;
    async fn delete(&self, id: i32) -> Result<bool>;
}
