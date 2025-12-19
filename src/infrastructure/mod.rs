use sqlx::{PgPool, postgres::PgPoolOptions};

/// Create a new PostgreSQL connection pool using a database URL.
async fn new_database_connection(
    database_url: &str,
    max_connections: u32,
) -> Result<PgPool, Box<dyn std::error::Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(database_url)
        .await?;

    Ok(pool)
}
