use anyhow::Result;
use async_trait::async_trait;
use modules::profile::{ProfileRepository, ProfileView};
use sqlx::PgPool;

pub struct ProfileRepositoryImpl {
    pool: PgPool,
}

impl ProfileRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProfileRepository for ProfileRepositoryImpl {
    async fn find(&self) -> Result<Option<ProfileView>> {
        let row = sqlx::query_as!(
            ProfileView,
            r#"SELECT id, name, title, bio, avatar_url, github_url, linkedin_url, website_url
               FROM profiles LIMIT 1"#
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    async fn upsert(&self, v: &ProfileView) -> Result<ProfileView> {
        let row = sqlx::query_as!(
            ProfileView,
            r#"INSERT INTO profiles (name, title, bio, avatar_url, github_url, linkedin_url, website_url)
               VALUES ($1, $2, $3, $4, $5, $6, $7)
               ON CONFLICT (id) DO UPDATE
               SET name=$1, title=$2, bio=$3, avatar_url=$4, github_url=$5,
                   linkedin_url=$6, website_url=$7, updated_at=NOW()
               RETURNING id, name, title, bio, avatar_url, github_url, linkedin_url, website_url"#,
            v.name, v.title, v.bio, v.avatar_url, v.github_url, v.linkedin_url, v.website_url
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(row)
    }
}
