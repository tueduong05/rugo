use crate::{
    application::{error::AppError, link::use_cases::get_link::dtos::GetLinkCommand},
    domain::link::value_objects::original_link::OriginalLink,
};

pub mod dtos;
pub mod interactor;

#[async_trait::async_trait]
pub trait GetLinkUseCase: Send + Sync {
    async fn execute(&self, cmd: GetLinkCommand) -> Result<OriginalLink, AppError>;
}
