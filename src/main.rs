use sqlx::postgres::PgPoolOptions;
use std::{net::SocketAddr, time::Duration};
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

mod handlers;
mod models;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .compact()
        .init();

    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        error!("DATABASE_URL is not set");
        std::process::exit(1);
    });

    let host_address = std::env::var("HOST_ADDRESS").unwrap_or_else(|_| {
        error!("HOST_ADDRESS is not set");
        std::process::exit(1);
    });

    let host_port = std::env::var("HOST_PORT").unwrap_or_else(|_| {
        error!("HOST_PORT is not set");
        std::process::exit(1);
    });

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await
        .unwrap_or_else(|e| {
            error!("Could not connect to database: {}", e);
            std::process::exit(1);
        });
    info!("Database connected");

    let routes = routes::routes(pool);

    let address: SocketAddr = format!("{}:{}", host_address, host_port)
        .parse()
        .unwrap_or_else(|e| {
            error!("Invalid address: {}", e);
            std::process::exit(1);
        });

    info!("Server running at http://{}", address);
    warp::serve(routes).run(address).await;
}
