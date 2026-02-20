use crate::domain::link::{
    entities::Link, error::DomainError, value_objects::short_code::ShortCode,
};

#[async_trait::async_trait]
pub trait LinkRepository: Send + Sync {
    async fn save(&self, link: &Link) -> Result<(), DomainError>;

    async fn find_by_short_code(&self, short_code: &ShortCode) -> Result<Link, DomainError>;
}
