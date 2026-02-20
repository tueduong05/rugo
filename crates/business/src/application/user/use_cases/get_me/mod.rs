use crate::{
    application::user::{common::user_profile_response::UserProfileResponse, error::AppError},
    domain::user::value_objects::user_id::UserId,
};

pub mod interactor;

#[async_trait::async_trait]
pub trait GetMeUseCase: Send + Sync {
    async fn execute(&self, user_id: UserId) -> Result<UserProfileResponse, AppError>;
}
