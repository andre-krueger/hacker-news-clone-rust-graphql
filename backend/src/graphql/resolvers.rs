#![deny(unused_must_use)]

use crate::error::{IncorrectLoginCredentials, UserNotFound};
use crate::graphql::schema::{Role, RoleGuard, User, UserData, UserNotFound2, UserResult};
use argon2::PasswordHash;
use argon2::{Argon2, PasswordVerifier};
// use async_graphql::guard::Guard;
use async_graphql::{
    connection, Context, Error, ErrorExtensionValues, ErrorExtensions, FieldError, FieldResult,
    Guard, Object, OutputType, Result, ResultExt, SimpleObject, ID,
};
use sqlx::postgres::PgRow;
use sqlx::{FromRow, PgPool, Postgres, Row};
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::{RwLock, RwLockWriteGuard};
use warp_sessions::Session;

use async_graphql::connection::{query, Connection, Edge, EmptyFields, PageInfo};
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

#[derive(SimpleObject)]
struct ConnectionFields {
    total_count: i32,
}

#[derive(SimpleObject)]
struct Diff {
    diff: i32,
}

#[derive(SimpleObject)]
struct MyConnection<T: Sync + std::marker::Send + OutputType> {
    edges: Vec<Edge<usize, T, EmptyFields>>,
    totalCount: usize,
    page_info: PageInfo,
}

macro_rules! query_with {
    ($entity:ident,$pool:expr, $query:literal, $table_name:literal) => {
        // sqlx::query_as!($entity, "select * from " + $table_name)
        //     .fetch_all($pool)
        //     .await
        //     .unwrap()
        query_with!($entity, $pool, $query, $table_name, "")
    };
    ($entity:ident,$pool:expr, $query:literal,$table_name:literal, $join:expr) => {{
        let rows = sqlx::query_as!($entity, $query + r#" FROM "# + $table_name + $join,)
            .fetch_all($pool)
            .await
            .unwrap();
        let mut edges = rows
            .into_iter()
            .enumerate()
            .map(|(index, item)| Edge::new(item.id as usize, item))
            .collect::<Vec<Edge<usize, $entity, EmptyFields>>>();
        // let mut connection = Connection::with_additional_fields(false, false, EmptyFields);
        // let mut connection = Connection::new(false, false);
        let connection = MyConnection {
            edges,
            totalCount: 10,
            page_info: PageInfo {
                has_previous_page: false,
                has_next_page: false,
                start_cursor: Some("".to_string()),
                end_cursor: Some("".to_string()),
            },
        };
        Ok::<MyConnection<$entity>, Error>(connection)
    }};
}

async fn paginate<T: for<'a> FromRow<'a, PgRow> + std::marker::Send + Unpin + OutputType>(
    pool: &PgPool,
) -> Result<MyConnection<T>> {
    let rows: Vec<T> =
        // sqlx::query_as::<_, T>(r#"select id,username,created_at,updated_at from users"#)
        sqlx::query_as::<_, T>(r#"
SELECT users.id as id, created_at, role_name as "role!: Role", username, updated_at
FROM users
         INNER JOIN user_roles ON users.id = user_roles.user_id
         INNER JOIN roles on user_roles.user_id = roles.id
"#
        )
            .fetch_all(&*pool)
            .await.unwrap();
    let mut edges = rows
        .into_iter()
        .enumerate()
        .map(|(index, item)| Edge::new(1 as usize, item))
        .collect::<Vec<Edge<usize, T, EmptyFields>>>();
    // let mut connection = Connection::with_additional_fields(false, false, EmptyFields);
    // let mut connection = Connection::new(false, false);
    let connection = MyConnection {
        edges,
        totalCount: 10,
        page_info: PageInfo {
            has_previous_page: false,
            has_next_page: false,
            start_cursor: Some("".to_string()),
            end_cursor: Some("".to_string()),
        },
    };
    Ok(connection)
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

    async fn numbers(
        &self,
        context: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<MyConnection<User>> {
        let pool = context.data::<PgPool>().unwrap();
        println!("nntnt");
        query_with!(
            User,
            pool,
            r#"
SELECT users.id as "id!" , created_at as "created_at!",roles.role_name as "role!: Role", username as "username!", updated_at as "updated_at!"
        "#,
            "users",
            r#"
         INNER JOIN user_roles ON users.id = user_roles.user_id
         INNER JOIN roles on user_roles.role_id = roles.id
            "#
        )
        // paginate::<User>(pool).await
        // Ok(1)
        // connection::query(
        //     after,
        //     before,
        //     first,
        //     last,
        //     |after, before, first, last| async move {
        //         let mut start = after.map(|after| after + 1).unwrap_or(0);
        //         let mut end = before.unwrap_or(10000);
        //         if let Some(first) = first {
        //             end = (start + first).min(end);
        //         }
        //         if let Some(last) = last {
        //             start = if last > end - start { end } else { end - last };
        //         }
        //         let mut connection = Connection::with_additional_fields(
        //             start > 0,
        //             end < 10000,
        //             ConnectionFields { total_count: 10000 },
        //         );
        //         connection.append((start..end).map(|n| {
        //             Edge::with_additional_fields(
        //                 n,
        //                 n as i32,
        //                 Diff {
        //                     diff: (10000 - n) as i32,
        //                 },
        //             )
        //         }));
        //         Ok::<_, Error>(connection)
        //     },
        // )
        // .await
    }
    async fn cool(&self, doErr: Option<bool>) -> Result<UserResult> {
        if (doErr.is_some() == true) {
            Ok(UserResult::UserNotFound2(UserNotFound2 {
                message: "Not Found".to_string(),
            }))
        } else {
            Ok(UserResult::UserData(UserData { id: 10 }))
        }
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

    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    async fn delete_user(
        &self,
        ctx: &Context<'_>,
        id: ID,
    ) -> Result<i32, IncorrectLoginCredentials> {
        let pool = ctx.data::<PgPool>().unwrap();
        Ok(0)
    }
}
