use axum::{Json, extract::State, http::StatusCode};
use business::application::{
    error::AppError,
    user::{
        common::{auth_response::AuthResponse, user_profile_response::UserProfileResponse},
        use_cases::{
            login::request::LoginRequest,
            logout::request::LogoutRequest,
            refresh::dtos::{RefreshSessionRequest, RefreshSessionResponse},
            register::request::RegisterRequest,
        },
    },
};
use validator::Validate;

use crate::{
    common::middleware::AuthenticatedUser,
    error::{HttpError, ProblemDetails},
    user::UserState,
};

#[utoipa::path(
    post,
    path = "/api/v1/users/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User successfully registered", body = AuthResponse),
        (status = 400, description = "Invalid input data (Validation failure)", body = ProblemDetails),
        (status = 409, description = "Username or Email already exists", body = ProblemDetails),
        (status = 422, description = "Password does not meet security requirements", body = ProblemDetails)
    ),
    tag = "Users"
)]
pub async fn register_handler(
    State(state): State<UserState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), HttpError> {
    payload.validate().map_err(AppError::from)?;

    let response = state.register_interactor.execute(payload).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

#[utoipa::path(
    post,
    path = "/api/v1/users/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 401, description = "Invalid credentials or session", body = ProblemDetails),
        (status = 403, description = "Account locked, disabled, or email not verified", body = ProblemDetails),
    ),
    tag = "Users"
)]
pub async fn login_handler(
    State(state): State<UserState>,
    Json(payload): Json<LoginRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), HttpError> {
    payload.validate().map_err(AppError::from)?;

    let response = state.login_interactor.execute(payload).await?;
    Ok((StatusCode::OK, Json(response)))
}

#[utoipa::path(
    post,
    path = "/api/v1/users/refresh",
    request_body = RefreshSessionRequest,
    responses(
        (status = 200, description = "Session refreshed", body = RefreshSessionResponse),
        (status = 400, description = "Invalid request format", body = ProblemDetails),
        (status = 401, description = "Invalid credentials or session", body = ProblemDetails),
        (status = 403, description = "Forbidden - Token reuse detected or session revoked", body = ProblemDetails),
    ),
    tag = "Users"
)]
pub async fn refresh_session_handler(
    State(state): State<UserState>,
    Json(payload): Json<RefreshSessionRequest>,
) -> Result<(StatusCode, Json<RefreshSessionResponse>), HttpError> {
    payload.validate().map_err(AppError::from)?;

    let response = state.refresh_session_interactor.execute(payload).await?;
    Ok((StatusCode::OK, Json(response)))
}

#[utoipa::path(
    post,
    path = "/api/v1/users/logout",
    request_body = LogoutRequest,
    responses(
        (status = 204, description = "User logged out successfully"),
        (status = 400, description = "Invalid logout request data", body = ProblemDetails),
        (status = 401, description = "Unauthorized - Valid bearer token required", body = ProblemDetails),
        (status = 403, description = "Forbidden - Session is invalid or already revoked", body = ProblemDetails),
    ),
    tag = "Users",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn logout_handler(
    State(state): State<UserState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Json(payload): Json<LogoutRequest>,
) -> Result<StatusCode, HttpError> {
    payload.validate().map_err(AppError::from)?;

    state.logout_interactor.execute(user_id, payload).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    get,
    path = "/api/v1/users/me",
    responses(
        (status = 200, description = "Current user profile retrieved", body = UserProfileResponse),
        (status = 401, description = "Unauthorized - Invalid token or user no longer exists", body = ProblemDetails),
        (status = 403, description = "Forbidden - Account locked, disabled, or email not verified", body = ProblemDetails),
    ),
    tag = "Users",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_me_handler(
    State(state): State<UserState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
) -> Result<(StatusCode, Json<UserProfileResponse>), HttpError> {
    let response = state.get_me_interactor.execute(user_id).await?;
    Ok((StatusCode::OK, Json(response)))
}
