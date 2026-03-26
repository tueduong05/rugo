use business::domain::{
    common::{error::BaseDomainError, value_objects::user_id::UserId},
    link::{
        entities::Link, error::LinkDomainError, repositories::LinkRepository,
        value_objects::short_code::ShortCode,
    },
};
use chrono::{DateTime, Utc};
use redis::{Script, aio::ConnectionManager};

use crate::link::persistence::models::LinkRecord;

pub struct RedisLinkRepository {
    manager: ConnectionManager,
    cache_ttl_seconds: u64,
    max_clicks_ttl_seconds: u64,
}

impl RedisLinkRepository {
    pub fn new(
        manager: ConnectionManager,
        cache_ttl_seconds: u64,
        max_clicks_ttl_seconds: u64,
    ) -> Self {
        Self {
            manager,
            cache_ttl_seconds,
            max_clicks_ttl_seconds,
        }
    }

    fn short_code_key(code: &str) -> String {
        format!("link:sc:{}", code)
    }
    fn click_counter_key(id: u64) -> String {
        format!("link:clicks:{}", id)
    }
    fn max_clicks_key(id: u64) -> String {
        format!("link:max:{}", id)
    }
    fn sync_set_key() -> &'static str {
        "link:pending_sync"
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
        let mut pipeline = redis::pipe();
        pipeline
            .atomic()
            .cmd("SETEX")
            .arg(&key)
            .arg(self.cache_ttl_seconds)
            .arg(&serialized)
            .ignore();

        if let Some(link_id) = record.id {
            let max_clicks_key = Self::max_clicks_key(link_id as u64);
            let max_clicks_value = record.max_clicks.unwrap_or(-1);
            pipeline
                .cmd("SETEX")
                .arg(&max_clicks_key)
                .arg(self.max_clicks_ttl_seconds)
                .arg(max_clicks_value)
                .ignore();
        }

        pipeline
            .query_async::<()>(&mut connection)
            .await
            .map_err(|e| BaseDomainError::Infrastructure(e.to_string()))?;

        Ok(())
    }

    #[tracing::instrument(skip(self), err, target = "infrastructure")]
    async fn find_by_id(&self, _id: u64) -> Result<Option<Link>, LinkDomainError> {
        Err(BaseDomainError::Unexpected("find_by_id is unsupported".into()).into())
    }

    #[tracing::instrument(skip(self, short_code), err, target = "infrastructure")]
    async fn find_by_short_code(
        &self,
        short_code: &ShortCode,
    ) -> Result<Option<Link>, LinkDomainError> {
        let key = Self::short_code_key(&short_code.to_string());
        let mut connection = self.manager.clone();

        let script = Script::new(
            r#"
            local link_key = KEYS[1]
            local link_json = redis.call('GET', link_key)

            if not link_json then
                return nil
            end

            local link_obj = cjson.decode(link_json)
            local link_id = link_obj['id']

            if link_id then
                local counter_key = 'link:clicks:' .. tostring(link_id)
                local live_clicks = redis.call('GET', counter_key)
                if live_clicks then
                    link_obj['current_clicks'] = tonumber(live_clicks)
                end
            end

            return cjson.encode(link_obj)
            "#,
        );

        let cached: Option<String> = script
            .key(key)
            .invoke_async(&mut connection)
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
        Err(BaseDomainError::Unexpected("find_by_user_id is unsupported".into()).into())
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
        let max_clicks_key = Self::max_clicks_key(id);
        let sync_key = Self::sync_set_key();
        let mut connection = self.manager.clone();

        // 0 => rejected
        // 1 => accepted
        // 2 => backfill from db and retry
        // TODO: Avoid using magic numbers (0, 1, 2) for return codes
        let script = Script::new(
            r#"
            local counter_key = KEYS[1]
            local max_clicks_key = KEYS[2]
            local sync_key = KEYS[3]

            local count = tonumber(ARGV[1])
            local link_id = ARGV[2]

            if not count or count <= 0 then
                return 0
            end

            local max_clicks = redis.call('GET', max_clicks_key)
            if not max_clicks then
                return 2
            end

            local max_value = tonumber(max_clicks)
            if not max_value then
                return 2
            end

            local new_clicks = redis.call('INCRBY', counter_key, count)

            if max_value >= 0 and new_clicks > max_value then
                redis.call('DECRBY', counter_key, count)
                return 0
            end

            redis.call('SADD', sync_key, link_id)
            return 1
            "#,
        );

        let rows_affected: i64 = script
            .key(counter_key)
            .key(max_clicks_key)
            .key(sync_key)
            .arg(count as i64)
            .arg(id)
            .invoke_async(&mut connection)
            .await
            .map_err(|e| BaseDomainError::Infrastructure(e.to_string()))?;

        Ok(rows_affected as u64)
    }
}
