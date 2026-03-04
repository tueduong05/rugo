use std::net::IpAddr;

use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct GetLinkRequest {
    #[validate(length(min = 3, message = "Password must be at least 3 characters"))]
    pub password: Option<String>,
}

pub struct GetLinkCommand {
    pub short_code: String,
    pub password: Option<String>,
    pub referrer: Option<String>,
    pub user_agent: Option<String>,
    pub ip: IpAddr,
}
