use axum::{Router, routing::get};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    link::{LinkState, handlers::get_link_handler, routes::link_routes},
    openapi::ApiDoc,
    user::{UserState, routes::user_routes},
};

mod common;
mod error;
pub mod link;
mod openapi;
pub mod user;

pub fn build_app(user_state: UserState, link_state: LinkState) -> Router {
    let user_api = Router::new()
        .nest("/api/v1/users", user_routes(user_state.clone()))
        .with_state(user_state);

    let link_api = Router::new()
        .nest("/api/v1/links", link_routes(link_state.clone()))
        .route("/{short_code}", get(get_link_handler))
        .with_state(link_state);

    Router::new()
        .merge(user_api)
        .merge(link_api)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
}
