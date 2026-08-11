use anyhow::Result;
use async_trait::async_trait;
use crate::laboratory::laboratory_command::LaboratoryCommand;
use crate::laboratory::laboratory_view::LaboratoryView;

#[async_trait]
pub trait LaboratoryService: Send + Sync {
    async fn find_by_slug_async(&self, slug: &str) -> Result<Option<LaboratoryView>>;
    async fn find_by_category_page_async(
        &self,
        category: &str,
        page: i64,
        per_page: i64,
    ) -> Result<(Vec<LaboratoryView>, i64)>;
    async fn find_all_admin_async(&self) -> Result<Vec<LaboratoryView>>;
    async fn find_all_admin_page_async(&self, page: i64, per_page: i64) -> Result<(Vec<LaboratoryView>, i64)>;
    async fn create_async(&self, input: LaboratoryCommand) -> Result<LaboratoryView>;
    async fn update_async(&self, id: i32, input: LaboratoryCommand) -> Result<Option<LaboratoryView>>;
    async fn delete_async(&self, id: i32) -> Result<bool>;
}
