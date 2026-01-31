use zxcvbn::{Score, zxcvbn};

use crate::domain::user::error::DomainError;

pub struct Password(String);

impl Password {
    pub fn new(value: String) -> Result<Self, DomainError> {
        if zxcvbn(&value, &[]).score() < Score::Three {
            return Err(DomainError::PasswordTooWeak);
        }

        Ok(Self(value))
    }
}
