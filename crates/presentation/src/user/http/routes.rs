use axum::{
    Router,
    routing::{get, post},
};

use crate::user::http::{
    UserState,
    handlers::{login_handler, refresh_token_handler, register_handler},
};

pub fn user_routes(state: UserState) -> Router {
    Router::new()
        .route("/health", get(|| async { "Hello, World!" }))
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .route("/refresh", post(refresh_token_handler))
        .with_state(state)
}
