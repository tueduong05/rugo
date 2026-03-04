use axum::{
    Json,
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Redirect, Response},
};
use axum_client_ip::ClientIp;
use business::application::{
    error::AppError,
    link::use_cases::{
        get_link::dtos::{GetLinkCommand, GetLinkRequest},
        get_user_links::response::GetUserLinksResponse,
        post_link::dtos::{PostLinkRequest, PostLinkResponse},
    },
};
use validator::Validate;

use crate::{
    common::middleware::AuthenticatedUser,
    error::{HttpError, ProblemDetails},
    link::LinkState,
};

#[utoipa::path(
    post,
    path = "/api/v1/links",
    request_body = PostLinkRequest,
    responses(
        (status = 201, description = "Shortened link created", body = PostLinkResponse),
        (status = 400, description = "Invalid input data", body = ProblemDetails),
        (status = 401, description = "Unauthorized", body = ProblemDetails),
        (status = 409, description = "Short code already exists", body = ProblemDetails),
        (status = 422, description = "Invalid URL format or link data", body = ProblemDetails)
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
        (status = 307, description = "Redirecting to original link"),
        (status = 401, description = "Password required or incorrect password", body = ProblemDetails),
        (status = 403, description = "Link is inactive or click limit reached", body = ProblemDetails),
        (status = 404, description = "Link not found", body = ProblemDetails),
        (status = 410, description = "Link has expired", body = ProblemDetails)
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
    headers: HeaderMap,
    ClientIp(ip): ClientIp,
    Query(params): Query<GetLinkRequest>,
) -> Result<Response, HttpError> {
    let referrer = headers
        .get(header::REFERER)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let user_agent = headers
        .get(header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let command = GetLinkCommand {
        short_code,
        password: params.password,
        referrer,
        user_agent,
        ip,
    };

    let original_link = state.get_link_interactor.execute(command).await?;
    Ok(Redirect::temporary(&original_link.to_string()).into_response())
}

#[utoipa::path(
    get,
    path = "/api/v1/links/me",
    responses(
        (status = 200, description = "Successfully retrieved user links", body = GetUserLinksResponse),
        (status = 401, description = "Unauthorized - Missing or invalid token", body = ProblemDetails),
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
