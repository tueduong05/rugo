use std::sync::Arc;

use axum::Router;
use business::application::user::use_cases::register::{
    RegisterUseCase, interactor::RegisterInteractor,
};
use infrastructure::user::{
    http::routes::user_routes,
    persistence::mock_repositories::MockUserRepository,
    security::{
        mock_password_services::{MockPasswordHasher, MockPasswordPolicy},
        mock_token_service::MockTokenService,
    },
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let user_repo = Arc::new(MockUserRepository::new());
    let password_policy = Arc::new(MockPasswordPolicy);
    let password_hasher = Arc::new(MockPasswordHasher);
    let token_service = Arc::new(MockTokenService);

    let register_interactor: Arc<dyn RegisterUseCase> = Arc::new(RegisterInteractor::new(
        user_repo,
        password_policy,
        password_hasher,
        token_service,
    ));

    let app = Router::new().nest("/user", user_routes(register_interactor));

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("🚀 Server running on http://0.0.0.0:8080");

    axum::serve(listener, app).await.unwrap();
}
