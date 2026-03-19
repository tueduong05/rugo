use std::{net::SocketAddr, time::Duration};

use infrastructure::db;
use presentation::build_app;
use tokio::{net::TcpListener, time::timeout};

mod app_state;
mod config;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let config = config::AppConfig::from_env().expect("Failed to load app configuration");

    let pool = db::create_pool(&config.database_url).await.unwrap();

    db::run_migrations(&pool).await.unwrap();

    let (states, worker_handle) = app_state::bootstrap(pool, config.jwt).await;

    let app = build_app(states.user, states.link, states.analytics);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("🚀 Server running on http://{}", addr);

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(async {
        tokio::signal::ctrl_c().await.unwrap();
    })
    .await
    .unwrap();

    let _ = timeout(Duration::from_secs(10), worker_handle).await;
}
