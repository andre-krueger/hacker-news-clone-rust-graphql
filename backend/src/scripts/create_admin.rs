use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};
use backend::database::pool::init_pool;
use rand_core::OsRng;
use std::env;

#[tokio::main]
async fn main() {
    let pool = init_pool().await;

    let args: Vec<String> = env::args().collect();

    let username = &args[1];
    let password = &args[2];

    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password_simple(password.as_bytes(), salt.as_ref())
        .unwrap()
        .to_string();

    let id = sqlx::query!(
        "INSERT INTO users(username, password) VALUES ($1, $2) RETURNING ID;",
        username,
        password_hash
    )
    .fetch_one(&pool)
    .await
    .unwrap()
    .id;

    sqlx::query!(
        "INSERT INTO user_roles(user_id, role_id) VALUES ($1, $2)",
        id,
        1
    )
    .execute(&pool)
    .await
    .unwrap();
}
