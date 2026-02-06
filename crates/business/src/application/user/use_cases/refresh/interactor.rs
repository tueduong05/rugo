use std::sync::Arc;

use crate::application::user::{
    error::AppError,
    services::token_service::TokenService,
    use_cases::refresh::{
        RefreshSessionUseCase,
        dtos::{RefreshSessionRequest, RefreshSessionResponse},
    },
};

pub struct RefreshSessionInteractor {
    token_service: Arc<dyn TokenService>,
}

impl RefreshSessionInteractor {
    pub fn new(token_service: Arc<dyn TokenService>) -> Self {
        Self { token_service }
    }
}

#[async_trait::async_trait]
impl RefreshSessionUseCase for RefreshSessionInteractor {
    async fn execute(
        &self,
        req: RefreshSessionRequest,
    ) -> Result<RefreshSessionResponse, AppError> {
        let new_tokens = self
            .token_service
            .refresh_session(&req.refresh_token)
            .await?;

        Ok(RefreshSessionResponse {
            access_token: new_tokens.access_token,
            token_type: "Bearer".into(),
            expires_in: new_tokens.expires_in,
            refresh_token: new_tokens.refresh_token,
        })
    }
}
