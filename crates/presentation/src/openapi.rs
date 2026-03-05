use crate::link::handlers::*;
use crate::link_analytics::handlers::*;
use crate::user::handlers::*;
use business::application::{link, link_analytics, user};
use utoipa::{
    Modify, OpenApi,
    openapi::{
        self,
        security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    },
};

use crate::error;

#[derive(OpenApi)]
#[openapi(
    paths(
        register_handler,
        login_handler,
        refresh_session_handler,
        logout_handler,
        get_me_handler,
        post_link_handler,
        get_link_handler,
        get_user_links_handler,
        get_link_stats_handler
    ),
    components(
        schemas(
            user::common::auth_response::AuthResponse,
            user::common::user_profile_response::UserProfileResponse,
            user::use_cases::login::request::LoginRequest,
            user::use_cases::logout::request::LogoutRequest,
            user::use_cases::refresh::dtos::RefreshSessionRequest,
            user::use_cases::refresh::dtos::RefreshSessionResponse,
            user::use_cases::register::request::RegisterRequest,
            link::use_cases::get_user_links::response::GetUserLinksResponse,
            link::use_cases::post_link::dtos::PostLinkRequest,
            link::use_cases::post_link::dtos::PostLinkResponse,
            link_analytics::use_cases::get_link_stats::response::GetLinkStatsResponse,
            error::ProblemDetails
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "Users", description = "User management"),
        (name = "Links", description = "Link management")
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}
