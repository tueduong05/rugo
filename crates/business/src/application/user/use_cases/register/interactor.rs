use std::sync::Arc;

use validator::Validate;

use crate::{
    application::{
        error::AppError,
        user::{
            dtos::auth_response::AuthResponse,
            services::token_service::TokenService,
            use_cases::register::{RegisterUseCase, request::RegisterRequest},
        },
    },
    domain::user::{
        entity::User,
        repository::UserRepository,
        services::password_service::PasswordService,
        value_objects::{email::Email, password::Password, user_id::UserId, username::Username},
    },
};

struct RegisterInteractor {
    user_repo: Arc<dyn UserRepository>,
    password_service: Arc<dyn PasswordService>,
    token_service: Arc<dyn TokenService>,
}

impl RegisterUseCase for RegisterInteractor {
    fn execute(&self, req: RegisterRequest) -> Result<AuthResponse, AppError> {
        req.validate().map_err(AppError::from)?;

        let username = Username::new(req.username)?;
        let email = Email::new(req.email)?;
        let raw_password = Password::new(req.password)?;

        let hashed_password = self.password_service.hash(&raw_password);

        todo!()
    }
}
