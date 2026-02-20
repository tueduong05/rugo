use business::domain::user::{
    entities::RefreshToken, error::DomainError, repositories::SessionRepository,
    value_objects::user_id::UserId,
};
use sha2::{Digest, Sha256};
use sqlx::PgPool;

use crate::user::persistence::models::RefreshTokenRecord;

#[derive(Clone)]
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
    ) -> Result<(), DomainError> {
        let record = RefreshTokenRecord::from(&session);

        match old_version {
            None => {
                if record.token.is_empty() {
                    return Err(DomainError::Infrastructure("Token cannot be empty".into()));
                }

                let hashed = self.hash_token(&record.token);

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
                .map_err(|e| DomainError::Infrastructure(e.to_string()))?;
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
                .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

                if result.rows_affected() == 0 {
                    return Err(DomainError::ConcurrencyError);
                }
            }
        }

        Ok(())
    }

    async fn find_by_token(&self, token: &str) -> Result<RefreshToken, DomainError> {
        let hashed = self.hash_token(token);

        let record = sqlx::query_as!(
            RefreshTokenRecord,
            r#"
            SELECT 
                id, 
                user_id, 
                '' as "token!",
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
        .map_err(|e| DomainError::Infrastructure(e.to_string()))?
        .ok_or(DomainError::InvalidSession)?;

        record.try_into_domain()
    }

    async fn revoke(&self, user_id: &UserId, token: &str) -> Result<(), DomainError> {
        let hashed = self.hash_token(token);

        let result = sqlx::query!(
            r#"
            UPDATE refresh_tokens 
            SET is_revoked = true 
            WHERE token_hash = $1 
                AND user_id = $2 
                AND is_used = false
                AND is_revoked = false
                AND expires_at > NOW()
            "#,
            hashed,
            user_id.value()
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(DomainError::InvalidSession);
        }

        Ok(())
    }

    async fn revoke_all(&self, user_id: &UserId) -> Result<(), DomainError> {
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
        .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

        Ok(())
    }
}
