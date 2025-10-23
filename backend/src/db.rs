use sqlx::PgPool;

/// Establishes a connection pool to the PostgreSQL database.
pub async fn connect_from_env() -> anyhow::Result<PgPool> {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in environment variables");
    let pool = PgPool::connect(&database_url).await?;
    Ok(pool)
}

/// Runs database migrations to ensure the schema is up to date.
pub async fn run_migrations(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}