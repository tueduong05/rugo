use redis::{Client, RedisError, aio::ConnectionManager};

pub async fn create_connection_manager(redis_url: &str) -> Result<ConnectionManager, RedisError> {
    tracing::info!(target: "infrastructure", "Creating redis connection manager");

    let client = Client::open(redis_url).map_err(|error| {
        tracing::error!(target: "infrastructure", error = ?error, "Failed to create redis client");
        error
    })?;

    client.get_connection_manager().await.map_err(|error| {
        tracing::error!(target: "infrastructure", error = ?error, "Failed to create redis connection manager");
        error
    })
}
