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
    pub expires_at: Option<u64>,
    pub hashed_password: Option<String>,
    pub max_clicks: Option<u32>,
    pub is_active: bool,
}

impl Link {
    pub fn new(
        id: u64,
        user_id: Option<UserId>,
        original_link: OriginalLink,
        short_code: ShortCode,
        is_custom: bool,
        expires_at: Option<u64>,
        hashed_password: Option<String>,
        max_clicks: Option<u32>,
        is_active: bool,
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
        }
    }
}
