use std::{cmp::max, time::Duration};

use anyhow::{Context, Ok};
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, Statement};

pub(crate) async fn init() -> anyhow::Result<DatabaseConnection> {
    let mut options =
        sea_orm::ConnectOptions::new("postgresql://postgres:201030@postgres:5432/mydb");
    let cpus = num_cpus::get() as u32;
    options
        .min_connections(max(cpus * 4, 10))
        .max_connections(max(cpus * 8, 20))
        .connect_timeout(Duration::from_secs(10))
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(300))
        .max_lifetime(Duration::from_secs(3600 * 24))
        .sqlx_logging(false)
        .set_schema_search_path("public");
    let conn = Database::connect(options)
        .await
        .with_context(|| "Failed to connect to database")?;
    conn.ping()
        .await
        .with_context(|| "Failed to ping database")?;
    tracing::info!("Connected to database");
    tracing::info!("Database version: {}", database_version(&conn).await?);
    Ok(conn)
}

async fn database_version(conn: &DatabaseConnection) -> anyhow::Result<String> {
    let version_res = conn
        .query_one(Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            "SELECT version();",
        ))
        .await
        .with_context(|| "Failed to query database version")?
        .ok_or_else(|| anyhow::anyhow!("not found database version"))?;
    let version: String = version_res.try_get_by_index(0)?;
    Ok(version)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_database_version() -> anyhow::Result<()> {
        let conn = init().await?;
        let version = database_version(&conn).await?;
        assert!(version.contains("PostgreSQL"));
        Ok(())
    }
}
