use std::sync::Arc;

use chrono::Utc;

use crate::{
    application::{
        error::AppError,
        link::use_cases::get_link::{GetLinkUseCase, dtos::GetLinkCommand},
    },
    domain::{
        common::{error::BaseDomainError, services::password_services::PasswordHasher},
        link::{
            error::LinkDomainError,
            repositories::LinkRepository,
            value_objects::{original_link::OriginalLink, short_code::ShortCode},
        },
        link_analytics::{
            entities::AnalyticsEvent, error::AnalyticsDomainError, services::AnalyticsQueue,
        },
    },
};

pub struct GetLinkInteractor {
    link_repo: Arc<dyn LinkRepository>,
    password_hasher: Arc<dyn PasswordHasher>,
    analytics_queue: Arc<dyn AnalyticsQueue>,
}

impl GetLinkInteractor {
    pub fn new(
        link_repo: Arc<dyn LinkRepository>,
        password_hasher: Arc<dyn PasswordHasher>,
        analytics_queue: Arc<dyn AnalyticsQueue>,
    ) -> Self {
        Self {
            link_repo,
            password_hasher,
            analytics_queue,
        }
    }
}

#[async_trait::async_trait]
impl GetLinkUseCase for GetLinkInteractor {
    async fn execute(&self, cmd: GetLinkCommand) -> Result<OriginalLink, AppError> {
        let short_code = ShortCode::new(cmd.short_code)?;

        let link = self.link_repo.find_by_short_code(&short_code).await?;

        // TODO: Get current clicks count when analytics is implemented
        link.is_valid(Utc::now(), 0)?;

        let original_link = match (cmd.password, &link.hashed_password) {
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

        let event = AnalyticsEvent {
            link_id: link.id.ok_or_else(|| {
                AnalyticsDomainError::from(BaseDomainError::Unexpected(
                    "LinkId should be available".into(),
                ))
            })?,
            referrer: cmd.referrer,
            user_agent: cmd.user_agent,
            ip: cmd.ip,
            timestamp: Utc::now(),
        };

        let _ = self.analytics_queue.push(event).await;

        Ok(original_link)
    }
}
