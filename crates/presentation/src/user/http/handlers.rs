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

use crate::user::http::{UserState, error::HttpError, middleware::AuthenticatedUser};

pub async fn register_handler(
    State(state): State<UserState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), HttpError> {
    payload.validate().map_err(AppError::from)?;

    let response = state.register_interactor.execute(payload).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn login_handler(
    State(state): State<UserState>,
    Json(payload): Json<LoginRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), HttpError> {
    payload.validate().map_err(AppError::from)?;

    let response = state.login_interactor.execute(payload).await?;
    Ok((StatusCode::OK, Json(response)))
}

pub async fn refresh_session_handler(
    State(state): State<UserState>,
    Json(payload): Json<RefreshSessionRequest>,
) -> Result<(StatusCode, Json<RefreshSessionResponse>), HttpError> {
    payload.validate().map_err(AppError::from)?;

    let response = state.refresh_session_interactor.execute(payload).await?;
    Ok((StatusCode::OK, Json(response)))
}

pub async fn logout_handler(
    State(state): State<UserState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Json(payload): Json<LogoutRequest>,
) -> Result<StatusCode, HttpError> {
    payload.validate().map_err(AppError::from)?;

    state.logout_interactor.execute(user_id, payload).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_me_handler(
    State(state): State<UserState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
) -> Result<(StatusCode, Json<UserProfileResponse>), HttpError> {
    let response = state.get_me_interactor.execute(user_id).await?;
    Ok((StatusCode::OK, Json(response)))
}
