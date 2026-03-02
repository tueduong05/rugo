use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct GetLinkRequest {
    #[validate(length(min = 3, message = "Password must be at least 3 characters"))]
    pub password: Option<String>,
}
