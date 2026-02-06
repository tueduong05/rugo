use crate::application::user::{error::AppError, use_cases::logout::dtos::LogoutCommand};

pub mod dtos;
pub mod interactor;

#[async_trait::async_trait]
pub trait LogoutUseCase: Send + Sync {
    async fn execute(&self, req: LogoutCommand) -> Result<(), AppError>;
}
