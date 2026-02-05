use std::sync::Arc;

use axum::Router;
use business::application::user::use_cases::{
    login::interactor::LoginInteractor,
    refresh::interactor::RefreshSessionInteractor,
    register::{RegisterUseCase, interactor::RegisterInteractor},
};
use infrastructure::user::{
    persistence::mock_repositories::MockUserRepository,
    security::{
        mock_password_services::{MockPasswordHasher, MockPasswordPolicy},
        mock_token_service::MockTokenService,
    },
};
use presentation::user::http::{UserState, routes::user_routes};
use tokio::net::TcpListener;

pub struct AppState {}

#[tokio::main]
async fn main() {
    let user_repo = Arc::new(MockUserRepository::new());
    let password_policy = Arc::new(MockPasswordPolicy);
    let password_hasher = Arc::new(MockPasswordHasher);
    let token_service = Arc::new(MockTokenService);

    let register_interactor: Arc<dyn RegisterUseCase> = Arc::new(RegisterInteractor::new(
        user_repo.clone(),
        password_policy.clone(),
        password_hasher.clone(),
        token_service.clone(),
    ));

    let login_interactor = Arc::new(LoginInteractor::new(
        user_repo,
        password_hasher,
        token_service.clone(),
    ));

    let refresh_token_interactor = Arc::new(RefreshSessionInteractor::new(token_service));

    let state = UserState {
        register_interactor,
        login_interactor,
        refresh_token_interactor,
    };

    let app = Router::new().nest("/user", user_routes(state));

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("🚀 Server running on http://0.0.0.0:8080");

    axum::serve(listener, app).await.unwrap();
}
