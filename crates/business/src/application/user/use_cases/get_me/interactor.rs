use std::sync::Arc;

use crate::{
    application::user::{
        common::user_profile_response::UserProfileResponse, error::AppError,
        use_cases::get_me::GetMeUseCase,
    },
    domain::user::{
        error::DomainError, repositories::UserRepository, value_objects::user_id::UserId,
    },
};

pub struct GetMeInteractor {
    user_repo: Arc<dyn UserRepository>,
}

impl GetMeInteractor {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }
}

#[async_trait::async_trait]
impl GetMeUseCase for GetMeInteractor {
    async fn execute(&self, user_id: UserId) -> Result<UserProfileResponse, AppError> {
        let user = self
            .user_repo
            .find_by_user_id(&user_id)
            .await?
            .ok_or(DomainError::UserNotFound)?;

        Ok(UserProfileResponse::from(user))
    }
}
