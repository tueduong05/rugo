use crate::{
    application::user::{error::AppError, use_cases::logout::request::LogoutRequest},
    domain::user::value_objects::user_id::UserId,
};

pub mod request;
pub mod interactor;

#[async_trait::async_trait]
pub trait LogoutUseCase: Send + Sync {
    async fn execute(&self, user_id: UserId, req: LogoutRequest) -> Result<(), AppError>;
}
