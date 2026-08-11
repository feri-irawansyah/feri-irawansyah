use anyhow::Result;
use async_trait::async_trait;
use connectors::cache::{self, CacheStore};
use modules::skills::{SkillCommand, SkillService, SkillView};
use std::sync::Arc;

use crate::skills::SkillServiceDeps;

const DOMAIN: &str = "skills";

pub struct SkillServiceImpl {
    repo: Arc<dyn modules::skills::SkillRepository>,
    cache: Arc<dyn CacheStore>,
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
    async fn find_all_async(&self) -> Result<Vec<SkillView>> {
        let version = cache::cache_version(self.cache.as_ref(), DOMAIN).await;
        let key = cache::versioned_key(DOMAIN, version, &["all"]);
        if let Some(cached) = cache::get_cached::<Vec<SkillView>>(self.cache.as_ref(), &key).await {
            return Ok(cached);
        }
        let rows = self.repo.find_all_async().await?;
        cache::set_cached(self.cache.as_ref(), &key, &rows).await;
        Ok(rows)
    }

    async fn find_page_async(&self, page: i64, per_page: i64) -> Result<(Vec<SkillView>, i64)> {
        let version = cache::cache_version(self.cache.as_ref(), DOMAIN).await;
        let key = cache::versioned_key(
            DOMAIN,
            version,
            &["page", &page.to_string(), &per_page.to_string()],
        );
        if let Some(cached) = cache::get_cached::<(Vec<SkillView>, i64)>(self.cache.as_ref(), &key).await {
            return Ok(cached);
        }
        let result = self.repo.find_page_async(page, per_page).await?;
        cache::set_cached(self.cache.as_ref(), &key, &result).await;
        Ok(result)
    }

    async fn create_async(&self, input: SkillCommand) -> Result<SkillView> {
        let result = self.repo.create_async(input).await?;
        cache::bump_cache_version(self.cache.as_ref(), DOMAIN).await;
        Ok(result)
    }

    async fn update_async(&self, skill_id: i32, input: SkillCommand) -> Result<Option<SkillView>> {
        let result = self.repo.update_async(skill_id, input).await?;
        cache::bump_cache_version(self.cache.as_ref(), DOMAIN).await;
        Ok(result)
    }

    async fn delete_async(&self, skill_id: i32) -> Result<bool> {
        let result = self.repo.delete_async(skill_id).await?;
        cache::bump_cache_version(self.cache.as_ref(), DOMAIN).await;
        Ok(result)
    }
}
