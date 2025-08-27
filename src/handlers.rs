use crate::models::{UrlRequest, UrlResponse, UrlStatsResponse};
use crate::services::{self, ServiceError};
use sqlx::PgPool;
use warp::{Rejection, Reply, http::StatusCode};

pub async fn create_short_url(body: UrlRequest, pool: PgPool) -> Result<impl Reply, Rejection> {
    match services::create_short_url(&body.url, pool).await {
        Ok(model) => {
            let response: UrlResponse = model.into();
            Ok(warp::reply::with_status(
                warp::reply::json(&response),
                StatusCode::CREATED,
            ))
        }
        Err(e) => {
            let code = match e {
                ServiceError::InvalidUrl => StatusCode::BAD_REQUEST,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };

            Ok(warp::reply::with_status(
                warp::reply::json(&serde_json::json!({"error": e.to_string()})),
                code,
            ))
        }
    }
}

pub async fn retrieve_original_url(
    short_code: String,
    pool: PgPool,
) -> Result<impl Reply, Rejection> {
    match services::retrieve_original_url(&short_code, pool).await {
        Ok(model) => {
            let response: UrlResponse = model.into();
            Ok(warp::reply::with_status(
                warp::reply::json(&response),
                StatusCode::OK,
            ))
        }
        Err(e) => {
            let code = match e {
                ServiceError::NotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };

            Ok(warp::reply::with_status(
                warp::reply::json(&serde_json::json!({"error" : e.to_string()})),
                code,
            ))
        }
    }
}

pub async fn update_short_url(
    short_code: String,
    body: UrlRequest,
    pool: PgPool,
) -> Result<impl Reply, Rejection> {
    match services::update_short_url(&short_code, &body.url, pool).await {
        Ok(model) => {
            let response: UrlResponse = model.into();
            Ok(warp::reply::with_status(
                warp::reply::json(&response),
                StatusCode::OK,
            ))
        }
        Err(e) => {
            let code = match e {
                ServiceError::InvalidUrl => StatusCode::BAD_REQUEST,
                ServiceError::NotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };

            Ok(warp::reply::with_status(
                warp::reply::json(&serde_json::json!({"error": e.to_string()})),
                code,
            ))
        }
    }
}

pub async fn delete_short_url(short_code: String, pool: PgPool) -> Result<impl Reply, Rejection> {
    match services::delete_short_url(&short_code, pool).await {
        Ok(()) => Ok(warp::reply::with_status("", StatusCode::NO_CONTENT)),
        Err(e) => {
            let code = match e {
                ServiceError::NotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };

            Ok(warp::reply::with_status("", code))
        }
    }
}

pub async fn get_url_statistics(short_code: String, pool: PgPool) -> Result<impl Reply, Rejection> {
    match services::get_url_statistics(&short_code, pool).await {
        Ok(model) => {
            let response: UrlStatsResponse = model.into();
            Ok(warp::reply::with_status(
                warp::reply::json(&response),
                StatusCode::OK,
            ))
        }
        Err(e) => {
            let code = match e {
                ServiceError::NotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };

            Ok(warp::reply::with_status(
                warp::reply::json(&serde_json::json!({"error": e.to_string()})),
                code,
            ))
        }
    }
}
