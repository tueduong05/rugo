use std::{path::Path, time::Duration};

use sqlx::{PgPool, migrate::Migrator, postgres::PgPoolOptions};

pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(database_url)
        .await
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    let migrator = Migrator::new(Path::new("migrations")).await?;
    migrator.run(pool).await?;
    Ok(())
}
