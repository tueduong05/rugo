use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use business::{application::error::AppError, domain::user::error::UserDomainError};
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

            // TODO: Do not leak sensitive errors to user
            AppError::User(user_err) => {
                let status = match user_err {
                    UserDomainError::UsernameTaken | UserDomainError::EmailTaken => {
                        StatusCode::CONFLICT
                    }
                    UserDomainError::InvalidCredentials => StatusCode::UNAUTHORIZED,
                    _ => StatusCode::BAD_REQUEST,
                };

                (status, json!({ "error": user_err.to_string() }))
            }

            AppError::Link(_link_err) => {
                todo!()
            }
        };

        (status, Json(body)).into_response()
    }
}
