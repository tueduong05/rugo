use crate::domain::link::error::DomainError;

#[derive(Debug)]
pub struct ValidationErrorDetail {
    pub field: String,
    pub message: String,
    pub code: String,
}

#[derive(Debug, Default, thiserror::Error)]
#[error("{details:?}")]
pub struct ValidationErrors {
    pub details: Vec<ValidationErrorDetail>,
}

impl ValidationErrors {
    pub fn new() -> Self {
        Self {
            details: Vec::new(),
        }
    }

    pub fn add(&mut self, field: &str, message: &str, code: &str) {
        self.details.push(ValidationErrorDetail {
            field: field.to_string(),
            message: message.to_string(),
            code: code.to_string(),
        });
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    Validation(ValidationErrors),

    #[error(transparent)]
    Domain(#[from] DomainError),
}

impl From<validator::ValidationErrors> for AppError {
    fn from(errors: validator::ValidationErrors) -> Self {
        let mut validation_errors = ValidationErrors::new();

        for (field, field_errors) in errors.field_errors() {
            for error in field_errors {
                let code = match error.code.as_ref() {
                    "length" => "INVALID_LENGTH",
                    "regex" | "url" => "INVALID_FORMAT",
                    _ => "VALIDATION_ERROR",
                };

                let message = error
                    .message
                    .as_deref()
                    .unwrap_or("Unknown validation error");

                validation_errors.add(field.as_ref(), message, code);
            }
        }

        AppError::Validation(validation_errors)
    }
}
