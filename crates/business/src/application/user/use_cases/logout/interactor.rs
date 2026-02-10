use std::sync::Arc;

use crate::application::user::{
    error::AppError,
    services::session_service::SessionService,
    use_cases::logout::{LogoutUseCase, dtos::LogoutCommand},
};

pub struct LogoutInteractor {
    session_service: Arc<dyn SessionService>,
}

impl LogoutInteractor {
    pub fn new(session_service: Arc<dyn SessionService>) -> Self {
        Self { session_service }
    }
}

#[async_trait::async_trait]
impl LogoutUseCase for LogoutInteractor {
    async fn execute(&self, command: LogoutCommand) -> Result<(), AppError> {
        self.session_service
            .end_session(&command.user_id, &command.refresh_token)
            .await?;

        Ok(())
    }
}
