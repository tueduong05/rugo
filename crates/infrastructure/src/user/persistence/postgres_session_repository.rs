use business::domain::{
    common::{error::BaseDomainError, value_objects::user_id::UserId},
    user::{entities::RefreshToken, error::UserDomainError, repositories::SessionRepository},
};
use sha2::{Digest, Sha256};
use sqlx::PgPool;

use crate::user::persistence::models::RefreshTokenRecord;

pub struct PostgresSessionRepository {
    pool: PgPool,
}

impl PostgresSessionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn hash_token(&self, token: &str) -> String {
        let hash = Sha256::digest(token.as_bytes());
        hex::encode(hash)
    }
}

#[async_trait::async_trait]
impl SessionRepository for PostgresSessionRepository {
    async fn save(
        &self,
        session: RefreshToken,
        old_version: Option<u64>,
    ) -> Result<(), UserDomainError> {
        let record = RefreshTokenRecord::from(&session);

        match old_version {
            None => {
                let token = record.token.as_ref().ok_or_else(|| {
                    BaseDomainError::Unexpected("Token missing for new record".into())
                })?;

                let hashed = self.hash_token(token);

                sqlx::query!(
                    r#"
                    INSERT INTO refresh_tokens (user_id, token_hash, expires_at, is_used, is_revoked, version)
                    VALUES ($1, $2, $3, $4, $5, $6)
                    "#,
                    record.user_id,
                    hashed,
                    record.expires_at,
                    record.is_used,
                    record.is_revoked,
                    record.version
                )
                .execute(&self.pool)
                .await
                .map_err(|e| match e {
                    sqlx::Error::Database(db_err) if db_err.is_foreign_key_violation() => {
                        UserDomainError::from(BaseDomainError::ResourceNotFound("User".into()))
                    }
                    _ => BaseDomainError::Infrastructure(e.to_string()).into(),
                })?;
            }

            Some(expected_version) => {
                let result = sqlx::query!(
                    r#"
                    UPDATE refresh_tokens 
                    SET expires_at = $1, is_used = $2, is_revoked = $3, version = $4
                    WHERE id = $5 AND version = $6
                    "#,
                    record.expires_at,
                    record.is_used,
                    record.is_revoked,
                    record.version,
                    record.id,
                    expected_version as i64
                )
                .execute(&self.pool)
                .await
                .map_err(|e| BaseDomainError::Infrastructure(e.to_string()))?;

                if result.rows_affected() == 0 {
                    return Err(UserDomainError::from(BaseDomainError::ConcurrencyError));
                }
            }
        }

        Ok(())
    }

    async fn find_by_token(&self, token: &str) -> Result<RefreshToken, UserDomainError> {
        let hashed = self.hash_token(token);

        let record = sqlx::query_as!(
            RefreshTokenRecord,
            r#"
            SELECT 
                id, 
                user_id, 
                NULL as "token: String",
                expires_at, 
                is_used, 
                is_revoked, 
                version
            FROM refresh_tokens 
            WHERE token_hash = $1
            "#,
            hashed
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| BaseDomainError::Infrastructure(e.to_string()))?
        .ok_or_else(|| {
            UserDomainError::from(BaseDomainError::ResourceNotFound("Session".into()))
        })?;

        record.try_into_domain()
    }

    async fn revoke(&self, user_id: UserId, token: &str) -> Result<(), UserDomainError> {
        let hashed = self.hash_token(token);

        let result = sqlx::query!(
            r#"
            UPDATE refresh_tokens 
            SET is_revoked = true 
            WHERE token_hash = $1 
                AND user_id = $2 
                AND is_revoked = false
            "#,
            hashed,
            user_id.value()
        )
        .execute(&self.pool)
        .await
        .map_err(|e| BaseDomainError::Infrastructure(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(BaseDomainError::SessionRevoked.into());
        }

        Ok(())
    }

    async fn revoke_all(&self, user_id: UserId) -> Result<(), UserDomainError> {
        sqlx::query!(
            r#"
            UPDATE refresh_tokens
            SET is_revoked = true
            WHERE user_id = $1 AND is_revoked = false
            "#,
            user_id.value()
        )
        .execute(&self.pool)
        .await
        .map_err(|e| BaseDomainError::Infrastructure(e.to_string()))?;

        Ok(())
    }
}
