use std::sync::Arc;

use crate::application::user::{
    error::AppError,
    services::token_service::TokenService,
    use_cases::logout::{LogoutUseCase, dtos::LogoutCommand},
};

pub struct LogoutInteractor {
    token_service: Arc<dyn TokenService>,
}

impl LogoutInteractor {
    pub fn new(token_service: Arc<dyn TokenService>) -> Self {
        Self { token_service }
    }
}

#[async_trait::async_trait]
impl LogoutUseCase for LogoutInteractor {
    async fn execute(&self, command: LogoutCommand) -> Result<(), AppError> {
        self.token_service
            .revoke_token(&command.user_id, &command.refresh_token)
            .await?;

        Ok(())
    }
}
