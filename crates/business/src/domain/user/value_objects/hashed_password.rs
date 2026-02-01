use crate::domain::user::error::DomainError;

#[derive(Clone)]
pub struct HashedPassword(String);

impl HashedPassword {
    pub fn new(value: String) -> Result<Self, DomainError> {
        if value.is_empty() {
            return Err(DomainError::Unexpected(
                "Hashed password does not meet domain requirements".into(),
            ));
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
