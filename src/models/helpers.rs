use dotenvy::dotenv;
use sqlx::{postgres::PgPoolOptions, Postgres, Pool};

pub async fn make_connection_pool() -> Pool<Postgres> {
    dotenv().ok();
    let connection_string = dotenvy::var("DATABASE_URL").unwrap();
    PgPoolOptions::new()
        .max_connections(5)
        .connect(connection_string.as_str())
        .await.expect("Failed to make connection to database.")
}