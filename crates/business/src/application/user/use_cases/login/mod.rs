use crate::application::{
    error::AppError,
    user::{common::auth_response::AuthResponse, use_cases::login::request::LoginRequest},
};

pub mod interactor;
pub mod request;

#[async_trait::async_trait]
pub trait LoginUseCase: Send + Sync {
    async fn execute(&self, req: LoginRequest) -> Result<AuthResponse, AppError>;
}
