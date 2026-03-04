use axum::{
    Json,
    extract::{Path, State},
};
use business::application::link_analytics::use_cases::get_link_stats::response::GetLinkStatsResponse;

use crate::{
    common::middleware::AuthenticatedUser,
    error::{HttpError, ProblemDetails},
    link_analytics::AnalyticsState,
};

#[utoipa::path(
    get,
    path = "/api/v1/links/{id}/analytics",
    responses(
        (status = 200, description = "Successfully retrieved link analytics", body = GetLinkStatsResponse),
        (status = 401, description = "Unauthorized", body = ProblemDetails),
        (status = 403, description = "The authenticated user does not have permission to view statistics for this specific link", body = ProblemDetails),
        (status = 404, description = "Link not found", body = ProblemDetails),
    ),
    params(
        ("id" = u64, Path, description = "The unique ID of the link")
    ),
    tag = "Links",
    security(("bearer_auth" = []))
)]
pub async fn get_link_stats_handler(
    State(state): State<AnalyticsState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Path(link_id): Path<u64>,
) -> Result<Json<GetLinkStatsResponse>, HttpError> {
    let response = state
        .get_link_stats_interactor
        .execute(user_id, link_id)
        .await?;

    Ok(Json(response))
}
