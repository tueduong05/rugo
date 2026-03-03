use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use business::{
    application::error::AppError,
    domain::{
        common::error::BaseDomainError, link::error::LinkDomainError, user::error::UserDomainError,
    },
};
use serde::Serialize;
use serde_json::json;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ProblemDetails {
    pub r#type: String,
    pub status: u16,
    pub title: String,
    pub detail: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<serde_json::Value>,
}

impl ProblemDetails {
    fn new(status: StatusCode, title: &str, detail: String) -> Self {
        Self {
            r#type: "about:blank".to_string(),
            status: status.as_u16(),
            title: title.to_string(),
            detail,
            instance: None,
            errors: None,
        }
    }

    fn with_type(mut self, r#type: &str) -> Self {
        self.r#type = r#type.to_string();
        self
    }

    fn as_response(self) -> (StatusCode, Json<Self>) {
        (
            StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            Json(self),
        )
    }
}

fn map_base_error(base: BaseDomainError) -> (StatusCode, Json<ProblemDetails>) {
    match base {
        BaseDomainError::ResourceNotFound(resource) => match resource.as_str() {
            "User" | "Session" => ProblemDetails::new(
                StatusCode::UNAUTHORIZED,
                "Unauthorized",
                "Invalid identity or insufficient permissions".into(),
            )
            .as_response(),

            _ => ProblemDetails::new(
                StatusCode::NOT_FOUND,
                "Not Found",
                format!("{} not found", resource),
            )
            .as_response(),
        },

        BaseDomainError::ConcurrencyError => ProblemDetails::new(
            StatusCode::CONFLICT,
            "Edit Conflict",
            "Data was modified by another request".into(),
        )
        .as_response(),

        _ => ProblemDetails::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error",
            "An unexpected error occurred".into(),
        )
        .as_response(),
    }
}

#[derive(ToSchema)]
pub struct HttpError(AppError);

impl From<AppError> for HttpError {
    fn from(error: AppError) -> Self {
        Self(error)
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        match self.0 {
            AppError::Validation(val_errors) => {
                let status = StatusCode::BAD_REQUEST;
                let details: Vec<_> = val_errors
                    .details
                    .into_iter()
                    .map(|d| json!({ "field": d.field, "message": d.message }))
                    .collect();

                (
                    status,
                    Json(ProblemDetails {
                        r#type: "about:blank".to_string(),
                        status: status.as_u16(),
                        title: "Validation Failed".into(),
                        detail: "One or more fields failed validation".into(),
                        instance: None,
                        errors: Some(json!(details)),
                    }),
                )
                    .into_response()
            }

            AppError::User(user_err) => match user_err {
                UserDomainError::Base(base) => map_base_error(base).into_response(),

                UserDomainError::UsernameTaken | UserDomainError::EmailTaken => {
                    ProblemDetails::new(StatusCode::CONFLICT, "Conflict", user_err.to_string())
                        .as_response()
                        .into_response()
                }

                UserDomainError::InvalidCredentials | UserDomainError::InvalidSession => {
                    ProblemDetails::new(
                        StatusCode::UNAUTHORIZED,
                        "Unauthorized",
                        "Authentication failed".into(),
                    )
                    .as_response()
                    .into_response()
                }

                UserDomainError::AccessDenied
                | UserDomainError::SessionExpired
                | UserDomainError::SessionAlreadyUsed
                | UserDomainError::SessionRevoked
                | UserDomainError::EmailNotVerified
                | UserDomainError::AccountLocked
                | UserDomainError::AccountDisabled => {
                    ProblemDetails::new(StatusCode::FORBIDDEN, "Forbidden", user_err.to_string())
                        .as_response()
                        .into_response()
                }

                UserDomainError::PasswordTooWeak => ProblemDetails::new(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    "Unprocessable Entity",
                    "The password does not meet the complexity requirements.".into(),
                )
                .as_response()
                .into_response(),
            },

            AppError::Link(link_err) => match link_err {
                LinkDomainError::Base(base) => map_base_error(base).into_response(),

                LinkDomainError::LinkExpired => {
                    ProblemDetails::new(StatusCode::GONE, "Link Expired", link_err.to_string())
                        .as_response()
                        .into_response()
                }

                LinkDomainError::ShortCodeAlreadyExists => {
                    ProblemDetails::new(StatusCode::CONFLICT, "Conflict", link_err.to_string())
                        .as_response()
                        .into_response()
                }

                LinkDomainError::PasswordRequired => ProblemDetails::new(
                    StatusCode::UNAUTHORIZED,
                    "Password Required",
                    "This link is protected. Please provide a password.".into(),
                )
                .with_type("about:blank/password-required")
                .as_response()
                .into_response(),

                LinkDomainError::WrongPassword => ProblemDetails::new(
                    StatusCode::UNAUTHORIZED,
                    "Unauthorized",
                    "The password provided is incorrect.".into(),
                )
                .with_type("about:blank/invalid-password")
                .as_response()
                .into_response(),

                LinkDomainError::LinkClickLimitReached | LinkDomainError::LinkNotActive => {
                    ProblemDetails::new(StatusCode::FORBIDDEN, "Forbidden", link_err.to_string())
                        .as_response()
                        .into_response()
                }

                LinkDomainError::InvalidLink => ProblemDetails::new(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    "Unprocessable Entity",
                    link_err.to_string(),
                )
                .as_response()
                .into_response(),

                LinkDomainError::ShortCodeCollisionLimitReached => ProblemDetails::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error",
                    "Could not generate a unique short code".into(),
                )
                .as_response()
                .into_response(),
            },
        }
    }
}
