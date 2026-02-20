use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct LogoutRequest {
    #[validate(length(min = 1))]
    pub refresh_token: String,
}
