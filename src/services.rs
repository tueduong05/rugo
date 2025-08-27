use crate::models::UrlModel;
use rand::distr::{Alphanumeric, SampleString};
use sqlx::{Error, PgPool};
use std::fmt::{Display, Formatter};
use url::Url;

pub enum ServiceError {
    InvalidUrl,
    NotFound,
    DatabaseError(Error),
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceError::InvalidUrl => write!(f, "Invalid URL"),
            ServiceError::NotFound => write!(f, "URL not found"),
            ServiceError::DatabaseError(e) => write!(f, "Database error: {}", e),
        }
    }
}

fn generate_short_code(length: usize) -> String {
    let mut rng = rand::rng();
    Alphanumeric.sample_string(&mut rng, length)
}

pub async fn create_short_url(url: &str, pool: PgPool) -> Result<UrlModel, ServiceError> {
    let parsed_url = Url::parse(url).map_err(|_| ServiceError::InvalidUrl)?;

    let short_code = loop {
        let candidate = generate_short_code(5);

        let exists = sqlx::query!(
            "SELECT EXISTS(SELECT 1 FROM urls WHERE short_code = $1)",
            candidate
        )
        .fetch_one(&pool)
        .await
        .map_err(ServiceError::DatabaseError)?
        .exists
        .unwrap_or(false);

        if !exists {
            break candidate;
        }
    };

    let model = sqlx::query_as!(
        UrlModel,
        r#"INSERT INTO urls (
            url,
            short_code
        ) VALUES ($1, $2)
        RETURNING *
        "#,
        parsed_url.as_str(),
        short_code
    )
    .fetch_one(&pool)
    .await
    .map_err(ServiceError::DatabaseError)?;

    Ok(model)
}

pub async fn retrieve_original_url(
    short_code: &str,
    pool: PgPool,
) -> Result<UrlModel, ServiceError> {
    let model = sqlx::query_as!(
        UrlModel,
        r#"
        UPDATE urls
        SET access_count = access_count + 1
        WHERE short_code = $1
        RETURNING *
        "#,
        short_code
    )
    .fetch_optional(&pool)
    .await
    .map_err(ServiceError::DatabaseError)?;

    model.ok_or(ServiceError::NotFound)
}

pub async fn update_short_url(
    short_code: &str,
    url: &str,
    pool: PgPool,
) -> Result<UrlModel, ServiceError> {
    let parsed_url = Url::parse(url).map_err(|_| ServiceError::InvalidUrl)?;

    let model = sqlx::query_as!(
        UrlModel,
        r#"
        UPDATE urls
        SET url = $2
        WHERE short_code = $1
        RETURNING *
        "#,
        short_code,
        parsed_url.as_str()
    )
    .fetch_optional(&pool)
    .await
    .map_err(ServiceError::DatabaseError)?;

    model.ok_or(ServiceError::NotFound)
}

pub async fn delete_short_url(short_code: &str, pool: PgPool) -> Result<(), ServiceError> {
    let result = sqlx::query!("DELETE FROM urls WHERE short_code = $1", short_code)
        .execute(&pool)
        .await
        .map_err(ServiceError::DatabaseError)?;

    if result.rows_affected() == 0 {
        Err(ServiceError::NotFound)
    } else {
        Ok(())
    }
}

pub async fn get_url_statistics(short_code: &str, pool: PgPool) -> Result<UrlModel, ServiceError> {
    let model = sqlx::query_as!(
        UrlModel,
        r#"
        SELECT * FROM urls WHERE short_code = $1
        "#,
        short_code
    )
    .fetch_optional(&pool)
    .await
    .map_err(ServiceError::DatabaseError)?;

    model.ok_or(ServiceError::NotFound)
}
