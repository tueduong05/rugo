use std::borrow::Cow;

use serde::Deserialize;
use serde_trim::string_trim;
use validator::{Validate, ValidationError};

use crate::domain::user::value_objects::{email::EMAIL_REGEX, username::USERNAME_REGEX};

#[derive(Deserialize, Validate)]
pub struct LoginRequest {
    #[serde(deserialize_with = "string_trim")]
    #[validate(length(min = 1), custom(function = "validate_identifier"))]
    pub identifier: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

fn validate_identifier(identifier: &str) -> Result<(), ValidationError> {
    let len = identifier.chars().count();

    #[allow(clippy::collapsible_else_if)]
    let error_data = if identifier.contains('@') {
        if len > 256 {
            Some(("length", "Email is too long"))
        } else if !EMAIL_REGEX.is_match(identifier) {
            Some(("regex", "Invalid email format"))
        } else {
            None
        }
    } else {
        if !(3..=20).contains(&len) {
            Some(("length", "Username must be between 3 and 20 characters"))
        } else if !USERNAME_REGEX.is_match(identifier) {
            Some(("regex", "Username contains invalid characters"))
        } else {
            None
        }
    };

    if let Some((code, msg)) = error_data {
        let mut error = ValidationError::new(code);
        error.message = Some(Cow::from(msg));
        return Err(error);
    }

    Ok(())
}
