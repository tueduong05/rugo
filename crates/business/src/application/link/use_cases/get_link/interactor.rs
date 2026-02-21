use std::sync::Arc;

use chrono::Utc;

use crate::{
    application::{
        error::AppError,
        link::use_cases::get_link::{GetLinkUseCase, request::GetLinkRequest},
    },
    domain::{
        common::services::password_services::PasswordHasher,
        link::{
            error::LinkDomainError,
            repositories::LinkRepository,
            value_objects::{original_link::OriginalLink, short_code::ShortCode},
        },
    },
};

pub struct GetLinkInteractor {
    link_repo: Arc<dyn LinkRepository>,
    password_hasher: Arc<dyn PasswordHasher>,
}

impl GetLinkInteractor {
    pub fn new(
        link_repo: Arc<dyn LinkRepository>,
        password_hasher: Arc<dyn PasswordHasher>,
    ) -> Self {
        Self {
            link_repo,
            password_hasher,
        }
    }
}

#[async_trait::async_trait]
impl GetLinkUseCase for GetLinkInteractor {
    async fn execute(
        &self,
        short_code: ShortCode,
        req: GetLinkRequest,
    ) -> Result<OriginalLink, AppError> {
        let link = self.link_repo.find_by_short_code(&short_code).await?;

        // TODO: Get current clicks count when analytics is implemented
        link.is_valid(Utc::now(), 0)?;

        let original_link = match (req.password, &link.hashed_password) {
            (Some(p), Some(hashed)) => {
                if self.password_hasher.verify(&p, hashed) {
                    link.original_link
                } else {
                    return Err(LinkDomainError::WrongPassword.into());
                }
            }

            (None, Some(_)) => return Err(LinkDomainError::PasswordRequired.into()),

            (_, None) => link.original_link,
        };

        Ok(original_link)
    }
}
