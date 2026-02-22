use axum::{
    Extension, Router,
    routing::{get, post},
};

use crate::user::{
    UserState,
    handlers::{
        get_me_handler, login_handler, logout_handler, refresh_session_handler, register_handler,
    },
};

pub fn user_routes(state: UserState) -> Router<UserState> {
    let public_routes = Router::new()
        .route("/health", get(|| async { "Hello, World!" }))
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .route("/refresh", post(refresh_session_handler));

    let protected_routes = Router::new()
        .route("/logout", post(logout_handler))
        .route("/me", get(get_me_handler))
        .layer(Extension(state.session_service.clone()));

    Router::new().merge(public_routes).merge(protected_routes)
}
