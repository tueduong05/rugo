use std::sync::Arc;

use axum::{Router, routing::post};
use business::application::user::use_cases::register::RegisterUseCase;

use crate::user::http::handlers::register_handler;

pub fn user_routes(interactor: Arc<dyn RegisterUseCase>) -> Router {
    Router::new()
        .route("/register", post(register_handler))
        .with_state(interactor)
}
