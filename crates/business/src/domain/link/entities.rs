use chrono::{DateTime, Utc};

use crate::domain::{
    link::value_objects::{original_link::OriginalLink, short_code::ShortCode},
    user::value_objects::user_id::UserId,
};

pub struct Link {
    pub id: u64,
    pub user_id: Option<UserId>,
    pub original_link: OriginalLink,
    pub short_code: ShortCode,
    pub is_custom: bool,
    pub expires_at: Option<DateTime<Utc>>,
    pub hashed_password: Option<String>,
    pub max_clicks: Option<u32>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Link {
    pub fn new(
        id: u64,
        user_id: Option<UserId>,
        original_link: OriginalLink,
        short_code: ShortCode,
        is_custom: bool,
        expires_at: Option<DateTime<Utc>>,
        hashed_password: Option<String>,
        max_clicks: Option<u32>,
        is_active: bool,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Link {
            id,
            user_id,
            original_link,
            short_code,
            is_custom,
            expires_at,
            hashed_password,
            max_clicks,
            is_active,
            created_at,
            updated_at,
        }
    }
}
