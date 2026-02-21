use crate::{
    application::{error::AppError, user::common::user_profile_response::UserProfileResponse},
    domain::user::value_objects::user_id::UserId,
};

pub mod interactor;

#[async_trait::async_trait]
pub trait GetMeUseCase: Send + Sync {
    async fn execute(&self, user_id: UserId) -> Result<UserProfileResponse, AppError>;
}
