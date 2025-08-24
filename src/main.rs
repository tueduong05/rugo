use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

mod handlers;
mod models;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let host_address = std::env::var("HOST_ADDRESS").expect("HOST_ADDRESS is not set");
    let host_port = std::env::var("HOST_PORT").expect("HOST_PORT is not set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await
        .expect("Could not connect to database");

    let routes = routes::routes(pool);

    let address: std::net::SocketAddr = format!("{}:{}", host_address, host_port)
        .parse()
        .expect("Invalid address format");
    println!("Server running at http://{}", address);
    warp::serve(routes).run(address).await;
}
