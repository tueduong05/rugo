use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use business::application::user::error::AppError;
use serde_json::json;

pub struct HttpError(AppError);

impl From<AppError> for HttpError {
    fn from(error: AppError) -> Self {
        Self(error)
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        let (status, body) = match self.0 {
            AppError::Validation(val_errors) => {
                let errors: Vec<_> = val_errors
                    .details
                    .into_iter()
                    .map(|d| {
                        json!({
                            "field": d.field,
                            "message": d.message,
                            "code": d.code
                        })
                    })
                    .collect();

                (StatusCode::BAD_REQUEST, json!({ "errors": errors }))
            }

            AppError::Domain(domain_err) => {
                let status = match domain_err {
                    business::domain::user::error::DomainError::UsernameTaken
                    | business::domain::user::error::DomainError::EmailTaken => {
                        StatusCode::CONFLICT
                    }
                    business::domain::user::error::DomainError::InvalidCredentials => {
                        StatusCode::UNAUTHORIZED
                    }
                    _ => StatusCode::BAD_REQUEST,
                };

                (status, json!({ "error": domain_err.to_string() }))
            }

            AppError::Technical(_msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "error": "An unexpected error occurred" }),
            ),
        };

        (status, Json(body)).into_response()
    }
}
