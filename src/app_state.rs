use std::sync::Arc;

use business::{
    application::{
        link::use_cases::{
            get_link::interactor::GetLinkInteractor, post_link::interactor::PostLinkInteractor,
        },
        user::use_cases::{
            get_me::interactor::GetMeInteractor, login::interactor::LoginInteractor,
            logout::interactor::LogoutInteractor, refresh::interactor::RefreshSessionInteractor,
            register::interactor::RegisterInteractor,
        },
    },
    domain::link::services::short_code_services::ShortCodeGenerator,
};
use infrastructure::{
    link::{
        persistence::mock_repositories::MockLinkRepository,
        services::mock_short_code_services::MockShortCodeGenerator,
    },
    user::{
        persistence::{
            postgres_session_repository::PostgresSessionRepository,
            postgres_user_repository::PostgresUserRepository,
        },
        security::{
            jwt_service::JwtService,
            password_services::{Argon2idHasher, ZxcvbnPolicy},
        },
    },
};
use presentation::{link::LinkState, user::UserState};
use sqlx::PgPool;

pub struct AppStates {
    pub user: UserState,
    pub link: LinkState,
}

pub async fn bootstrap(pool: PgPool) -> AppStates {
    let user_repo = Arc::new(PostgresUserRepository::new(pool.clone()));
    let session_repo = Arc::new(PostgresSessionRepository::new(pool.clone()));
    let link_repo = Arc::new(MockLinkRepository::new());

    let password_policy = Arc::new(ZxcvbnPolicy::new(3));
    let password_hasher = Arc::new(Argon2idHasher);
    let session_service = Arc::new(JwtService::new(
        session_repo,
        "123".into(),
        "me".into(),
        900,
        3600,
    ));

    let short_code_generator: Arc<dyn ShortCodeGenerator> = Arc::new(MockShortCodeGenerator);

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
        session_service,
        post_link_interactor: Arc::new(PostLinkInteractor::new(
            link_repo.clone(),
            short_code_generator,
            password_hasher.clone(),
        )),
        get_link_interactor: Arc::new(GetLinkInteractor::new(link_repo, password_hasher)),
    };

    AppStates {
        user: user_state,
        link: link_state,
    }
}
