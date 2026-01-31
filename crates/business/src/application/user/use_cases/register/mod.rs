use crate::application::{
    error::AppError,
    user::{dtos::auth_response::AuthResponse, use_cases::register::request::RegisterRequest},
};

mod interactor;
mod request;

trait RegisterUseCase {
    async fn execute(&self, req: RegisterRequest) -> Result<AuthResponse, AppError>;
}
