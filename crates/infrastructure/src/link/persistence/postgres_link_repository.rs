use std::borrow::Cow;

use business::domain::{
    common::error::BaseDomainError,
    link::{
        entities::Link, error::LinkDomainError, repositories::LinkRepository,
        value_objects::short_code::ShortCode,
    },
    user::value_objects::user_id::UserId,
};
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
            if let Some(db_err) = e.as_database_error() {
                if db_err.code() == Some(Cow::Borrowed("23505")) {
                    return LinkDomainError::ShortCodeAlreadyExists;
                }
            }
            BaseDomainError::Infrastructure(e.to_string()).into()
        })?;

        Ok(())
    }

    async fn find_by_short_code(&self, short_code: &ShortCode) -> Result<Link, LinkDomainError> {
        let code_str = short_code.to_string();

        let record = sqlx::query_as!(
            LinkRecord,
            "SELECT * FROM links WHERE short_code = $1",
            code_str
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| BaseDomainError::Infrastructure(e.to_string()))?
        .ok_or(LinkDomainError::InvalidShortCode)?;

        record.try_into_domain()
    }

    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Vec<Link>, LinkDomainError> {
        let records = sqlx::query_as!(
            LinkRecord,
            "SELECT * FROM links WHERE user_id = $1",
            user_id.value()
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| BaseDomainError::Infrastructure(e.to_string()))?;

        records
            .into_iter()
            .map(|r| r.try_into_domain())
            .collect::<Result<Vec<Link>, LinkDomainError>>()
    }
}
