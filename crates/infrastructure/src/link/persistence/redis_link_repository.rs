use business::domain::{
    common::{error::BaseDomainError, value_objects::user_id::UserId},
    link::{
        entities::Link, error::LinkDomainError, repositories::LinkRepository,
        value_objects::short_code::ShortCode,
    },
};
use chrono::{DateTime, Utc};
use redis::{AsyncCommands, aio::ConnectionManager};

use crate::link::persistence::models::LinkRecord;

pub struct RedisLinkRepository {
    manager: ConnectionManager,
    cache_ttl_seconds: u64,
}

impl RedisLinkRepository {
    pub fn new(manager: ConnectionManager, cache_ttl_seconds: u64) -> Self {
        Self {
            manager,
            cache_ttl_seconds,
        }
    }

    fn short_code_key(code: &str) -> String {
        format!("link:sc:{}", code)
    }
    fn click_counter_key(id: u64) -> String {
        format!("link:clicks:{}", id)
    }
    fn sync_set_key() -> &'static str {
        "links_pending_sync"
    }
}

#[async_trait::async_trait]
impl LinkRepository for RedisLinkRepository {
    #[tracing::instrument(skip(self, link), err, target = "infrastructure")]
    async fn create(&self, link: &Link) -> Result<(), LinkDomainError> {
        let key = Self::short_code_key(&link.short_code.to_string());
        let record = LinkRecord::from(link);
        let serialized = serde_json::to_string(&record)
            .map_err(|e| BaseDomainError::Infrastructure(e.to_string()))?;

        let mut connection = self.manager.clone();
        connection
            .set_ex::<_, _, ()>(key, serialized, self.cache_ttl_seconds)
            .await
            .map_err(|e| BaseDomainError::Infrastructure(e.to_string()))?;

        Ok(())
    }

    #[tracing::instrument(skip(self), err, target = "infrastructure")]
    async fn find_by_id(&self, _id: u64) -> Result<Option<Link>, LinkDomainError> {
        Err(BaseDomainError::Unexpected(
            "find_by_id is unsupported".into(),
        )
        .into())
    }

    #[tracing::instrument(skip(self, short_code), err, target = "infrastructure")]
    async fn find_by_short_code(
        &self,
        short_code: &ShortCode,
    ) -> Result<Option<Link>, LinkDomainError> {
        let key = Self::short_code_key(&short_code.to_string());
        let mut connection = self.manager.clone();

        let cached: Option<String> = connection
            .get(key)
            .await
            .map_err(|e| BaseDomainError::Infrastructure(e.to_string()))?;

        match cached {
            Some(json) => {
                let record: LinkRecord = serde_json::from_str(&json)
                    .map_err(|e| BaseDomainError::Infrastructure(e.to_string()))?;
                record.try_into_domain().map(Some)
            }
            None => Ok(None),
        }
    }

    #[tracing::instrument(skip(self), err, target = "infrastructure")]
    async fn find_by_user_id(&self, _user_id: UserId) -> Result<Vec<Link>, LinkDomainError> {
        Err(BaseDomainError::Unexpected(
            "find_by_user_id is unsupported".into(),
        )
        .into())
    }

    #[tracing::instrument(skip(self, _now), err, target = "infrastructure")]
    async fn increment_clicks(
        &self,
        id: u64,
        count: u32,
        _now: DateTime<Utc>,
    ) -> Result<u64, LinkDomainError> {
        if count == 0 {
            return Ok(0);
        }

        let counter_key = Self::click_counter_key(id);
        let sync_key = Self::sync_set_key();
        let mut connection = self.manager.clone();

        let current_clicks: u64 = connection
            .incr(counter_key, count as i64)
            .await
            .map_err(|e| BaseDomainError::Infrastructure(e.to_string()))?;

        connection
            .sadd::<_, _, ()>(sync_key, id)
            .await
            .map_err(|e| BaseDomainError::Infrastructure(e.to_string()))?;

        Ok(current_clicks)
    }
}
