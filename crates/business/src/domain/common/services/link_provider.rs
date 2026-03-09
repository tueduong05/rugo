use crate::domain::common::{
    error::BaseDomainError,
    value_objects::{original_link::OriginalLink, user_id::UserId},
};

#[async_trait::async_trait]
pub trait LinkProvider: Send + Sync {
    async fn verify_ownership(
        &self,
        link_id: u64,
        user_id: UserId,
    ) -> Result<OriginalLink, BaseDomainError>;
}
