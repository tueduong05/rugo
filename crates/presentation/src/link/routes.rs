use axum::{
    Extension, Router,
    routing::{get, post},
};

use crate::link::{
    LinkState,
    handlers::{get_link_handler, get_user_links_handler, post_link_handler},
};

pub fn link_routes(state: LinkState) -> Router<LinkState> {
    let public_routes = Router::new()
        .route("/", post(post_link_handler))
        .route("/{short_code}", get(get_link_handler))
        .layer(Extension(state.session_service.clone()));

    let protected_routes = Router::new()
        .route("/me", get(get_user_links_handler))
        .layer(Extension(state.session_service.clone()));

    Router::new().merge(public_routes).merge(protected_routes)
}
