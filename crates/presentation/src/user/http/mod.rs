use std::sync::Arc;

use business::application::user::{
    services::session_service::SessionService,
    use_cases::{
        get_me::GetMeUseCase, login::LoginUseCase, logout::LogoutUseCase,
        refresh::RefreshSessionUseCase, register::RegisterUseCase,
    },
};

mod error;
mod handlers;
mod middleware;
pub mod routes;

#[derive(Clone)]
pub struct UserState {
    pub session_service: Arc<dyn SessionService>,
    pub register_interactor: Arc<dyn RegisterUseCase>,
    pub login_interactor: Arc<dyn LoginUseCase>,
    pub refresh_session_interactor: Arc<dyn RefreshSessionUseCase>,
    pub logout_interactor: Arc<dyn LogoutUseCase>,
    pub get_me_interactor: Arc<dyn GetMeUseCase>,
}
