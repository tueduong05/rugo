use crate::domain::{link::{
    entities::Link, error::LinkDomainError, value_objects::short_code::ShortCode,
}, user::value_objects::user_id::UserId};

#[async_trait::async_trait]
pub trait LinkRepository: Send + Sync {
    async fn save(&self, link: &Link) -> Result<(), LinkDomainError>;

    async fn find_by_short_code(&self, short_code: &ShortCode) -> Result<Link, LinkDomainError>;

    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Vec<Link>, LinkDomainError>;
}
