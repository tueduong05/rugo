use crate::domain::link::error::DomainError;

pub struct ShortCode(String);

impl ShortCode {
    pub fn new(value: String) -> Result<Self, DomainError> {
        if value != value.trim()
            || !(3..=20).contains(&value.len())
            || !value.chars().all(|c| c.is_ascii_alphanumeric())
        {
            return Err(DomainError::Unexpected(
                "Short code does not meet domain requirements".into(),
            ));
        }

        Ok(Self(value))
    }
}
