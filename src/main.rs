use std::{net::SocketAddr, time::Duration};

use infrastructure::{db, redis};
use presentation::build_app;
use tokio::{net::TcpListener, time::timeout};

mod app_state;
mod config;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();
    let config = config::AppConfig::from_env().expect("Failed to load app configuration");

    let pool = db::create_pool(&config.database_url).await.unwrap();

    db::run_migrations(&pool).await.unwrap();

    let redis_manager = redis::create_connection_manager(&config.redis_url)
        .await
        .unwrap();

    let (states, worker_handle) = app_state::bootstrap(
        pool,
        redis_manager,
        config.jwt,
        config.link_cache_ttl_seconds,
    )
    .await;

    let app = build_app(states.user, states.link, states.analytics);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = TcpListener::bind(addr).await.unwrap();
    tracing::info!(%addr, "Server started");

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
