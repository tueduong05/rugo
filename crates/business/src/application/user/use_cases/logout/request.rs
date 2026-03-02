use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct LogoutRequest {
    #[validate(length(min = 1))]
    pub refresh_token: String,
}
