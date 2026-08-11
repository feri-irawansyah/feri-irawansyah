use anyhow::Result;
use async_trait::async_trait;
use modules::positions::{PositionCommand, PositionRow, PositionService, PositionView};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::positions::PositionServiceDeps;

pub struct PositionServiceImpl {
    repo: Arc<dyn modules::positions::PositionRepository>,
    experience_repo: Arc<dyn modules::experience::ExperienceRepository>,
}

impl PositionServiceImpl {
    pub fn new(deps: PositionServiceDeps) -> Self {
        Self {
            repo: deps.position_repo,
            experience_repo: deps.experience_repo,
        }
    }

    /// Joins position rows against their parent experience (company) records
    /// in application code, so the repository layer never has to JOIN SQL.
    async fn merge(&self, rows: Vec<PositionRow>) -> Result<Vec<PositionView>> {
        let ids: Vec<i32> = rows
            .iter()
            .map(|r| r.experience_id)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        let experiences = self.experience_repo.find_by_ids_async(&ids).await?;
        let exp_by_id: HashMap<i32, modules::experience::ExperienceView> =
            experiences.into_iter().map(|e| (e.id, e)).collect();

        Ok(rows
            .into_iter()
            .filter_map(|r| {
                let exp = exp_by_id.get(&r.experience_id)?;
                Some(PositionView {
                    id: r.id,
                    experience_id: r.experience_id,
                    title: r.title,
                    company: exp.company.clone(),
                    url_docs: exp.url_docs.clone(),
                    image_src: exp.image_src.clone(),
                    address: r.address,
                    start_date: r.start_date,
                    end_date: r.end_date,
                    description: r.description,
                    job_position: r.job_position,
                    job_type: r.job_type,
                    sort_order: r.sort_order,
                })
            })
            .collect())
    }
}

#[async_trait]
impl PositionService for PositionServiceImpl {
    async fn find_all_async(&self) -> Result<Vec<PositionView>> {
        let rows = self.repo.find_all_async().await?;
        self.merge(rows).await
    }

    async fn find_page_async(&self, page: i64, per_page: i64) -> Result<(Vec<PositionView>, i64)> {
        let (rows, total) = self.repo.find_page_async(page, per_page).await?;
        let merged = self.merge(rows).await?;
        Ok((merged, total))
    }

    async fn find_by_id_async(&self, id: i32) -> Result<Option<PositionView>> {
        let Some(row) = self.repo.find_by_id_async(id).await? else {
            return Ok(None);
        };
        let merged = self.merge(vec![row]).await?;
        Ok(merged.into_iter().next())
    }

    async fn create_async(&self, input: PositionCommand) -> Result<PositionView> {
        let row = self.repo.create_async(input).await?;
        let merged = self.merge(vec![row]).await?;
        merged
            .into_iter()
            .next()
            .ok_or_else(|| anyhow::anyhow!("Experience terkait tidak ditemukan"))
    }

    async fn update_async(&self, id: i32, input: PositionCommand) -> Result<Option<PositionView>> {
        let Some(row) = self.repo.update_async(id, input).await? else {
            return Ok(None);
        };
        let merged = self.merge(vec![row]).await?;
        Ok(merged.into_iter().next())
    }

    async fn delete_async(&self, id: i32) -> Result<bool> {
        self.repo.delete_async(id).await
    }
}
