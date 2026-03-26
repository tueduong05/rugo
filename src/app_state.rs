use std::sync::Arc;

use crate::config::JwtConfig;

use business::{
    application::{
        link::{
            services::link_provider_impl::LinkProviderImpl,
            use_cases::{
                get_link::interactor::GetLinkInteractor,
                get_user_links::interactor::GetUserLinksInteractor,
                post_link::interactor::PostLinkInteractor,
            },
        },
        link_analytics::{
            use_cases::get_link_stats::interactor::GetLinkStatsInteractor,
            workers::AnalyticsBatchWorker,
        },
        user::use_cases::{
            get_me::interactor::GetMeInteractor, login::interactor::LoginInteractor,
            logout::interactor::LogoutInteractor, refresh::interactor::RefreshSessionInteractor,
            register::interactor::RegisterInteractor,
        },
    },
    domain::{
        common::{
            events::analytics_event::AnalyticsEvent, services::analytics_queue::AnalyticsQueue,
        },
        link::{repositories::LinkRepository, services::short_code_services::ShortCodeGenerator},
    },
};
use infrastructure::{
    common::security::password_services::{Argon2idHasher, ZxcvbnPolicy},
    link::{
        persistence::cache_aside_link_repository::CacheAsideLinkRepository,
        persistence::postgres_link_repository::PostgresLinkRepository,
        persistence::redis_link_repository::RedisLinkRepository,
        services::short_code_services::RandomShortCodeGenerator,
    },
    link_analytics::{
        persistence::postgres_analytics_repository::PostgresAnalyticsRepository,
        runner::run_analytics_worker,
        services::{
            mock_geo_service::MockGeoService, mpsc_analytics_queue::MPSCAnalyticsQueue,
            woothee_parser::WootheeUserAgentParser,
        },
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
use redis::aio::ConnectionManager;
use sqlx::PgPool;
use tokio::{sync::mpsc, task::JoinHandle};

pub struct AppStates {
    pub user: UserState,
    pub link: LinkState,
    pub analytics: AnalyticsState,
}

pub async fn bootstrap(
    pool: PgPool,
    redis_manager: ConnectionManager,
    jwt_config: JwtConfig,
    link_cache_ttl_seconds: u64,
    link_max_clicks_ttl_seconds: u64,
) -> (AppStates, JoinHandle<()>) {
    let user_repo = Arc::new(PostgresUserRepository::new(pool.clone()));
    let session_repo = Arc::new(PostgresSessionRepository::new(pool.clone()));
    let postgres_link_repo: Arc<dyn LinkRepository> =
        Arc::new(PostgresLinkRepository::new(pool.clone()));
    let redis_link_repo = RedisLinkRepository::new(
        redis_manager,
        link_cache_ttl_seconds,
        link_max_clicks_ttl_seconds,
    );
    let link_repo: Arc<dyn LinkRepository> = Arc::new(CacheAsideLinkRepository::new(
        postgres_link_repo.clone(),
        Arc::new(redis_link_repo),
    ));
    let analytics_repo = Arc::new(PostgresAnalyticsRepository::new(pool.clone()));

    let link_provider = Arc::new(LinkProviderImpl::new(link_repo.clone()));

    let password_policy = Arc::new(ZxcvbnPolicy::new(3));
    let password_hasher = Arc::new(Argon2idHasher);
    let session_service = Arc::new(JwtService::new(
        session_repo,
        jwt_config.secret,
        jwt_config.issuer,
        jwt_config.access_token_seconds,
        jwt_config.refresh_token_seconds,
    ));

    let short_code_generator: Arc<dyn ShortCodeGenerator> = Arc::new(RandomShortCodeGenerator);

    let (tx, rx) = mpsc::channel::<AnalyticsEvent>(1024);

    let analytics_queue: Arc<dyn AnalyticsQueue> = Arc::new(MPSCAnalyticsQueue::new(tx));

    let geo = MockGeoService;
    let ua = WootheeUserAgentParser::new();
    let worker = AnalyticsBatchWorker::new(analytics_repo.clone(), geo, ua);

    let worker_handle = tokio::spawn(async move {
        run_analytics_worker::<MockGeoService, WootheeUserAgentParser>(rx, worker).await;
    });

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
        get_link_stats_interactor: Arc::new(GetLinkStatsInteractor::new(
            link_provider,
            analytics_repo,
        )),
    };

    (
        AppStates {
            user: user_state,
            link: link_state,
            analytics: analytics_state,
        },
        worker_handle,
    )
}
