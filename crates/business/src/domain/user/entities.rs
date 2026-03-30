use chrono::{DateTime, Utc};

use crate::domain::{
    common::value_objects::{hashed_password::HashedPassword, user_id::UserId},
    user::value_objects::{email::Email, user_status::UserStatus, username::Username},
};

#[derive(Clone)]
pub struct User {
    pub id: UserId,
    pub username: Username,
    pub email: Email,
    pub hashed_password: HashedPassword,
    pub status: UserStatus,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn new(
        id: UserId,
        username: Username,
        email: Email,
        hashed_password: HashedPassword,
        status: UserStatus,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            username,
            email,
            hashed_password,
            status,
            created_at,
        }
    }
}

#[derive(Clone)]
pub struct RefreshToken {
    pub id: Option<u64>,
    pub user_id: UserId,
    pub token: Option<String>,
    pub expires_at: DateTime<Utc>,
    pub is_used: bool,
    pub is_revoked: bool,
    pub version: u64,
}

impl RefreshToken {
    pub fn is_valid(&self, now: DateTime<Utc>) -> bool {
        !self.is_revoked && !self.is_used && self.expires_at > now
    }

    pub fn mark_used(&mut self) {
        self.is_used = true;
        self.version += 1;
    }
}

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use super::*;

    #[test]
    fn test_refresh_token_is_valid() {
        let now = Utc::now();
        let token = RefreshToken {
            id: Some(1),
            user_id: UserId::generate(),
            token: Some("refresh-token".to_string()),
            expires_at: now + Duration::minutes(10),
            is_used: false,
            is_revoked: false,
            version: 0,
        };

        assert!(token.is_valid(now));
    }

    #[test]
    fn test_refresh_token_invalid_when_used() {
        let now = Utc::now();
        let token = RefreshToken {
            id: Some(1),
            user_id: UserId::generate(),
            token: Some("refresh-token".to_string()),
            expires_at: now + Duration::minutes(10),
            is_used: true,
            is_revoked: false,
            version: 0,
        };

        assert!(!token.is_valid(now));
    }

    #[test]
    fn test_refresh_token_invalid_when_revoked() {
        let now = Utc::now();
        let token = RefreshToken {
            id: Some(1),
            user_id: UserId::generate(),
            token: Some("refresh-token".to_string()),
            expires_at: now + Duration::minutes(10),
            is_used: false,
            is_revoked: true,
            version: 0,
        };

        assert!(!token.is_valid(now));
    }

    #[test]
    fn test_refresh_token_invalid_when_expired() {
        let now = Utc::now();
        let token = RefreshToken {
            id: Some(1),
            user_id: UserId::generate(),
            token: Some("refresh-token".to_string()),
            expires_at: now - Duration::minutes(1),
            is_used: false,
            is_revoked: false,
            version: 0,
        };

        assert!(!token.is_valid(now));
    }

    #[test]
    fn test_refresh_token_mark_used() {
        let now = Utc::now();
        let mut token = RefreshToken {
            id: Some(1),
            user_id: UserId::generate(),
            token: Some("refresh-token".to_string()),
            expires_at: now + Duration::minutes(10),
            is_used: false,
            is_revoked: false,
            version: 3,
        };

        token.mark_used();

        assert!(token.is_used);
        assert_eq!(token.version, 4);
    }
}
