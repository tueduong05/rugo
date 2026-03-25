use std::sync::Arc;

use business::domain::{
    common::value_objects::user_id::UserId,
    link::{
        entities::Link, error::LinkDomainError, repositories::LinkRepository,
        value_objects::short_code::ShortCode,
    },
};
use chrono::{DateTime, Utc};

pub struct CacheAsideLinkRepository {
    postgres_repo: Arc<dyn LinkRepository>,
    redis_repo: Arc<dyn LinkRepository>,
}

impl CacheAsideLinkRepository {
    pub fn new(postgres_repo: Arc<dyn LinkRepository>, redis_repo: Arc<dyn LinkRepository>) -> Self {
        Self {
            postgres_repo,
            redis_repo,
        }
    }
}

#[async_trait::async_trait]
impl LinkRepository for CacheAsideLinkRepository {
    #[tracing::instrument(skip(self, link), err, target = "infrastructure")]
    async fn create(&self, link: &Link) -> Result<(), LinkDomainError> {
        self.postgres_repo.create(link).await?;

        self.redis_repo.create(link).await?;

        Ok(())
    }

    #[tracing::instrument(skip(self), err, target = "infrastructure")]
    async fn find_by_id(&self, id: u64) -> Result<Option<Link>, LinkDomainError> {
        self.postgres_repo.find_by_id(id).await
    }

    #[tracing::instrument(skip(self, short_code), err, target = "infrastructure")]
    async fn find_by_short_code(
        &self,
        short_code: &ShortCode,
    ) -> Result<Option<Link>, LinkDomainError> {
        if let Some(link) = self.redis_repo.find_by_short_code(short_code).await? {
            return Ok(Some(link));
        }

        let link = self.postgres_repo.find_by_short_code(short_code).await?;

        if let Some(ref link) = link {
            self.redis_repo.create(link).await?;
        }

        Ok(link)
    }

    #[tracing::instrument(skip(self), err, target = "infrastructure")]
    async fn find_by_user_id(&self, user_id: UserId) -> Result<Vec<Link>, LinkDomainError> {
        self.postgres_repo.find_by_user_id(user_id).await
    }

    #[tracing::instrument(skip(self, now), err, target = "infrastructure")]
    async fn increment_clicks(
        &self,
        id: u64,
        count: u32,
        now: DateTime<Utc>,
    ) -> Result<u64, LinkDomainError> {
        self.redis_repo.increment_clicks(id, count, now).await
    }
}
