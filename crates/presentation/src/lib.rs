use axum::{Router, routing::get};
use tower_http::trace::{
    DefaultMakeSpan, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer,
};
use tracing::Level;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    link::{LinkState, handlers::get_link_handler, routes::link_routes},
    link_analytics::{AnalyticsState, routes::analytics_routes},
    openapi::ApiDoc,
    user::{UserState, routes::user_routes},
};

mod common;
mod error;
pub mod link;
pub mod link_analytics;
mod openapi;
pub mod user;

pub fn build_app(
    user_state: UserState,
    link_state: LinkState,
    analytics_states: AnalyticsState,
) -> Router {
    let user_api = Router::new()
        .nest("/api/v1/users", user_routes(user_state.clone()))
        .with_state(user_state);

    let link_api = Router::new()
        .nest("/api/v1/links", link_routes(link_state.clone()))
        .route("/{short_code}", get(get_link_handler))
        .with_state(link_state);

    let analytics_api = Router::new()
        .nest("/api/v1/links", analytics_routes(analytics_states.clone()))
        .with_state(analytics_states);

    Router::new()
        .merge(user_api)
        .merge(link_api)
        .merge(analytics_api)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(
                    DefaultMakeSpan::new()
                        .level(Level::INFO)
                        .include_headers(false),
                )
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO))
                .on_failure(DefaultOnFailure::new().level(Level::ERROR)),
        )
}
