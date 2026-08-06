use anyhow::Result;
use async_trait::async_trait;
use connectors::cache::{self, CacheConn};
use modules::skills::{SkillCommand, SkillService, SkillView};
use std::sync::Arc;

use crate::skills::SkillServiceDeps;

const DOMAIN: &str = "skills";

pub struct SkillServiceImpl {
    repo: Arc<dyn modules::skills::SkillRepository>,
    cache: CacheConn,
}

impl SkillServiceImpl {
    pub fn new(deps: SkillServiceDeps) -> Self {
        Self {
            repo: deps.skill_repo,
            cache: deps.cache,
        }
    }
}

#[async_trait]
impl SkillService for SkillServiceImpl {
    async fn list(&self) -> Result<Vec<SkillView>> {
        let version = cache::cache_version(&self.cache, DOMAIN).await;
        let key = cache::versioned_key(DOMAIN, version, &["all"]);
        if let Some(cached) = cache::get_cached::<Vec<SkillView>>(&self.cache, &key).await {
            return Ok(cached);
        }
        let rows = self.repo.find_all().await?;
        cache::set_cached(&self.cache, &key, &rows).await;
        Ok(rows)
    }

    async fn list_page(&self, page: i64, per_page: i64) -> Result<(Vec<SkillView>, i64)> {
        let version = cache::cache_version(&self.cache, DOMAIN).await;
        let key = cache::versioned_key(
            DOMAIN,
            version,
            &["page", &page.to_string(), &per_page.to_string()],
        );
        if let Some(cached) = cache::get_cached::<(Vec<SkillView>, i64)>(&self.cache, &key).await {
            return Ok(cached);
        }
        let result = self.repo.find_page(page, per_page).await?;
        cache::set_cached(&self.cache, &key, &result).await;
        Ok(result)
    }

    async fn create(&self, input: SkillCommand) -> Result<SkillView> {
        let result = self.repo.create(input).await?;
        cache::bump_cache_version(&self.cache, DOMAIN).await;
        Ok(result)
    }

    async fn update(&self, skill_id: i32, input: SkillCommand) -> Result<Option<SkillView>> {
        let result = self.repo.update(skill_id, input).await?;
        cache::bump_cache_version(&self.cache, DOMAIN).await;
        Ok(result)
    }

    async fn delete(&self, skill_id: i32) -> Result<bool> {
        let result = self.repo.delete(skill_id).await?;
        cache::bump_cache_version(&self.cache, DOMAIN).await;
        Ok(result)
    }
}
