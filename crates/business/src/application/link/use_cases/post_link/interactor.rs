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
        common::{
            services::password_services::PasswordHasher,
            value_objects::hashed_password::HashedPassword,
        },
        link::{
            entities::Link,
            error::LinkDomainError,
            repositories::LinkRepository,
            services::short_code_services::ShortCodeGenerator,
            value_objects::{original_link::OriginalLink, short_code::ShortCode},
        },
        user::value_objects::user_id::UserId,
    },
};

pub struct PostLinkInteractor {
    link_repo: Arc<dyn LinkRepository>,
    short_code_generator: Arc<dyn ShortCodeGenerator>,
    password_hasher: Arc<dyn PasswordHasher>,
}

impl PostLinkInteractor {
    pub fn new(
        link_repo: Arc<dyn LinkRepository>,
        short_code_generator: Arc<dyn ShortCodeGenerator>,
        password_hasher: Arc<dyn PasswordHasher>,
    ) -> Self {
        Self {
            link_repo,
            short_code_generator,
            password_hasher,
        }
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
        let is_custom = req.short_code.is_some();

        let mut retries = 0;
        const MAX_RETRIES: u32 = 5;

        let link = loop {
            let short_code = match &req.short_code {
                Some(value) => ShortCode::new(value.clone())?,
                None => {
                    let code = self.short_code_generator.generate();
                    ShortCode::new(code)?
                }
            };

            let password_hash = match &req.password {
                None => None,
                Some(p) => {
                    let hashed = self.password_hasher.hash(p);
                    Some(HashedPassword::new(hashed).map_err(LinkDomainError::Base)?)
                }
            };

            let new_link = Link::new(
                user_id,
                original_link.clone(),
                short_code,
                is_custom,
                req.expires_at,
                password_hash,
                req.max_clicks,
                req.is_active,
            );

            match self.link_repo.save(&new_link).await {
                Ok(_) => break new_link,

                Err(LinkDomainError::ShortCodeAlreadyExists) if !is_custom => {
                    retries += 1;
                    if retries >= MAX_RETRIES {
                        return Err(LinkDomainError::ShortCodeCollisionLimitReached.into());
                    }
                    continue;
                }

                Err(e) => return Err(e.into()),
            }
        };

        Ok(PostLinkResponse {
            id: link.id,
            original_link: link.original_link.to_string(),
            short_code: link.short_code.to_string(),
            expires_at: link.expires_at,
            max_clicks: link.max_clicks,
            is_active: link.is_active,
            created_at: link.created_at,
        })
    }
}
