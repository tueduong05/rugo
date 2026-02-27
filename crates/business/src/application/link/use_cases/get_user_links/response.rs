use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct GetUserLinkItem {
    pub original_link: String,
    pub short_code: String,
    pub is_custom: bool,
    pub expires_at: Option<DateTime<Utc>>,
    pub max_clicks: Option<u32>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct GetUserLinksResponse {
    pub links: Vec<GetUserLinkItem>,
    pub total_count: usize,
}
