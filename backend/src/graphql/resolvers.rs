use crate::error::{IncorrectLoginCredentials, UserNotFound};
use crate::graphql::schema::{Role, RoleGuard, User};
use argon2::PasswordHash;
use argon2::{Argon2, PasswordVerifier};
use async_graphql::guard::Guard;
use async_graphql::{Context, Object, ID};
use sqlx::postgres::PgRow;
use sqlx::{PgPool, Row};
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::{RwLock, RwLockWriteGuard};
use warp_sessions::Session;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn user(&self, ctx: &Context<'_>, id: ID) -> Result<User, UserNotFound> {
        let pool = ctx.data::<PgPool>().unwrap();
        let mut user;
        if ctx.look_ahead().field("role").exists() {
            user = sqlx::query_file!(
                "src/database/queries/user_by_id_with_role.sql",
                str::parse::<i32>(&id).unwrap()
            )
            .map(|data| User {
                id: data.id,
                username: data.username,
                created_at: data.created_at,
                updated_at: data.updated_at,
                role: data.role,
            })
            // .map(|row: PgRow| User {
            //     id: row.get("id"),
            //     role: row.get("role_name"),
            //     created_at: row.get("created_at"),
            //     updated_at: row.get("updated_at"),
            //
            //     username: row.get("username"),
            // })
            .fetch_one(&*pool)
            .await;
            println!("test");
        } else {
            user = sqlx::query_file!(
                "src/database/queries/user_by_id.sql",
                str::parse::<i32>(&id).unwrap()
            )
            .map(|data| User {
                id: data.id,
                username: data.username,
                created_at: data.created_at,
                updated_at: data.updated_at,
                role: Role::Guest,
            })
            .fetch_one(&*pool)
            .await;
        }
        return match user {
            Ok(user) => Ok(user),
            _ => return Err(UserNotFound),
        };
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    // Quick test with a Rust docstring to showcase the documentation feature in GraphQL Playground
    /// Returns a user if the user exists with the given credentials
    ///
    /// # Arguments
    /// * `username` - The user's username
    /// * `password` - A strong password that was chosen for the user, which is hopefully not hunter2
    async fn login(
        &self,
        ctx: &Context<'_>,
        username: String,
        password: String,
    ) -> Result<User, IncorrectLoginCredentials> {
        let pool = ctx.data::<PgPool>().unwrap();
        let mut session: RwLockWriteGuard<Session> =
            ctx.data::<Arc<RwLock<Session>>>().unwrap().write().await;
        session.insert("user", "1".to_string()).unwrap();
        let mut user_password;
        let user = match sqlx::query!("SELECT * FROM users WHERE username = $1", username)
            .fetch_one(&*pool)
            .await
        {
            Ok(user) => {
                user_password = user.password;
                User {
                    id: user.id,
                    username: user.username,
                    created_at: user.created_at,
                    updated_at: user.updated_at,
                    // TODO: do a join for the user role
                    role: Role::Admin,
                }
            }
            _ => return Err(IncorrectLoginCredentials),
        };
        let argon2 = Argon2::default();
        let parsed_hash = PasswordHash::new(&user_password).unwrap();
        return match argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(_) => Ok(user),
            _ => Err(IncorrectLoginCredentials),
        };
    }

    #[graphql(guard(RoleGuard(role = "Role::Admin")))]
    async fn delete_user(
        &self,
        ctx: &Context<'_>,
        id: ID,
    ) -> Result<i32, IncorrectLoginCredentials> {
        let pool = ctx.data::<PgPool>().unwrap();
        Ok(0)
    }
}
