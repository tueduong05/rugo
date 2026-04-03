use chrono::{DateTime, Utc};

use crate::domain::{
    common::value_objects::{
        hashed_password::HashedPassword, original_link::OriginalLink, user_id::UserId,
    },
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
    pub current_clicks: u32,
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
        let now = Utc::now();

        Self {
            id: None,
            user_id: cmd.user_id,
            original_link: cmd.original_link,
            short_code: cmd.short_code,
            is_custom: cmd.is_custom,
            expires_at: cmd.expires_at,
            hashed_password: cmd.hashed_password,
            current_clicks: 0,
            max_clicks: cmd.max_clicks,
            is_active: cmd.is_active,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn is_active(&self) -> Result<(), LinkDomainError> {
        if !self.is_active {
            return Err(LinkDomainError::LinkNotActive);
        }
        Ok(())
    }

    pub fn is_not_expired(&self, current_time: DateTime<Utc>) -> Result<(), LinkDomainError> {
        if self.expires_at.is_some_and(|expiry| current_time > expiry) {
            return Err(LinkDomainError::LinkExpired);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use super::*;

    #[test]
    fn test_link_is_active() {
        let link = Link::new(CreateLinkCommand {
            user_id: Some(UserId::generate()),
            original_link: OriginalLink::new("https://example.com".to_string())
                .expect("Valid original link"),
            short_code: ShortCode::new("example_123".to_string()).expect("Valid short code"),
            is_custom: false,
            expires_at: None,
            hashed_password: None,
            max_clicks: None,
            is_active: true,
        });

        assert!(link.is_active().is_ok());
    }

    #[test]
    fn test_link_invalid_when_inactive() {
        let link = Link::new(CreateLinkCommand {
            user_id: Some(UserId::generate()),
            original_link: OriginalLink::new("https://example.com".to_string())
                .expect("Valid original link"),
            short_code: ShortCode::new("example_123".to_string()).expect("Valid short code"),
            is_custom: false,
            expires_at: None,
            hashed_password: None,
            max_clicks: None,
            is_active: false,
        });

        assert!(matches!(
            link.is_active(),
            Err(LinkDomainError::LinkNotActive)
        ));
    }

    #[test]
    fn test_link_not_expired_without_expiry() {
        let now = Utc::now();
        let link = Link::new(CreateLinkCommand {
            user_id: Some(UserId::generate()),
            original_link: OriginalLink::new("https://example.com".to_string())
                .expect("Valid original link"),
            short_code: ShortCode::new("example_123".to_string()).expect("Valid short code"),
            is_custom: false,
            expires_at: None,
            hashed_password: None,
            max_clicks: None,
            is_active: true,
        });

        assert!(link.is_not_expired(now).is_ok());
    }

    #[test]
    fn test_link_not_expired_when_expiry_in_future() {
        let now = Utc::now();
        let link = Link::new(CreateLinkCommand {
            user_id: Some(UserId::generate()),
            original_link: OriginalLink::new("https://example.com".to_string())
                .expect("Valid original link"),
            short_code: ShortCode::new("example_123".to_string()).expect("Valid short code"),
            is_custom: false,
            expires_at: Some(now + Duration::minutes(5)),
            hashed_password: None,
            max_clicks: None,
            is_active: true,
        });

        assert!(link.is_not_expired(now).is_ok());
    }

    #[test]
    fn test_link_invalid_when_expired() {
        let now = Utc::now();
        let link = Link::new(CreateLinkCommand {
            user_id: Some(UserId::generate()),
            original_link: OriginalLink::new("https://example.com".to_string())
                .expect("Valid original link"),
            short_code: ShortCode::new("example_123".to_string()).expect("Valid short code"),
            is_custom: false,
            expires_at: Some(now - Duration::minutes(1)),
            hashed_password: None,
            max_clicks: None,
            is_active: true,
        });

        assert!(matches!(
            link.is_not_expired(now),
            Err(LinkDomainError::LinkExpired)
        ));
    }

    #[test]
    fn test_link_not_expired_when_expiry_equals_now() {
        let now = Utc::now();
        let link = Link::new(CreateLinkCommand {
            user_id: Some(UserId::generate()),
            original_link: OriginalLink::new("https://example.com".to_string())
                .expect("Valid original link"),
            short_code: ShortCode::new("example_123".to_string()).expect("Valid short code"),
            is_custom: false,
            expires_at: Some(now),
            hashed_password: None,
            max_clicks: None,
            is_active: true,
        });

        assert!(link.is_not_expired(now).is_ok());
    }
}
