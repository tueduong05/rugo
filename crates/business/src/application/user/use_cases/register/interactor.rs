use std::sync::Arc;

use crate::{
    application::{
        error::AppError,
        user::{
            dtos::auth_response::AuthResponse,
            services::token_service::TokenService,
            use_cases::register::{RegisterUseCase, request::RegisterRequest},
        },
    },
    domain::user::{repository::UserRepository, services::password_service::PasswordService},
};

struct RegisterInteractor {
    user_repo: Arc<dyn UserRepository>,
    password_service: Arc<dyn PasswordService>,
    token_service: Arc<dyn TokenService>,
}

impl RegisterUseCase for RegisterInteractor {
    fn execute(&self, req: RegisterRequest) -> Result<AuthResponse, AppError> {
        todo!()
    }
}
