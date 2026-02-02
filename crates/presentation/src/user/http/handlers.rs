use axum::{Json, extract::State, http::StatusCode};
use business::application::user::{
    dtos::auth_response::AuthResponse,
    use_cases::{login::request::LoginRequest, register::request::RegisterRequest},
};

use crate::user::http::{UserState, error::HttpError};

pub async fn register_handler(
    State(state): State<UserState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), HttpError> {
    let response = state.register_interactor.execute(payload).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn login_handler(
    State(state): State<UserState>,
    Json(payload): Json<LoginRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), HttpError> {
    let response = state.login_interactor.execute(payload).await?;
    Ok((StatusCode::CREATED, Json(response)))
}
