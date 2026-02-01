use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use business::application::user::{
    dtos::auth_response::AuthResponse,
    use_cases::register::{RegisterUseCase, request::RegisterRequest},
};

use crate::user::http::error::HttpError;

pub async fn register_handler(
    State(interactor): State<Arc<dyn RegisterUseCase>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), HttpError> {
    let response = interactor.execute(payload).await?;

    Ok((StatusCode::CREATED, Json(response)))
}
