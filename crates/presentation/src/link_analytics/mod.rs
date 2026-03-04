use std::sync::Arc;

use business::application::{
    link_analytics::use_cases::get_link_stats::GetLinkStatsUseCase,
    user::services::session_service::SessionService,
};

pub mod handlers;
pub mod routes;

#[derive(Clone)]
pub struct AnalyticsState {
    pub session_service: Arc<dyn SessionService>,
    pub get_link_stats_interactor: Arc<dyn GetLinkStatsUseCase>,
}
