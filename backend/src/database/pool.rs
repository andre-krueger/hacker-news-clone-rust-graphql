use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use deadpool_redis::{Config, Pool, Runtime};
use dotenv::dotenv;
use std::env;

pub async fn init_pool() -> PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").unwrap();
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap()
}

pub async fn init_redis_pool() -> Pool {
    dotenv().ok();
    let database_url = env::var("REDIS_URL").unwrap();
    let redis_config = Config::from_url(database_url);
    redis_config.create_pool(Some(Runtime::Tokio1)).unwrap()
}
