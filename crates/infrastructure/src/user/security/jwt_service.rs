use std::sync::Arc;

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use business::{
    application::user::{
        error::AppError,
        services::session_service::{SessionService, Tokens},
    },
    domain::user::{
        entities::RefreshToken, error::DomainError, repositories::SessionRepository,
        value_objects::user_id::UserId,
    },
};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use rand::{RngCore, rng};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iss: String,
}

pub struct JwtService {
    repo: Arc<dyn SessionRepository>,
    secret: String,
    issuer: String,
    access_token_seconds: u64,
    refresh_token_seconds: u64,
}

impl JwtService {
    pub fn new(
        repo: Arc<dyn SessionRepository>,
        secret: String,
        issuer: String,
        access_token_seconds: u64,
        refresh_token_seconds: u64,
    ) -> Self {
        Self {
            repo,
            secret,
            issuer,
            access_token_seconds,
            refresh_token_seconds,
        }
    }

    fn generate_random_token(&self) -> String {
        let mut bytes = [0u8; 32];
        rng().fill_bytes(&mut bytes);
        URL_SAFE_NO_PAD.encode(bytes)
    }
}

#[async_trait::async_trait]
impl SessionService for JwtService {
    async fn start_session(&self, user_id: &UserId) -> Result<Tokens, AppError> {
        let now = Utc::now();
        let access_expiry = now + Duration::seconds(self.access_token_seconds as i64);

        let claims = Claims {
            sub: user_id.to_string(),
            exp: access_expiry.timestamp() as usize,
            iss: self.issuer.clone(),
        };

        let access_token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
        .map_err(|_| DomainError::Unexpected("Token signing failed".into()))?;

        let refresh_token = self.generate_random_token();
        let expires_at = now + Duration::seconds(self.refresh_token_seconds as i64);

        let refresh_token = RefreshToken {
            id: 0,
            user_id: user_id.clone(),
            token: refresh_token,
            expires_at,
            is_used: false,
            is_revoked: false,
            version: 1,
        };

        self.repo.save(refresh_token.clone(), None).await?;

        Ok(Tokens {
            access_token,
            expires_in: self.access_token_seconds,
            refresh_token: refresh_token.token,
        })
    }

    async fn rotate_session(&self, refresh_token: &str) -> Result<Tokens, AppError> {
        let mut session = self
            .repo
            .find_by_token(refresh_token)
            .await
            .map_err(|_| DomainError::InvalidSession)?;

        if !session.is_valid(Utc::now()) {
            if session.is_used && !session.is_revoked {
                let _ = self.repo.revoke_all(&session.user_id).await;
            }
            return Err(DomainError::InvalidSession.into());
        }

        let old_version = session.version;
        session.mark_used();

        self.repo
            .save(session.clone(), Some(old_version))
            .await
            .map_err(|_| DomainError::ConcurrencyError)?;

        self.start_session(&session.user_id).await
    }

    async fn end_session(&self, user_id: &UserId, refresh_token: &str) -> Result<(), AppError> {
        self.repo.revoke(user_id, refresh_token).await?;
        Ok(())
    }

    async fn end_all_sessions(&self, user_id: &UserId) -> Result<(), AppError> {
        self.repo.revoke_all(user_id).await?;
        Ok(())
    }

    async fn authenticate(&self, access_token: &str) -> Result<UserId, AppError> {
        let token_data = decode::<Claims>(
            access_token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|e| match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => DomainError::SessionExpired,
            _ => DomainError::InvalidSession,
        })?;

        let user_id = token_data.claims.sub.parse::<UserId>().map_err(|_| {
            DomainError::Unexpected("UserId does not meet domain requirements".into())
        })?;

        Ok(user_id)
    }
}
