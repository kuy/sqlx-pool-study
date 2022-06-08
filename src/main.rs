use sqlx::{postgres::PgPoolOptions, Postgres, Pool};
use tracing_subscriber::EnvFilter;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).init();

    let pool = PgPoolOptions::new()
        .max_connections(1)
        .max_lifetime(Some(Duration::from_secs(1)))
        .connect("postgres://sqlx:sqlx@localhost:5432/sqlx").await?;

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

        let entity = reproduced_code(&pool).await?;

        tracing::warn!("Result: {:?}", entity);
    }

    Ok(())
}

async fn reproduced_code(pool: &Pool<Postgres>) -> Result<CoffeeEntity, sqlx::Error> {
    sqlx::query_as("SELECT * FROM coffee")
        .fetch_one(pool).await
}

async fn fixed_code(pool: &Pool<Postgres>) -> Result<CoffeeEntity, sqlx::Error> {
    sqlx::query_as("SELECT id, country FROM coffee")
        .fetch_one(pool).await
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CoffeeEntity {
    pub id: i32,
    pub country: String,
}
