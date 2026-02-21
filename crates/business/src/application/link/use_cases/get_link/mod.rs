use crate::{
    application::{error::AppError, link::use_cases::get_link::request::GetLinkRequest},
    domain::link::value_objects::{original_link::OriginalLink, short_code::ShortCode},
};

pub mod interactor;
pub mod request;

#[async_trait::async_trait]
pub trait GetLinkUseCase: Send + Sync {
    async fn execute(
        &self,
        short_code: ShortCode,
        req: GetLinkRequest,
    ) -> Result<OriginalLink, AppError>;
}
