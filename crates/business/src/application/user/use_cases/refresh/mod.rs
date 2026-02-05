use crate::application::user::{
    error::AppError,
    use_cases::refresh::dtos::{RefreshSessionRequest, RefreshSessionResponse},
};

pub mod dtos;
pub mod interactor;

#[async_trait::async_trait]
pub trait RefreshSessionUseCase: Send + Sync {
    async fn execute(&self, req: RefreshSessionRequest)
    -> Result<RefreshSessionResponse, AppError>;
}
