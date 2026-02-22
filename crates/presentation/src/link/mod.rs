use std::sync::Arc;

use business::application::{
    link::use_cases::{get_link::GetLinkUseCase, post_link::PostLinkUseCase},
    user::services::session_service::SessionService,
};

pub mod handlers;
pub mod routes;

#[derive(Clone)]
pub struct LinkState {
    pub session_service: Arc<dyn SessionService>,
    pub post_link_interactor: Arc<dyn PostLinkUseCase>,
    pub get_link_interactor: Arc<dyn GetLinkUseCase>,
}
