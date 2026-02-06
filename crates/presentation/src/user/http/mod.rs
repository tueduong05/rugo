use std::sync::Arc;

use business::application::user::{
    services::token_service::TokenService,
    use_cases::{
        login::LoginUseCase, logout::LogoutUseCase, refresh::RefreshSessionUseCase,
        register::RegisterUseCase,
    },
};

mod error;
mod handlers;
mod middleware;
pub mod routes;

#[derive(Clone)]
pub struct UserState {
    pub token_service: Arc<dyn TokenService>,
    pub register_interactor: Arc<dyn RegisterUseCase>,
    pub login_interactor: Arc<dyn LoginUseCase>,
    pub refresh_session_interactor: Arc<dyn RefreshSessionUseCase>,
    pub logout_interactor: Arc<dyn LogoutUseCase>,
}
