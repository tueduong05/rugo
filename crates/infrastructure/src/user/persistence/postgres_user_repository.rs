use std::borrow::Cow;

use business::domain::user::{
    entities::User,
    error::DomainError,
    repositories::UserRepository,
    value_objects::{login_identifier::LoginIdentifier, user_id::UserId},
};
use sqlx::PgPool;

use crate::user::persistence::models::{DbUserStatus, UserRecord};

#[derive(Clone)]
pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserRepository for PostgresUserRepository {
    async fn save(&self, user: &User) -> Result<(), DomainError> {
        let user_record = UserRecord::from(user);

        sqlx::query!(
            r#"
            INSERT INTO users (id, username, email, hashed_password, status, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            user_record.id,
            user_record.username,
            user_record.email,
            user_record.hashed_password,
            user_record.status as DbUserStatus,
            user_record.created_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            if let Some(db_err) = e.as_database_error()
                && db_err.code() == Some(Cow::Borrowed("23505"))
            {
                let constraint = db_err.constraint().unwrap_or("");
                if constraint.contains("email") {
                    return DomainError::EmailTaken;
                }
                if constraint.contains("username") {
                    return DomainError::UsernameTaken;
                }
            }
            DomainError::Infrastructure(e.to_string())
        })?;

        Ok(())
    }

    async fn find_by_identifier(
        &self,
        identifier: &LoginIdentifier,
    ) -> Result<Option<User>, DomainError> {
        let row = match identifier {
            LoginIdentifier::Email(e) => {
                sqlx::query_as!(
                    UserRecord,
                    r#"
                    SELECT id, username, email, hashed_password, status as "status: DbUserStatus", created_at
                    FROM users
                    WHERE email = $1
                    "#,
                    e.as_str()
                )
                .fetch_optional(&self.pool)
                .await
            }
            LoginIdentifier::Username(u) => {
                sqlx::query_as!(
                    UserRecord,
                    r#"
                    SELECT id, username, email, hashed_password, status as "status: DbUserStatus", created_at
                    FROM users
                    WHERE username = $1
                    "#,
                    u.as_str()
                )
                .fetch_optional(&self.pool)
                .await
            }
        }
        .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

        Ok(row.map(|r| r.try_into_domain()).transpose()?)
    }

    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Option<User>, DomainError> {
        let record_opt = sqlx::query_as!(
            UserRecord,
            r#"
            SELECT id, username, email, hashed_password, status as "status: DbUserStatus", created_at 
            FROM users 
            WHERE id = $1
            "#,
            user_id.value()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

        record_opt.map(|r| r.try_into_domain()).transpose()
    }
}
