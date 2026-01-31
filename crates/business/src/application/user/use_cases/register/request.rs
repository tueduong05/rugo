use serde::Deserialize;
use serde_trim::string_trim;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct RegisterRequest {
    #[serde(deserialize_with = "string_trim")]
    #[validate(
        length(min = 3, max = 20, message = "Username must be between 3 and 20 characters"),
        regex(path = *crate::domain::user::value_objects::username::USERNAME_REGEX, message = "Username contains invalid characters")
    )]
    pub username: String,

    #[serde(deserialize_with = "string_trim")]
    #[validate(
        length(max = 256, message = "Email is too long"),
        regex(path = *crate::domain::user::value_objects::email::EMAIL_REGEX, message = "Invalid email format")
    )]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}
