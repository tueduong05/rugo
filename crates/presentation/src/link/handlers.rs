use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
};
use business::application::{
    error::AppError,
    link::use_cases::{
        get_link::request::GetLinkRequest,
        post_link::dtos::{PostLinkRequest, PostLinkResponse},
    },
};
use validator::Validate;

use crate::{common::middleware::AuthenticatedUser, error::HttpError, link::LinkState};

pub async fn post_link_handler(
    State(state): State<LinkState>,
    user: Result<AuthenticatedUser, HttpError>,
    Json(payload): Json<PostLinkRequest>,
) -> Result<(StatusCode, Json<PostLinkResponse>), HttpError> {
    payload.validate().map_err(AppError::from)?;

    let user_id = user.ok().map(|AuthenticatedUser(id)| id);

    let response = state.post_link_interactor.execute(user_id, payload).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn get_link_handler(
    State(state): State<LinkState>,
    Path(short_code): Path<String>,
    Query(params): Query<GetLinkRequest>,
) -> Result<Response, HttpError> {
    let original_link = state
        .get_link_interactor
        .execute(short_code, params)
        .await?;
    Ok(Redirect::temporary(&original_link.to_string()).into_response())
}
