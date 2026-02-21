use std::sync::Arc;

use crate::{
    application::{
        error::AppError,
        user::{
            services::session_service::SessionService,
            use_cases::logout::{LogoutUseCase, request::LogoutRequest},
        },
    },
    domain::user::value_objects::user_id::UserId,
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
    async fn execute(&self, user_id: UserId, req: LogoutRequest) -> Result<(), AppError> {
        self.session_service
            .end_session(&user_id, &req.refresh_token)
            .await?;

        Ok(())
    }
}
