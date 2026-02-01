use crate::application::user::{
    dtos::auth_response::AuthResponse, error::AppError,
    use_cases::register::request::RegisterRequest,
};

pub mod interactor;
pub mod request;

#[async_trait::async_trait]
pub trait RegisterUseCase: Send + Sync {
    async fn execute(&self, req: RegisterRequest) -> Result<AuthResponse, AppError>;
}
