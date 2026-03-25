use chrono::{DateTime, Utc};

use crate::domain::{
    common::value_objects::user_id::UserId,
    link::{entities::Link, error::LinkDomainError, value_objects::short_code::ShortCode},
};

#[async_trait::async_trait]
pub trait LinkRepository: Send + Sync {
    async fn create(&self, link: &Link) -> Result<(), LinkDomainError>;

    async fn find_by_id(&self, id: u64) -> Result<Option<Link>, LinkDomainError>;

    async fn find_by_short_code(
        &self,
        short_code: &ShortCode,
    ) -> Result<Option<Link>, LinkDomainError>;

    async fn find_by_user_id(&self, user_id: UserId) -> Result<Vec<Link>, LinkDomainError>;

    async fn increment_clicks(
        &self,
        id: u64,
        count: u32,
        now: DateTime<Utc>,
    ) -> Result<u64, LinkDomainError>;
}
