use chrono::{DateTime, Utc};

use crate::domain::user::value_objects::{
    email::Email, hashed_password::HashedPassword, user_id::UserId, user_status::UserStatus,
    username::Username,
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

pub struct RefreshToken {
    pub id: u64,
    pub family_id: u64,
    pub user_id: UserId,
    pub token: String,
    pub expires_at: u64,
    pub is_used: bool,
    pub is_revoked: bool,
}

impl RefreshToken {
    pub fn is_valid(&self, now: u64) -> bool {
        !self.is_revoked && !self.is_used && self.expires_at > now
    }

    pub fn mark_as_used(&mut self) {
        self.is_used = true;
    }

    pub fn revoke(&mut self) {
        self.is_revoked = true;
    }
}
