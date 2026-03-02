use std::{env, net::SocketAddr};

use infrastructure::db;
use presentation::build_app;
use tokio::net::TcpListener;

mod app_state;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = db::create_pool(&database_url).await.unwrap();

    db::run_migrations(&pool).await.unwrap();

    let states = app_state::bootstrap(pool).await;

    let app = build_app(states.user, states.link);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("🚀 Server running on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
