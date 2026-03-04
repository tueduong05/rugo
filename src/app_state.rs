use std::sync::Arc;

use business::{
    application::{
        link::use_cases::{
            get_link::interactor::GetLinkInteractor,
            get_user_links::interactor::GetUserLinksInteractor,
            post_link::interactor::PostLinkInteractor,
        },
        link_analytics::use_cases::get_link_stats::interactor::GetLinkStatsInteractor,
        user::use_cases::{
            get_me::interactor::GetMeInteractor, login::interactor::LoginInteractor,
            logout::interactor::LogoutInteractor, refresh::interactor::RefreshSessionInteractor,
            register::interactor::RegisterInteractor,
        },
    },
    domain::{
        link::services::short_code_services::ShortCodeGenerator,
        link_analytics::services::AnalyticsQueue,
    },
};
use infrastructure::{
    common::security::password_services::{Argon2idHasher, ZxcvbnPolicy},
    link::{
        persistence::postgres_link_repository::PostgresLinkRepository,
        services::short_code_services::RandomShortCodeGenerator,
    },
    link_analytics::{
        persistence::mock_repositories::MockAnalyticsRepository,
        services::mock_services::MockAnalyticsQueue,
    },
    user::{
        persistence::{
            postgres_session_repository::PostgresSessionRepository,
            postgres_user_repository::PostgresUserRepository,
        },
        security::jwt_service::JwtService,
    },
};
use presentation::{link::LinkState, link_analytics::AnalyticsState, user::UserState};
use sqlx::PgPool;

pub struct AppStates {
    pub user: UserState,
    pub link: LinkState,
    pub analytics: AnalyticsState,
}

pub async fn bootstrap(pool: PgPool) -> AppStates {
    let user_repo = Arc::new(PostgresUserRepository::new(pool.clone()));
    let session_repo = Arc::new(PostgresSessionRepository::new(pool.clone()));
    let link_repo = Arc::new(PostgresLinkRepository::new(pool.clone()));
    let analytics_repo = Arc::new(MockAnalyticsRepository::new());

    let password_policy = Arc::new(ZxcvbnPolicy::new(3));
    let password_hasher = Arc::new(Argon2idHasher);
    let session_service = Arc::new(JwtService::new(
        session_repo,
        "123".into(),
        "me".into(),
        900,
        3600,
    ));

    let short_code_generator: Arc<dyn ShortCodeGenerator> = Arc::new(RandomShortCodeGenerator);

    let analytics_queue: Arc<dyn AnalyticsQueue> = Arc::new(MockAnalyticsQueue);

    let user_state = UserState {
        session_service: session_service.clone(),
        register_interactor: Arc::new(RegisterInteractor::new(
            user_repo.clone(),
            password_policy,
            password_hasher.clone(),
            session_service.clone(),
        )),
        login_interactor: Arc::new(LoginInteractor::new(
            user_repo.clone(),
            password_hasher.clone(),
            session_service.clone(),
        )),
        refresh_session_interactor: Arc::new(RefreshSessionInteractor::new(
            session_service.clone(),
        )),
        logout_interactor: Arc::new(LogoutInteractor::new(session_service.clone())),
        get_me_interactor: Arc::new(GetMeInteractor::new(user_repo)),
    };

    let link_state = LinkState {
        session_service: session_service.clone(),
        post_link_interactor: Arc::new(PostLinkInteractor::new(
            link_repo.clone(),
            short_code_generator,
            password_hasher.clone(),
        )),
        get_link_interactor: Arc::new(GetLinkInteractor::new(
            link_repo.clone(),
            password_hasher,
            analytics_queue,
        )),
        get_user_links_interactor: Arc::new(GetUserLinksInteractor::new(link_repo.clone())),
    };

    let analytics_state = AnalyticsState {
        session_service,
        get_link_stats_interactor: Arc::new(GetLinkStatsInteractor::new(link_repo, analytics_repo)),
    };

    AppStates {
        user: user_state,
        link: link_state,
        analytics: analytics_state,
    }
}
