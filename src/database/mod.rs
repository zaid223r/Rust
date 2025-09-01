use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;

pub type Database = PgPool;

pub async fn create_connection_pool(database_url: &str) -> Result<Database, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(20)
        .acquire_timeout(Duration::from_secs(30))
        .connect(database_url)
        .await
}