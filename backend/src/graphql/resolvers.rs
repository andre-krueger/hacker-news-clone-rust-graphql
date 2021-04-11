use crate::error::IncorrectLoginCredentials;
use crate::graphql::schema::User;
use argon2::PasswordHash;
use argon2::{Argon2, PasswordVerifier};
use async_graphql::{Context, Object};
use sqlx::PgPool;

pub struct QueryRoot;
#[Object]
impl QueryRoot {
    async fn stub(&self, ctx: &Context<'_>) -> Result<i32, &str> {
        Ok(0)
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn login(
        &self,
        ctx: &Context<'_>,
        username: String,
        password: String,
    ) -> Result<User, IncorrectLoginCredentials> {
        let pool = ctx.data::<PgPool>().unwrap();
        let user = match sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username)
            .fetch_one(&*pool)
            .await
        {
            Ok(user) => user,
            _ => return Err(IncorrectLoginCredentials),
        };
        let argon2 = Argon2::default();
        let parsed_hash = PasswordHash::new(&user.password).unwrap();
        return match argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(_) => Ok(user),
            _ => Err(IncorrectLoginCredentials),
        };
    }
}
