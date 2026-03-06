use std::sync::Arc;

use crate::domain::{
    common::{
        error::BaseDomainError,
        services::link_provider::LinkProvider,
        value_objects::{original_link::OriginalLink, user_id::UserId},
    },
    link::repositories::LinkRepository,
};

pub struct LinkProviderImpl {
    link_repo: Arc<dyn LinkRepository>,
}

impl LinkProviderImpl {
    pub fn new(link_repo: Arc<dyn LinkRepository>) -> Self {
        Self { link_repo }
    }
}

#[async_trait::async_trait]
impl LinkProvider for LinkProviderImpl {
    async fn verify_ownership(
        &self,
        link_id: u64,
        user_id: UserId,
    ) -> Result<OriginalLink, BaseDomainError> {
        let link = self
            .link_repo
            .find_by_id(link_id)
            .await
            .map_err(|e| BaseDomainError::Infrastructure(e.to_string()))?
            .ok_or_else(|| BaseDomainError::ResourceNotFound("Link".into()))?;

        if link.user_id != Some(user_id) {
            return Err(BaseDomainError::AccessDenied);
        }

        Ok(link.original_link)
    }
}
