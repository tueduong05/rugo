use std::{collections::HashMap, sync::Arc};

use business::domain::{
    common::{error::BaseDomainError, value_objects::user_id::UserId},
    link::{
        entities::Link, error::LinkDomainError, repositories::LinkRepository,
        value_objects::short_code::ShortCode,
    },
};
use chrono::{DateTime, Utc};
use tokio::sync::{Mutex, Notify};

pub struct CacheAsideLinkRepository {
    pg_link_repo: Arc<dyn LinkRepository>,
    redis_link_repo: Arc<dyn LinkRepository>,
    backfill_in_flight: Mutex<HashMap<u64, Arc<Notify>>>,
}

impl CacheAsideLinkRepository {
    pub fn new(
        pg_link_repo: Arc<dyn LinkRepository>,
        redis_link_repo: Arc<dyn LinkRepository>,
    ) -> Self {
        Self {
            pg_link_repo,
            redis_link_repo,
            backfill_in_flight: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait::async_trait]
impl LinkRepository for CacheAsideLinkRepository {
    #[tracing::instrument(skip(self, link), err, target = "infrastructure")]
    async fn create(&self, link: &Link) -> Result<(), LinkDomainError> {
        self.pg_link_repo.create(link).await?;

        self.redis_link_repo.create(link).await?;

        Ok(())
    }

    #[tracing::instrument(skip(self), err, target = "infrastructure")]
    async fn find_by_id(&self, id: u64) -> Result<Option<Link>, LinkDomainError> {
        self.pg_link_repo.find_by_id(id).await
    }

    #[tracing::instrument(skip(self, short_code), err, target = "infrastructure")]
    async fn find_by_short_code(
        &self,
        short_code: &ShortCode,
    ) -> Result<Option<Link>, LinkDomainError> {
        if let Some(link) = self.redis_link_repo.find_by_short_code(short_code).await? {
            return Ok(Some(link));
        }

        let link = self.pg_link_repo.find_by_short_code(short_code).await?;

        if let Some(ref link) = link {
            self.redis_link_repo.create(link).await?;
        }

        Ok(link)
    }

    #[tracing::instrument(skip(self), err, target = "infrastructure")]
    async fn find_by_user_id(&self, user_id: UserId) -> Result<Vec<Link>, LinkDomainError> {
        self.pg_link_repo.find_by_user_id(user_id).await
    }

    #[tracing::instrument(skip(self, now), err, target = "infrastructure")]
    async fn increment_clicks(
        &self,
        id: u64,
        count: u32,
        now: DateTime<Utc>,
    ) -> Result<u64, LinkDomainError> {
        let redis_result = self
            .redis_link_repo
            .increment_clicks(id, count, now)
            .await?;

        if redis_result != 2 {
            return Ok(redis_result);
        }

        let (is_leader, notify) = {
            let mut in_flight = self.backfill_in_flight.lock().await;
            if let Some(existing_notify) = in_flight.get(&id) {
                (false, Arc::clone(existing_notify))
            } else {
                let notify = Arc::new(Notify::new());
                in_flight.insert(id, Arc::clone(&notify));
                (true, notify)
            }
        };

        if is_leader {
            let backfill_result = async {
                if let Some(link) = self.pg_link_repo.find_by_id(id).await? {
                    self.redis_link_repo.create(&link).await?;
                    let retried = self
                        .redis_link_repo
                        .increment_clicks(id, count, now)
                        .await?;
                    if retried == 2 {
                        return Err(
                            BaseDomainError::Infrastructure("Backfill failed".into()).into()
                        );
                    }
                    return Ok(retried);
                }

                Ok(0)
            }
            .await;

            let mut in_flight = self.backfill_in_flight.lock().await;
            if let Some(notify) = in_flight.remove(&id) {
                notify.notify_waiters();
            }

            return backfill_result;
        }

        notify.notified().await;
        let retried = self
            .redis_link_repo
            .increment_clicks(id, count, now)
            .await?;
        if retried == 2 {
            return Err(BaseDomainError::Infrastructure("Backfill failed".into()).into());
        }
        Ok(retried)
    }
}
