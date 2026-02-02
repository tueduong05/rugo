use axum::{Router, routing::post};

use crate::user::http::{
    UserState,
    handlers::{login_handler, register_handler},
};

pub fn user_routes(state: UserState) -> Router {
    Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .with_state(state)
}
