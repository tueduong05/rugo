use axum::{Extension, Router, routing::get};

use crate::link_analytics::{AnalyticsState, handlers::get_link_stats_handler};

pub fn analytics_routes(state: AnalyticsState) -> Router<AnalyticsState> {
    Router::new()
        .route("/{id}/analytics", get(get_link_stats_handler))
        .layer(Extension(state.session_service.clone()))
}
