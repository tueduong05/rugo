use business::domain::{
    common::value_objects::{
        hashed_password::HashedPassword, original_link::OriginalLink, user_id::UserId,
    },
    link::{entities::Link, error::LinkDomainError, value_objects::short_code::ShortCode},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Uuid};

#[derive(Serialize, Deserialize, FromRow)]
pub struct LinkRecord {
    pub id: Option<i64>,
    pub user_id: Option<Uuid>,
    pub original_link: String,
    pub short_code: String,
    pub is_custom: bool,
    pub expires_at: Option<DateTime<Utc>>,
    pub hashed_password: Option<String>,
    pub current_clicks: i32,
    pub max_clicks: Option<i32>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<&Link> for LinkRecord {
    fn from(link: &Link) -> Self {
        Self {
            id: link.id.map(|id| id as i64),
            user_id: link.user_id.as_ref().map(|id| id.value()),
            original_link: link.original_link.to_string(),
            short_code: link.short_code.to_string(),
            is_custom: link.is_custom,
            expires_at: link.expires_at,
            hashed_password: link.hashed_password.as_ref().map(|p| p.to_string()),
            current_clicks: link.current_clicks as i32,
            max_clicks: link.max_clicks.map(|c| c as i32),
            is_active: link.is_active,
            created_at: link.created_at,
            updated_at: link.updated_at,
        }
    }
}

impl LinkRecord {
    pub fn try_into_domain(self) -> Result<Link, LinkDomainError> {
        Ok(Link {
            id: self.id.map(|id| id as u64),
            user_id: self.user_id.map(UserId::from),
            original_link: OriginalLink::new(self.original_link)?,
            short_code: ShortCode::new(self.short_code)?,
            is_custom: self.is_custom,
            expires_at: self.expires_at,
            hashed_password: self.hashed_password.map(HashedPassword::new).transpose()?,
            current_clicks: self.current_clicks as u32,
            max_clicks: self.max_clicks.map(|c| c as u32),
            is_active: self.is_active,
            created_at: self.created_at,
            updated_at: self.updated_at,
        })
    }
}
