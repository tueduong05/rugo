use std::sync::Arc;

use crate::{
    application::{
        error::AppError,
        link::use_cases::post_link::{
            PostLinkUseCase,
            dtos::{PostLinkRequest, PostLinkResponse},
        },
    },
    domain::{
        link::{
            repositories::LinkRepository,
            value_objects::{original_link::OriginalLink, short_code::ShortCode},
        },
        user::value_objects::user_id::UserId,
    },
};

pub struct PostLinkInteractor {
    link_repo: Arc<dyn LinkRepository>,
}

impl PostLinkInteractor {
    pub fn new(link_repo: Arc<dyn LinkRepository>) -> Self {
        Self { link_repo }
    }
}

#[async_trait::async_trait]
impl PostLinkUseCase for PostLinkInteractor {
    async fn execute(
        &self,
        user_id: Option<UserId>,
        req: PostLinkRequest,
    ) -> Result<PostLinkResponse, AppError> {
        let original_link = OriginalLink::new(req.original_link)?;
        let short_code = match req.short_code {
            None => todo!("Random short code"),
            Some(value) => ShortCode::new(value)?,
        };

        todo!()
    }
}
