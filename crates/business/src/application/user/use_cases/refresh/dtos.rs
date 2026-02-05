use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct RefreshSessionRequest {
    #[validate(length(min = 1))]
    pub refresh_token: String,
}

#[derive(Serialize)]
pub struct RefreshSessionResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_token: String,
}
