use chrono::{DateTime, Utc};

use crate::domain::{
    common::value_objects::{hashed_password::HashedPassword, original_link::OriginalLink, user_id::UserId},
    link::{error::LinkDomainError, value_objects::short_code::ShortCode},
};

#[derive(Clone)]
pub struct Link {
    pub id: Option<u64>,
    pub user_id: Option<UserId>,
    pub original_link: OriginalLink,
    pub short_code: ShortCode,
    pub is_custom: bool,
    pub expires_at: Option<DateTime<Utc>>,
    pub hashed_password: Option<HashedPassword>,
    pub max_clicks: Option<u32>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct CreateLinkCommand {
    pub user_id: Option<UserId>,
    pub original_link: OriginalLink,
    pub short_code: ShortCode,
    pub is_custom: bool,
    pub expires_at: Option<DateTime<Utc>>,
    pub hashed_password: Option<HashedPassword>,
    pub max_clicks: Option<u32>,
    pub is_active: bool,
}

impl Link {
    pub fn new(cmd: CreateLinkCommand) -> Self {
        Self {
            id: None,
            user_id: cmd.user_id,
            original_link: cmd.original_link,
            short_code: cmd.short_code,
            is_custom: cmd.is_custom,
            expires_at: cmd.expires_at,
            hashed_password: cmd.hashed_password,
            max_clicks: cmd.max_clicks,
            is_active: cmd.is_active,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn is_valid(
        &self,
        current_time: DateTime<Utc>,
        current_clicks: u32,
    ) -> Result<(), LinkDomainError> {
        if !self.is_active {
            return Err(LinkDomainError::LinkNotActive);
        }

        if self.expires_at.is_some_and(|expiry| current_time > expiry) {
            return Err(LinkDomainError::LinkExpired);
        }

        if self.max_clicks.is_some_and(|max| current_clicks >= max) {
            return Err(LinkDomainError::LinkClickLimitReached);
        }

        Ok(())
    }
}
