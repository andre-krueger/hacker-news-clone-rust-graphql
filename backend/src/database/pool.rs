use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

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
