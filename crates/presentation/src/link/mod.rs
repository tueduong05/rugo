use std::sync::Arc;

use business::application::{
    link::use_cases::{
        get_link::GetLinkUseCase, get_user_links::GetUserLinksUseCase, post_link::PostLinkUseCase,
    },
    user::services::session_service::SessionService,
};

pub mod handlers;
pub mod routes;

#[derive(Clone)]
pub struct LinkState {
    pub session_service: Arc<dyn SessionService>,
    pub post_link_interactor: Arc<dyn PostLinkUseCase>,
    pub get_link_interactor: Arc<dyn GetLinkUseCase>,
    pub get_user_links_interactor: Arc<dyn GetUserLinksUseCase>,
}
