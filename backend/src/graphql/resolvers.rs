#![deny(unused_must_use)]
use crate::error::{IncorrectLoginCredentials, UserNotFound};
use crate::graphql::schema::{Role, RoleGuard, User};
use argon2::PasswordHash;
use argon2::{Argon2, PasswordVerifier};
use async_graphql::guard::Guard;
use async_graphql::{
    Context, Error, ErrorExtensionValues, ErrorExtensions, FieldError, FieldResult, Object,
    ResultExt, ID,
};
use sqlx::postgres::PgRow;
use sqlx::{PgPool, Row};
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::{RwLock, RwLockWriteGuard};
use warp_sessions::Session;

use core::fmt;
use std::fmt::Formatter;
use std::num::ParseIntError;
use thiserror::Error;

pub struct QueryRoot;

#[derive(Debug, Error)]
pub enum ResolverError {
    #[error("User not found")]
    UserNotFound(#[from] sqlx::Error),
    // #[error("st")]
    // LUserNotFound(#[from] std::num::ParseIntError),
    // #[error("bbb")]
    // Bla(#[from] std::num::ParseIntError),
    #[error("nnt")]
    Cool,
}

impl ErrorExtensions for ResolverError {
    // lets define our base extensions
    fn extend(&self) -> FieldError {
        self.extend_with(|err, e| match err {
            ResolverError::UserNotFound(error) => {
                println!("net");
                e.set("code", "USER_NOT_FOUND");
                e.set("reason", error.to_string())
            }
            _ => {} // MyError::ServerError(reason) => e.set("reason", reason.to_string()),
                    // MyError::ErrorWithoutExtensions => {}
        })
    }
}

#[Object]
impl QueryRoot {
    async fn user(&self, ctx: &Context<'_>, id: ID) -> FieldResult<User> {
        let pool = ctx.data::<PgPool>().unwrap();
        let mut user: Result<User, sqlx::Error>; //: Result<User, _>;
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
            .fetch_one(&*pool)
            .await;
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
        let bx = ErrorExtensionValues::default();
        // let x = Ok("te".parse()?);
        // Ok("t".parse()).map_err(|e: Error| e.extend())
        // let nnnn = user.map_err(|e| e.extend());
        // Ok::<i32, Error>("t".parse().extend()?)
        // Err(ResolverError::Cool.extend())
        // Ok("t".parse().map_err(|e: ResolverError| e.extend())?)
        Ok(user.map_err(|e| ResolverError::UserNotFound(e).extend())?)
        // Ok::<User, Result<User, ResolverError>>(user?).extend()
        // Ok(user).map_err(|err: ResolverError| {
        //     Err("nt".into());
        //     err.extend()
        // })?
        // Err("n".into()).map_err(|e: ResolverError| e.extend()?)
        // let nn = Ok(user).map_err(|e: ResolverError| e.extend());
        // nn
        // Err(ResolverError::Cool).map_err(|e| e.extend())
        // Err(2).map_err(|e: ResolverError| e.extend())
        // x.map_err(|e: ResolverError| e.extend())
        // x?.map_err(|e: ResolverError| e.extend_with(|_, e| e.set("nt", "nt")))
        // Ok("2l".parse().extend())?
        // Ok::<User, ResolverError>(user?).extend()
        // return match user {
        //     Ok(user) => Ok(user),
        //     _ => return Err(ResolverError::UserNotFound), // _ => return Err(NotFoundError::NotFound.extend()),
        // };
    }
}

pub struct MutationRoot;

#[must_use]
async fn query_something(pool: &PgPool) {
    // let x: Result<(), ()> = Err(());
    // x.is_ok();
    let x = sqlx::query(r#"ISERT INTO roles (role_name) VALUES('admin')"#)
        .fetch_one(&*pool)
        .await;
}

#[must_use]
async fn bla(pool: &PgPool) {
    query_something(pool).await;
}

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
        bla(pool).await;
        let mut user_password;
        let user = match sqlx::query_file!(
            "src/database/queries/user_by_username_with_role.sql",
            username
        )
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
                    role: user.role,
                }
            }
            _ => return Err(IncorrectLoginCredentials),
        };
        let argon2 = Argon2::default();
        let parsed_hash = PasswordHash::new(&user_password).unwrap();
        return match argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(_) => {
                let mut session: RwLockWriteGuard<Session> =
                    ctx.data::<Arc<RwLock<Session>>>().unwrap().write().await;
                session.insert("user", user.id).unwrap();
                Ok(user)
            }
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
