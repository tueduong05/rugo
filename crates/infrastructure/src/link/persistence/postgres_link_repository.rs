use std::borrow::Cow;

use business::domain::{
    common::{error::BaseDomainError, value_objects::user_id::UserId},
    link::{
        entities::Link, error::LinkDomainError, repositories::LinkRepository,
        value_objects::short_code::ShortCode,
    },
};
use chrono::{DateTime, Utc};
use sqlx::PgPool;

use crate::link::persistence::models::LinkRecord;

pub struct PostgresLinkRepository {
    pool: PgPool,
}

impl PostgresLinkRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl LinkRepository for PostgresLinkRepository {
    async fn create(&self, link: &Link) -> Result<(), LinkDomainError> {
        let record = LinkRecord::from(link);

        sqlx::query!(
            r#"
            INSERT INTO links (
                user_id, original_link, short_code, is_custom, 
                expires_at, hashed_password, max_clicks, is_active, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
            record.user_id,
            record.original_link,
            record.short_code,
            record.is_custom,
            record.expires_at,
            record.hashed_password,
            record.max_clicks,
            record.is_active,
            record.created_at,
            record.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            if let Some(db_err) = e.as_database_error()
                && db_err.code() == Some(Cow::Borrowed("23505"))
            {
                return LinkDomainError::ShortCodeAlreadyExists;
            }
            BaseDomainError::Infrastructure(e.to_string()).into()
        })?;

        Ok(())
    }

    async fn find_by_id(&self, id: u64) -> Result<Option<Link>, LinkDomainError> {
        let record = sqlx::query_as!(LinkRecord, "SELECT * FROM links WHERE id = $1", id as i64)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| BaseDomainError::Infrastructure(e.to_string()))?;

        record.map(|r| r.try_into_domain()).transpose()
    }

    async fn find_by_short_code(
        &self,
        short_code: &ShortCode,
    ) -> Result<Option<Link>, LinkDomainError> {
        let code_str = short_code.to_string();

        let record = sqlx::query_as!(
            LinkRecord,
            "SELECT * FROM links WHERE short_code = $1",
            code_str
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| BaseDomainError::Infrastructure(e.to_string()))?;

        record.map(|r| r.try_into_domain()).transpose()
    }

    async fn find_by_user_id(&self, user_id: UserId) -> Result<Vec<Link>, LinkDomainError> {
        let records = sqlx::query_as!(
            LinkRecord,
            "SELECT * FROM links WHERE user_id = $1",
            user_id.value()
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| BaseDomainError::Infrastructure(e.to_string()))?;

        if records.is_empty() {
            return Ok(vec![]);
        }

        records
            .into_iter()
            .map(|r| r.try_into_domain())
            .collect::<Result<Vec<Link>, LinkDomainError>>()
    }

    async fn increment_clicks(&self, id: u64, now: DateTime<Utc>) -> Result<u64, LinkDomainError> {
        let result = sqlx::query!(
            r#"
            UPDATE links
            SET 
                current_clicks = current_clicks + 1,
                updated_at = $1
            WHERE id = $2
                AND is_active = true
                AND (expires_at IS NULL OR expires_at > $1)
                AND (max_clicks IS NULL OR current_clicks < max_clicks)
            "#,
            now,
            id as i64
        )
        .execute(&self.pool)
        .await
        .map_err(|e| BaseDomainError::Infrastructure(e.to_string()))?;

        Ok(result.rows_affected())
    }
}
