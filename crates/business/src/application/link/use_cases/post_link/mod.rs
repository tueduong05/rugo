use crate::{
    application::link::{
        error::AppError,
        use_cases::post_link::dtos::{PostLinkRequest, PostLinkResponse},
    },
    domain::user::value_objects::user_id::UserId,
};

pub mod dtos;
pub mod interactor;

#[async_trait::async_trait]
pub trait PostLinkUseCase: Send + Sync {
    async fn execute(
        &self,
        user_id: Option<UserId>,
        req: PostLinkRequest,
    ) -> Result<PostLinkResponse, AppError>;
}
