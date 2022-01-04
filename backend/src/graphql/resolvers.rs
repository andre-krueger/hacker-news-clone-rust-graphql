#![deny(unused_must_use)]

use crate::error::{IncorrectLoginCredentials, UserNotFound};
use crate::graphql::schema::{
    OrderBy, PaginationIncorrect, PaginationVecString, Role, RoleGuard, User, UserColumns,
    UserData, UserNotFound2, UserResult,
};
use argon2::PasswordHash;
use argon2::{Argon2, PasswordVerifier};
use serde::{Deserialize, Serialize};
// use async_graphql::guard::Guard;
use async_graphql::{
    connection, scalar, Context, ErrorExtensionValues, ErrorExtensions, FieldError, FieldResult,
    Guard, InputType, InputValueError, InputValueResult, Object, OutputType, Result, ResultExt,
    SimpleObject, Upload, Value, ID,
};
use phf::phf_map;
use sqlx::postgres::PgRow;
use sqlx::{FromRow, PgPool, Postgres, Row};
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::{RwLock, RwLockWriteGuard};
use warp_sessions::Session;

use async_graphql::connection::{query, Connection, Edge, EmptyFields, PageInfo};
use async_graphql::registry::Registry;
use core::fmt;
use std::borrow::Cow;
use std::convert::Infallible;
use std::fmt::{Display, Formatter};
use std::io::Empty;
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

#[derive(async_graphql::Union)]
enum PaginationVecValues {
    PaginationVecString(PaginationVecString), // String(String),
                                              // Int(Int)
}

#[derive(SimpleObject)]
pub struct ConnectionFields {
    total_count: usize,
    //  pagination_type: UserColumns,
    pagination_vec: Vec<String>,
}

#[derive(SimpleObject)]
struct Diff {
    diff: i32,
}

// #[derive(SimpleObject)]
// struct MyConnection<T: Sync + std::marker::Send + OutputType> {
//     edges: Vec<Edge<usize, T, EmptyFields>>,
//     totalCount: usize,
//     page_info: PageInfo,
// }

async fn query_fn(pool: &PgPool, limit: Option<i32>, skip: Option<i32>) -> ConnectionResult<User> {
    let has_previous_page = false;
    let mut has_next_page = false;
    let limitstring = match limit {
        Some(l) => format!("LIMIT {}", l + 1),
        _ => "".to_string(),
    };
    let offsetstring = match skip {
        Some(l) => format!("OFFSET {}", l),
        _ => "".to_string(),
    };
    let mut rows: Vec<User> = sqlx::query_as::<_, User>(
&*format!(        r#"
SELECT users.id as id, created_at, role_name as role, username, updated_at
FROM users
 INNER JOIN user_roles ON users.id = user_roles.user_id INNER JOIN roles on user_roles.role_id = roles.id 
  {} {}
    "#, limitstring, offsetstring),
    )
    .fetch_all(pool)
    .await?;
    if let Some(i) = limit {
        if (rows.len() > i as usize) {
            has_next_page = true;
        }
    }
    rows = rows[0..std::cmp::max(rows.len() - 1, 0)].to_vec();
    // let mut connection = Connection::with_additional_fields(
    //     has_previous_page,
    //     has_next_page,
    //     ConnectionFields {
    //         total_count: 0,
    //         pagination_vec: vec![],
    //     },
    // );

    // let mut connection: Connection<String, T, ConnectionFields, _> =
    //     BlaConnection(Connection::with_additional_fields(
    //         has_previous_page,
    //         has_next_page,
    //         ConnectionFields {
    //             total_count: 0,
    //             pagination_vec: vec![],
    //         },
    //     ));

    let mut edges = rows
        .into_iter()
        .enumerate()
        .map(|(index, item)| Edge::new(item.created_at.to_string(), item))
        .collect::<Vec<Edge<_, User, EmptyFields>>>();
    // .collect::<Vec<Edge<_, $entity, EmptyFields>>>();

    let mut pagination_vec: Vec<String> = sqlx::query(r#"
SELECT created_at
FROM users
 INNER JOIN user_roles ON users.id = user_roles.user_id INNER JOIN roles on user_roles.role_id = roles.id 
    "#)
        .fetch_all(pool)
        .await?
        .iter()
        .map(|r| {
            r.get::<chrono::DateTime<chrono::Utc>, _>("created_at")
                .to_string()
        })
        .collect();

    let mut connection = Connection::with_additional_fields(
        has_previous_page,
        has_next_page,
        ConnectionFields {
            total_count: edges.len(),
            pagination_vec,
        },
    );
    connection.append(edges);
    // Ok::<MyConnection<$entity>, Error>(connection)
    Ok(connection)
}

#[derive(async_graphql::MergedObject)]
#[graphql(name = "BlaConnection")]
struct BlaConnection<T: Send + Sync + OutputType>(
    Connection<String, T, ConnectionFields, EmptyFields>,
);

pub type ConnectionResult<T> = Result<Connection<String, T, ConnectionFields, EmptyFields>>;

macro_rules! query_with {
    ($entity:ident,$pool:expr, $query:literal, $table_name:literal, $id_column:expr,$after:expr, $before:expr, $first:expr,$last:expr,$skip:expr,$back:expr,$page:expr) => {
        // sqlx::query_as!($entity, "select * from " + $table_name)
        //     .fetch_all($pool)
        //     .await
        //     .unwrap()
        query_with!(
            $entity,
            $pool,
            $query,
            $table_name,
            $id_column,
            $after,
            $before,
            $first,
            $last,
            Some(false),
            false,
            1,
            "",
            "",
            "",
            ""
        )
    };
    // ($entity:ident,$pool:expr, $query:literal, $table_name:literal, $id_column:expr,$after:expr, $before:expr, $first:expr,$last:expr,$join:expr) => {
    //     // sqlx::query_as!($entity, "select * from " + $table_name)
    //     //     .fetch_all($pool)
    //     //     .await
    //     //     .unwrap()
    //     query_with!(
    //         $entity,
    //         $pool,
    //         $query,
    //         $table_name,
    //         $id_column,
    //         $after,
    //         $before,
    //         $first,
    //         $last,
    //         $join,
    //         r#""id!""#,
    //         "ASC",
    //     )
    // };
    ($entity:ident,$pool:expr, $query:literal,$table_name:literal, $id_column:expr, $after:expr, $before:expr, $first:expr, $last:expr,$skip:expr,$back:expr,$page:expr, $join:expr, $order_by_column:expr, $order_by: expr,$filter:expr) => {{


                // let mut pagination_type =  match $order_by_column {
                //     Some(order_by_column)=>order_by_column,
                //     _=>$id_column
// };

        if let Some(_first) = $first {
            if _first < 0 {

                // return Ok(UserResult::PaginationIncorrect(
                //     PaginationIncorrect::default(), // message: "n".to_string(),
                // ));
            }
        }
        let offset = match $skip {
            Some(v)=> format!(" OFFSET {} ", v),
            _ => "".to_string()
        };
        // let mut total_count=0;
        let mut rows = vec![];
        let mut has_previous_page = false;
        let mut has_next_page = false;
        let mut should_reverse = false;
        let total_count: (i64,) = sqlx::query_as(&*format!("SELECT COUNT(*) from {} {}", $table_name, $filter))
            .fetch_one($pool)
            .await?;
                let order_by_text =  if $order_by_column.is_empty() {
                    ""
                } else {
                    "ORDER BY"
                };

        match ($after, $before, $first, $last) {
            (None, None, Some(first), None) => {
                rows = sqlx::query_as::<_, $entity>(
                    &*format!("SELECT * FROM ({} FROM {} {} ORDER BY {} ASC {}) AS _ {} {} {} {} LIMIT $1",
                    $query,
                    $table_name,
                    $join,
                    $id_column,
                    offset,
                        $filter,
                    order_by_text,
                        $order_by_column,
                        $order_by
                    )
                ).bind(first)
                .fetch_all($pool)
                .await?;

 // total_count=rows.len();
 // rows = rows[0 .. std::cmp::min(first as usize, total_count)].to_vec();
                should_reverse = false;
                has_previous_page = false;
                // has_next_page = total_count.0 as usize > first as usize;
            }
            (None, None, None, Some(last)) => {
                if ($filter.is_empty()) {
                    rows = sqlx::query_as::<_, $entity>(
                    &*format!("SELECT * FROM ({} FROM {} {} ORDER BY {} DESC LIMIT $1) AS _ {} {} {} {}",
                    $query,
                    $table_name,
                    $join,
                    $id_column,
                        $filter,
                    order_by_text,
                        $order_by_column,
                        $order_by
                    )
                ).bind(last)
                .fetch_all($pool)
                .await?;
                }
                else {
                rows = sqlx::query_as::<_, $entity>(
                    &*format!("SELECT * FROM ({} FROM {} {} ORDER BY {} DESC) AS _ {} {} {} {} LIMIT $1",
                    $query,
                    $table_name,
                    $join,
                    $id_column,
                        $filter,
                    order_by_text,
                        $order_by_column,
                        $order_by
                    )
                ).bind(last)
                .fetch_all($pool)
                .await?;
    }
                should_reverse = true;
                // has_previous_page = total_count.0 as usize >last as usize;
                has_next_page = false;
            }
            (Some(after), None,Some(first), None) =>{
 let _has_previous_page: (i64,) =
                        sqlx::query_as(
 &*format!("SELECT COUNT(*) from {} where {} < $1::timestamp", $table_name, $id_column)
 )
                            // .bind(&after.parse::<i32>().unwrap() )
 .bind(&after)
                            .fetch_one($pool)
                            .await?;
let operator = match $back {
  Some(true) => "<".to_string(),
     _ => ">".to_string()
};
let orderby = match $back {
  Some(true) => "DESC".to_string(),
     _ => "ASC".to_string()
};
 let bb = match $back{
     Some(true) => -10*first,
     _ => 10*first
 };
                rows = sqlx::query_as::<_, $entity>(
                    &*format!("SELECT * FROM ({} FROM {} {} WHERE {} {} $1::timestamp ORDER BY {} {} LIMIT $2 {}) AS _ {} {} {} {}",
                    $query,
                    $table_name,
                    $join,
                    $id_column,
                    operator,
                    $id_column,
                    orderby,
                        offset,
                        $filter,
                    order_by_text,
                        $order_by_column,
                        $order_by
                    )
                )
                // .bind(&after)
 // .bind(&after.parse::<i32>()?)
 .bind(after)
                .bind(first)
                .fetch_all($pool)
                .await?;
 // total_count=rows.len();//std::cmp::max(rows.len(), (first + bb) as usize);//rows.len()+(first as usize) ;
 // rows = rows[0 .. std::cmp::min(first as usize, total_count)].to_vec();
 //                should_reverse = false;
 if let Some(bb) =&rows.last() {
                 // has_next_page = (bb.id as i64) <total_count.0 ;
 } else {
     has_next_page=false;
 }
                // has_next_page = (*bb  as i64) <total_count.0 ;
                has_previous_page= _has_previous_page.0 > 0 && !&rows.is_empty();
                // has_previous_page = total_count.0 as usize >last as usize;
                // has_next_page = false;
            }
            (None,Some(before),None,Some(last))=>{
 let _has_next_page: (i64,) =
                        sqlx::query_as(
 &*format!("SELECT COUNT(*) from {} where {} >= $1::timestamp", $table_name, $id_column)
 )
                            // .bind(&after.parse::<i32>().unwrap() )
 .bind(&before)
                            .fetch_one($pool)
                            .await?;
                has_next_page= _has_next_page.0 > 0;
                // has_previous_page = total_count.0 as usize >last as usize;
                should_reverse=true;

                rows = sqlx::query_as::<_, $entity>(
                    &*format!("SELECT * FROM ({} FROM {} {} WHERE {} < $1::timestamp ORDER BY {} DESC) AS _ {} {} {} {} LIMIT $2",
                    $query,
                    $table_name,
                    $join,
                    $id_column,
                    $id_column,
                        $filter,
                    order_by_text,
                        $order_by_column,
                        $order_by
                    )
                )
                // .bind(&after)
 .bind(before)
                .bind(last)
                .fetch_all($pool)
                .await?;
            }
            (None,None,None,None) => {

                has_next_page= false;
                has_previous_page = false;
                should_reverse=false;
                rows = sqlx::query_as::<_, $entity>(
                    &*format!("{} FROM {} {} {} {} {} {}",
                    $query,
                    $table_name,
                    $join,
                        $filter,
                    order_by_text,
                        $order_by_column,
                        $order_by
                    )
                )
                .fetch_all($pool)
                .await?;
            }
            _ => {
                // return Ok(UserResult::PaginationIncorrect(
                //     PaginationIncorrect::default(), // message: "n".to_string(),
                // ));
            } // Ok(UserResult::UserNotFound2(UserNotFound2 {
              //     message: "Not Found".to_string(),
              // }))
        }

        let mut edges = rows
            .into_iter()
            .enumerate()
            .map(|(index, item)| Edge::new(item.created_at.to_string(), item))
            .collect::<Vec<Edge<_, $entity, EmptyFields>>>();
        // let mut connection = Connection::with_additional_fields(false, false, EmptyFields);
        // let mut connection = Connection::new(false, false);
        // let connection = MyConnection {
        //     edges,
        //     totalCount: 10,
        //     page_info: PageInfo {
        //         has_previous_page: false,
        //         has_next_page: false,
        //         start_cursor: Some("".to_string()),
        //         end_cursor: Some("".to_string()),
        //     },
        // };
        // let total_count=*&rows.len() as i64;
        if should_reverse {
            edges.reverse();
        }
        let mut pagination_vec =sqlx::query("select created_at from users").fetch_all($pool).await?.iter().map(|r|r.get::<chrono::DateTime<chrono::Utc>,_>("created_at").to_string()).collect();
        // pagination_vec.push("test".to_string());
        // pagination_vec.push( PaginationVecValues::PaginationVecString(PaginationVecString {
        //     val: "tet".to_string(),
        // }));
        let mut connection = Connection::with_additional_fields(
            has_previous_page,
            has_next_page,
            ConnectionFields {
                total_count:total_count.0 as usize,
                // pagination_type,
                pagination_vec
                // total_count:std::cmp::min(total_count.0,edges.len() as i64)
            },
        );

        connection.append(edges);
        // Ok::<MyConnection<$entity>, Error>(connection)
        // Ok(UserResult::Connection(connection))
        Ok(connection)
    }};
}

// use crate::graphql::schema::UserResult::Connection;
use regex::Regex;

#[derive(async_graphql::InputObject)]
pub struct UsersFilterInput {
    pub not: Box<Option<UsersFilterInput>>,
    pub and: Option<Vec<UsersFilterInput>>,
    pub or: Option<Vec<UsersFilterInput>>,
    pub username: Option<Condition<String>>,
    pub id: Option<Condition<i32>>,
    pub created_at: Option<Condition<String>>,
}

#[derive(async_graphql::InputObject)]
#[graphql(concrete(name = "IntegerCondition", params(i32)))]
#[graphql(concrete(name = "StringCondition", params(String)))]
pub struct Condition<T: InputType> {
    pub equals: Option<T>,
    pub like: Option<T>,
    pub greater_than: Option<T>,
    pub less_than: Option<T>,
    #[graphql(name = "in")]
    pub inside: Option<Vec<T>>,
}

impl std::fmt::Display for UsersFilterInput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(n) = &self.username {
            write!(f, "username {} AND ", n)?;
        }
        if let Some(n) = &self.created_at {
            write!(f, " created_at {} AND ", n)?;
        }
        if let Some(n) = &self.id {
            write!(f, "  id {} AND ", n)?;
        }

        match self {
            UsersFilterInput { and: Some(and), .. } => {
                for b in and {
                    let mut x = "".to_string();
                    if let UsersFilterInput { id: Some(id), .. } = &b {
                        x = format!("id {} AND {} ", id, x).to_string();
                    }
                    if let UsersFilterInput {
                        username: Some(username),
                        ..
                    } = &b
                    {
                        x = format!(" username {} AND {}", username, x).to_string();
                    }
                    write!(f, "{}", &x)?;
                }
            }
            _ => write!(f, "")?,
        }
        match self {
            UsersFilterInput { or: Some(or), .. } => {
                for b in or {
                    let mut x = "".to_string();
                    if let UsersFilterInput { id: Some(id), .. } = &b {
                        x = format!("id {} OR {} ", id, x).to_string();
                    }
                    if let UsersFilterInput {
                        username: Some(username),
                        ..
                    } = &b
                    {
                        x = format!(" username {} OR {}", username, x).to_string();
                    }
                    if let UsersFilterInput {
                        created_at: Some(created_at),
                        ..
                    } = &b
                    {
                        x = format!(" created_at {} OR {}", created_at, x).to_string();
                    }
                    write!(f, "{}", &x)?;
                }
            }
            _ => write!(f, "")?,
        }
        write!(f, "")
    }
}

impl<T: InputType + std::fmt::Display> std::fmt::Display for Condition<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Condition {
                equals: Some(equals),
                ..
            } => write!(f, " = '{}'", equals),
            Condition {
                like: Some(like), ..
            } => write!(f, " LIKE '{}'", like),
            Condition {
                greater_than: Some(greater_than),
                ..
            } => write!(f, " > '{}'", greater_than),
            Condition {
                less_than: Some(less_than),
                ..
            } => write!(f, " < '{}'", less_than),
            Condition {
                inside: Some(inside),
                ..
            } => write!(
                f,
                " in ('{}')",
                inside
                    .into_iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            _ => write!(f, ""),
        }
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
                str::parse::<i32>(&id)?
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
                str::parse::<i32>(&id)?
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

    async fn numbers2(
        &self,
        context: &Context<'_>,
        limit: Option<i32>,
        skip: Option<i32>,
    ) -> ConnectionResult<User> {
        let pool = context.data::<PgPool>().unwrap();
        query_fn(pool, limit, skip).await
    }

    // async fn numbers2(
    //     &self,
    //     context: &Context<'_>,
    // ) -> Result<Connection<usize, User, ConnectionFields, EmptyFields>> {
    //     let pool = context.data::<PgPool>().unwrap();
    //     query_fn(pool)
    // }

    async fn numbers(
        &self,
        context: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i64>,
        last: Option<i64>,
        order_by_column: Option<UserColumns>,
        order_by: Option<OrderBy>,
        filter: Option<UsersFilterInput>,
        skip: Option<i32>,
        back: Option<bool>,
        page: Option<i32>,
    ) -> ConnectionResult<User>
// Result<Connection<String, User, ConnectionFields, EmptyFields>>
    {
        let pool = context.data::<PgPool>().unwrap();
        let _filter_string: String = filter.map(|c| c.to_string()).unwrap_or_default();
        let regex = Regex::new(r"(AND|OR)$")?;
        let filter_string = if _filter_string.len() > 0 {
            format!(
                "WHERE {}",
                regex.replace(&_filter_string.trim(), "").to_string()
            )
        } else {
            "".to_string()
        };
        query_with!(
            User,
            pool,
            r#"
        SELECT users.id as id , created_at ,role_name as role, username , updated_at
                "#,
            "users",
            // "users.created_at",
            UserColumns::CreatedAt,
            after,
            before,
            first,
            last,
            skip,
            back,
            page,
            r#" INNER JOIN user_roles ON users.id = user_roles.user_id INNER JOIN roles on user_roles.role_id = roles.id "#,
            &order_by_column.unwrap_or(UserColumns::Id).to_string(),
            // order_by_column,
            &order_by.unwrap_or(OrderBy::ASC).to_string(),
            filter_string
        )
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

    // async fn numbers(
    //     &self,
    //     after: Option<String>,
    //     before: Option<String>,
    //     first: Option<i32>,
    //     last: Option<i32>,
    // ) -> Result<Connection<String, i32, EmptyFields, EmptyFields>> {
    //     query::<String, i32, EmptyFields, _, _, _, Infallible>(
    //         after,
    //         before,
    //         first,
    //         last,
    //         |after, before, first, last| async move {
    //             // let mut start = after.map(|after| after + 1).unwrap_or(0);
    //             // let mut end = before.unwrap_or(10000);
    //             // if let Some(first) = first {
    //             //     end = (start + first).min(end);
    //             // }
    //             // if let Some(last) = last {
    //             //     start = if last > end - start { end } else { end - last };
    //             // }
    //             let mut connection: Connection<String, i32> = Connection::new(false, false);
    //             connection.append(
    //                 (0..1)
    //                     .into_iter()
    //                     .map(|n| Edge::new("test".to_string(), n as i32)),
    //             );
    //             Ok(connection)
    //         },
    //     )
    //     .await
    // }
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

    async fn logout(&self, ctx: &Context<'_>) -> Result<i32> {
        let mut session: RwLockWriteGuard<Session> =
            ctx.data::<Arc<RwLock<Session>>>()?.write().await;
        session.remove("user");
        Ok(1)
    }

    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    async fn upload(&self, ctx: &Context<'_>, file: Upload) -> bool {
        // println!("upload: filename={}", file.value(ctx).unwrap().filename);
        true
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
