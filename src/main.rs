use std::{env, sync::Arc};

use axum::Router;
use business::application::user::use_cases::{
    get_me::interactor::GetMeInteractor,
    login::interactor::LoginInteractor,
    logout::interactor::LogoutInteractor,
    refresh::interactor::RefreshSessionInteractor,
    register::{RegisterUseCase, interactor::RegisterInteractor},
};
use infrastructure::user::{
    persistence::{
        db, postgres_session_repository::PostgresSessionRepository,
        postgres_user_repository::PostgresUserRepository,
    },
    security::{
        jwt_service::JwtService,
        password_services::{Argon2idHasher, ZxcvbnPolicy},
    },
};
use presentation::user::http::{UserState, routes::user_routes};
use tokio::net::TcpListener;

pub struct AppState {}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failed to load .env");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = db::create_pool(&database_url).await.unwrap();

    let user_repo = Arc::new(PostgresUserRepository::new(pool.clone()));
    let session_repo = Arc::new(PostgresSessionRepository::new(pool));
    let password_policy = Arc::new(ZxcvbnPolicy::new(3));
    let password_hasher = Arc::new(Argon2idHasher);
    let session_service = Arc::new(JwtService::new(
        session_repo,
        "123".into(),
        "me".into(),
        900,
        3600,
    ));

    let register_interactor: Arc<dyn RegisterUseCase> = Arc::new(RegisterInteractor::new(
        user_repo.clone(),
        password_policy.clone(),
        password_hasher.clone(),
        session_service.clone(),
    ));

    let login_interactor = Arc::new(LoginInteractor::new(
        user_repo.clone(),
        password_hasher,
        session_service.clone(),
    ));

    let refresh_session_interactor =
        Arc::new(RefreshSessionInteractor::new(session_service.clone()));

    let logout_interactor = Arc::new(LogoutInteractor::new(session_service.clone()));

    let get_me_interactor = Arc::new(GetMeInteractor::new(user_repo));

    let state = UserState {
        session_service,
        register_interactor,
        login_interactor,
        refresh_session_interactor,
        logout_interactor,
        get_me_interactor,
    };

    let app = Router::new().nest("/v1/user", user_routes(state));

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("🚀 Server running on http://0.0.0.0:8080");

    axum::serve(listener, app).await.unwrap();
}
