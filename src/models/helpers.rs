use std::env;
use sqlx::{postgres::PgPoolOptions, Postgres, Pool};

/// Helper function to create a connection to the postgress DB.
pub async fn make_connection_pool() -> Pool<Postgres> {
    let connection_string = env::var("DATABASE_URL").unwrap();
    PgPoolOptions::new()
        .max_connections(5)
        .connect(connection_string.as_str())
        .await.expect("Failed to make connection to database.")
}