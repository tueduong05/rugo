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
        get_user_links::response::GetUserLinksResponse,
        post_link::dtos::{PostLinkRequest, PostLinkResponse},
    },
};
use validator::Validate;

use crate::{common::middleware::AuthenticatedUser, error::HttpError, link::LinkState};

#[utoipa::path(
    post,
    path = "/api/v1/links",
    request_body = PostLinkRequest,
    responses(
        (status = 201, description = "Shortened link created", body = PostLinkResponse),
    ),
    tag = "Links",
    security((), ("bearer_auth" = []))
)]
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

#[utoipa::path(
    get,
    path = "/api/v1/links/{short_code}",
    responses(
        (status = 307, description = "Redirecting to orignal link"),
    ),
    params(
        ("short_code" = String, Path, description = "The unique short slug"),
        ("password" = Option<String>, Query, description = "Link password")
    ),
    tag = "Links",
)]
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

#[utoipa::path(
    get,
    path = "/api/v1/links/me",
    responses(
        (status = 200, description = "Successfully retrieved user links", body = GetUserLinksResponse),
    ),
    tag = "Links",
    security(("bearer_auth" = []))
)]
pub async fn get_user_links_handler(
    State(state): State<LinkState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
) -> Result<Json<GetUserLinksResponse>, HttpError> {
    let response = state.get_user_links_interactor.execute(user_id).await?;
    Ok(Json(response))
}
