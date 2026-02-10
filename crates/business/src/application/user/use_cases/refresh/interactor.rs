use std::sync::Arc;

use crate::application::user::{
    error::AppError,
    services::session_service::SessionService,
    use_cases::refresh::{
        RefreshSessionUseCase,
        dtos::{RefreshSessionRequest, RefreshSessionResponse},
    },
};

pub struct RefreshSessionInteractor {
    session_service: Arc<dyn SessionService>,
}

impl RefreshSessionInteractor {
    pub fn new(session_service: Arc<dyn SessionService>) -> Self {
        Self { session_service }
    }
}

#[async_trait::async_trait]
impl RefreshSessionUseCase for RefreshSessionInteractor {
    async fn execute(
        &self,
        req: RefreshSessionRequest,
    ) -> Result<RefreshSessionResponse, AppError> {
        let new_tokens = self
            .session_service
            .rotate_session(&req.refresh_token)
            .await?;

        Ok(RefreshSessionResponse {
            access_token: new_tokens.access_token,
            token_type: "Bearer".into(),
            expires_in: new_tokens.expires_in,
            refresh_token: new_tokens.refresh_token,
        })
    }
}
