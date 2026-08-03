use anyhow::Result;
use async_trait::async_trait;
use crate::laboratory::laboratory_command::LaboratoryCommand;
use crate::laboratory::laboratory_view::LaboratoryView;

#[async_trait]
pub trait LaboratoryRepository: Send + Sync {
    async fn find_by_slug(&self, slug: &str) -> Result<Option<LaboratoryView>>;
    /// Public, paginated listing scoped to one category (`enabled = TRUE` only)
    /// — backs the `/laboratory/:category` list page.
    async fn find_by_category_page(
        &self,
        category: &str,
        page: i64,
        per_page: i64,
    ) -> Result<(Vec<LaboratoryView>, i64)>;
    /// Every entry regardless of category/enabled — for the admin listing.
    async fn find_all_admin(&self) -> Result<Vec<LaboratoryView>>;
    async fn find_all_admin_page(&self, page: i64, per_page: i64) -> Result<(Vec<LaboratoryView>, i64)>;
    async fn create(&self, input: LaboratoryCommand) -> Result<LaboratoryView>;
    async fn update(&self, id: i32, input: LaboratoryCommand) -> Result<Option<LaboratoryView>>;
    async fn delete(&self, id: i32) -> Result<bool>;
}
