use business::domain::user::{
    entities::{RefreshToken, User},
    error::DomainError,
    value_objects::{
        email::Email, hashed_password::HashedPassword, user_id::UserId, user_status::UserStatus,
        username::Username,
    },
};
use chrono::{DateTime, Utc};
use sqlx::{prelude::Type, types::Uuid};

#[derive(Type)]
#[sqlx(type_name = "user_status", rename_all = "lowercase")]
pub enum DbUserStatus {
    Unverified,
    Verified,
    Locked,
    Disabled,
}

impl From<&UserStatus> for DbUserStatus {
    fn from(status: &UserStatus) -> Self {
        match status {
            UserStatus::Unverified => Self::Unverified,
            UserStatus::Verified => Self::Verified,
            UserStatus::Locked => Self::Locked,
            UserStatus::Disabled => Self::Disabled,
        }
    }
}

impl From<DbUserStatus> for UserStatus {
    fn from(db_status: DbUserStatus) -> Self {
        match db_status {
            DbUserStatus::Unverified => Self::Unverified,
            DbUserStatus::Verified => Self::Verified,
            DbUserStatus::Locked => Self::Locked,
            DbUserStatus::Disabled => Self::Disabled,
        }
    }
}

#[derive(sqlx::FromRow)]
pub struct UserRecord {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub hashed_password: String,
    pub status: DbUserStatus,
    pub created_at: DateTime<Utc>,
}

impl From<&User> for UserRecord {
    fn from(user: &User) -> Self {
        Self {
            id: user.id.value(),
            username: user.username.to_string(),
            email: user.email.to_string(),
            hashed_password: user.hashed_password.to_string(),
            status: DbUserStatus::from(&user.status),
            created_at: user.created_at,
        }
    }
}

impl UserRecord {
    pub fn try_into_domain(self) -> Result<User, DomainError> {
        Ok(User {
            id: UserId::from(self.id),
            username: Username::new(self.username)?,
            email: Email::new(self.email)?,
            hashed_password: HashedPassword::new(self.hashed_password)?,
            status: UserStatus::from(self.status),
            created_at: self.created_at,
        })
    }
}

#[derive(sqlx::FromRow)]
pub struct RefreshTokenRecord {
    pub id: u64,
    pub user_id: Uuid,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub is_used: bool,
    pub is_revoked: bool,
    pub version: u64,
}

impl From<&RefreshToken> for RefreshTokenRecord {
    fn from(refresh_token: &RefreshToken) -> Self {
        Self {
            id: refresh_token.id,
            user_id: refresh_token.user_id.value(),
            token: refresh_token.token.clone(),
            expires_at: refresh_token.expires_at,
            is_used: refresh_token.is_used,
            is_revoked: refresh_token.is_revoked,
            version: refresh_token.version,
        }
    }
}
