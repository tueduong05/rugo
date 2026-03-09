use std::{env, net::SocketAddr, time::Duration};

use infrastructure::db;
use presentation::build_app;
use tokio::{net::TcpListener, time::timeout};

mod app_state;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = db::create_pool(&database_url).await.unwrap();

    db::run_migrations(&pool).await.unwrap();

    let (states, worker_handle) = app_state::bootstrap(pool).await;

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
