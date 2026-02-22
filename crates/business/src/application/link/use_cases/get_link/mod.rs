use crate::{
    application::{error::AppError, link::use_cases::get_link::request::GetLinkRequest},
    domain::link::value_objects::{original_link::OriginalLink},
};

pub mod interactor;
pub mod request;

#[async_trait::async_trait]
pub trait GetLinkUseCase: Send + Sync {
    async fn execute(
        &self,
        short_code: String,
        req: GetLinkRequest,
    ) -> Result<OriginalLink, AppError>;
}
