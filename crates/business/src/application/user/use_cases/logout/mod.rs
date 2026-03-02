use crate::{
    application::{error::AppError, user::use_cases::logout::request::LogoutRequest},
    domain::user::value_objects::user_id::UserId,
};

pub mod interactor;
pub mod request;

#[async_trait::async_trait]
pub trait LogoutUseCase: Send + Sync {
    async fn execute(&self, user_id: UserId, req: LogoutRequest) -> Result<(), AppError>;
}
