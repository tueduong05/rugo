use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_trim::{option_string_trim, string_trim};
use validator::Validate;

use crate::domain::link::value_objects::short_code::SHORTCODE_REGEX;

#[derive(Deserialize, Validate)]
pub struct PostLinkRequest {
    #[serde(deserialize_with = "string_trim")]
    #[validate(
        length(max = 4096, message = "Link is too long"),
        url(message = "Invalid link format")
    )]
    pub original_link: String,

    #[serde(deserialize_with = "option_string_trim")]
    #[validate(
        length(min = 3, max = 20, message = "Short code must be between 3 and 20 characters"),
        regex(path = SHORTCODE_REGEX, message = "Short code contains invalid characters")
    )]
    pub short_code: Option<String>,

    pub expires_at: Option<u64>,
    pub password: Option<String>,
    pub max_clicks: Option<u32>,
}

#[derive(Serialize)]
pub struct PostLinkResponse {
    pub id: u64,
    pub original_link: String,
    pub short_code: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub max_clicks: Option<u32>,
    pub created_at: DateTime<Utc>,
}
