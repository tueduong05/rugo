use std::sync::Arc;

use business::application::user::use_cases::{login::LoginUseCase, register::RegisterUseCase};

mod error;
mod handlers;
pub mod routes;

#[derive(Clone)]
pub struct UserState {
    pub register_interactor: Arc<dyn RegisterUseCase>,
    pub login_interactor: Arc<dyn LoginUseCase>,
}
