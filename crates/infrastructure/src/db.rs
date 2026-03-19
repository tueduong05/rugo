use std::{path::Path, time::Duration};

use sqlx::{PgPool, migrate::Migrator, postgres::PgPoolOptions};

pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    tracing::info!(target: "infrastructure", "Creating postgres connection pool");

    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(database_url)
        .await
        .map_err(|error| {
            tracing::error!(target: "infrastructure", error = ?error, "Failed to create postgres connection pool");
            error
        })
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    tracing::info!(target: "infrastructure", "Running database migrations");

    let migrator = Migrator::new(Path::new("migrations")).await.map_err(|error| {
        tracing::error!(target: "infrastructure", error = ?error, "Failed to load database migrations");
        error
    })?;

    migrator.run(pool).await.map_err(|error| {
        tracing::error!(target: "infrastructure", error = ?error, "Failed to execute database migrations");
        error
    })?;

    tracing::info!(target: "infrastructure", "Database migrations completed");
    Ok(())
}
