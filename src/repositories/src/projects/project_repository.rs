use anyhow::Result;
use async_trait::async_trait;
use modules::projects::{ProjectRepository, ProjectView};
use sqlx::PgPool;

pub struct ProjectRepositoryImpl {
    pool: PgPool,
}

impl ProjectRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProjectRepository for ProjectRepositoryImpl {
    async fn find_all(&self) -> Result<Vec<ProjectView>> {
        let rows = sqlx::query_as!(
            ProjectView,
            r#"SELECT id, title, slug, summary, description, tech_stack, repo_url, demo_url, image_url, featured, sort_order
               FROM projects ORDER BY sort_order, created_at DESC"#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    async fn find_featured(&self) -> Result<Vec<ProjectView>> {
        let rows = sqlx::query_as!(
            ProjectView,
            r#"SELECT id, title, slug, summary, description, tech_stack, repo_url, demo_url, image_url, featured, sort_order
               FROM projects WHERE featured = TRUE ORDER BY sort_order"#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    async fn find_by_slug(&self, slug: &str) -> Result<Option<ProjectView>> {
        let row = sqlx::query_as!(
            ProjectView,
            r#"SELECT id, title, slug, summary, description, tech_stack, repo_url, demo_url, image_url, featured, sort_order
               FROM projects WHERE slug = $1"#,
            slug
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }
}
