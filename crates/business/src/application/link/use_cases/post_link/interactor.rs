use std::sync::Arc;

use crate::{
    application::link::{
        error::AppError,
        use_cases::post_link::{
            PostLinkUseCase,
            dtos::{PostLinkRequest, PostLinkResponse},
        },
    },
    domain::{
        link::{repositories::LinkRepository},
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
        todo!()
    }
}
