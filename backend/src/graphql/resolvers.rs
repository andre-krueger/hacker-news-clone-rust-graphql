#![deny(unused_must_use)]

use crate::error::{IncorrectLoginCredentials, UserNotFound};
use crate::graphql::schema::{
    OrderBy, PaginationIncorrect, Role, RoleGuard, User, UserColumns, UserData, UserNotFound2,
    UserResult,
};
use argon2::PasswordHash;
use argon2::{Argon2, PasswordVerifier};
use serde::{Deserialize, Serialize};
// use async_graphql::guard::Guard;
use async_graphql::{
    connection, scalar, Context, Error, ErrorExtensionValues, ErrorExtensions, FieldError,
    FieldResult, Guard, InputType, InputValueError, InputValueResult, Object, OutputType, Result,
    ResultExt, SimpleObject, Upload, Value, ID,
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
use std::fmt::{Display, Formatter};
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
pub struct ConnectionFields {
    total_count: i64,
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
    ($entity:ident,$pool:expr, $query:literal, $table_name:literal, $id_column:expr,$after:expr, $before:expr, $first:expr,$last:expr) => {
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
    ($entity:ident,$pool:expr, $query:literal,$table_name:literal, $id_column:expr, $after:expr, $before:expr, $first:expr, $last:expr, $join:expr, $order_by_column:expr, $order_by: expr,$filter:expr) => {{

        // let x =concat!("", cool_stuff!());
        // let xxxb =sqlx::query_as!($entity, concat!("")).fetch_all($pool).await;
        // let xxx = sqlx::query_as!($entity, concat!("select * from ",cool_stuff!())).fetch_all($pool).await;
        //      let x=   sqlx::query_as::<_, $entity>(&b).fetch_all($pool).await;
        if let Some(_first) = $first {
            if _first < 0 {

                return Ok(UserResult::PaginationIncorrect(
                    PaginationIncorrect::default(), // message: "n".to_string(),
                ));
            }
        }
        let mut rows = vec![];
        let mut has_previous_page = false;
        let mut has_next_page = false;
        let mut should_reverse = false;
        let total_count: (i64,) = sqlx::query_as(concat!("SELECT COUNT(*) from ", $table_name))
            .fetch_one($pool)
            .await?;

        match ($after, $before, $first, $last) {
            (None, None, Some(first), None) => {
                // rows = sqlx::query_as!(
                //
                //     $entity,
                //     "SELECT * from  ("
                //         + $query
                //         + " FROM "
                //         + $table_name
                //         + $join
                //         + " ORDER BY "
                //         + $id_column
                //         + " ASC LIMIT $1"
                //         // + r#") as bla order by "username!" ASC"#,
                //     + ") as _ "
                //     // + $order_by_column + " " + $order_by ,
                //     ,
                //     first
                // )
                rows = sqlx::query_as::<_, $entity>(
                    // &(
                    //     "SELECT * FROM (".to_owned()
                    //     + $query
                    //       + " FROM "
                    //     + $table_name
                    //     + $join
                    //     + " ORDER BY "
                    //     + $id_column
                    //     + " ASC LIMIT $1 ) AS _ ORDER BY  "
                    //     + $order_by_column + " " + $order_by
                    // )
                    &*format!("SELECT * FROM ({} FROM {} {} ORDER BY {} ASC LIMIT $1) AS _ {} ORDER BY {} {}",
                    $query,
                    $table_name,
                    $join,
                    $id_column,
                        $filter,
                        $order_by_column,
                        $order_by
                    )



                    // "SELECT * from  (".to_owned()
                    //     + $query
                    //     + " FROM "
                    //     + $table_name
                    //     + $join
                    //     + " ORDER BY "
                    //     + $id_column
                    //     + " ASC LIMIT $1"
                    //     // + r#") as bla order by "username!" ASC"#,
                    // + ") as _ order by " + $order_by_column + " " + $order_by
                ).bind(first)
                .fetch_all($pool)
                .await?;
                should_reverse = false;
                has_previous_page = false;
                has_next_page = total_count.0 as usize > first as usize;
            }
            // (None, None, None, Some(last)) => {
            //     rows = sqlx::query_as!(
            //         $entity,
            //         $query
            //             + " FROM "
            //             + $table_name
            //             + $join
            //             + " ORDER BY "
            //             + $id_column
            //             + " DESC LIMIT $1",
            //         last
            //     )
            //     // .bind(1)
            //     .fetch_all($pool)
            //     .await
            //     .unwrap();
            //     should_reverse = true;
            //     has_previous_page = total_count.0 as usize > last as usize;
            //     has_next_page = false
            // }
            // (Some(after), None, Some(first), None) => {
            //     // let _has_previous_page: (bool,) = sqlx::query_as(concat!(
            //     //     "SELECT COUNT(*) from ",
            //     //     $table_name,
            //     //     " where ",
            //     //     $id_column,
            //     //     " <= ?"
            //     // ))
            //     // .bind(after as u32)
            //     // .fetch_one(x)
            //     // .await
            //     // .unwrap();
            //     rows = sqlx::query_as!(
            //         $entity,
            //         $query
            //             + " FROM "
            //             + $table_name
            //             + $join
            //             + " where "
            //             + $id_column
            //             + " > $1 "
            //             + "ORDER BY "
            //             + $id_column
            //             + " ASC LIMIT $2",
            //         after.parse::<i32>().unwrap(),
            //         first,
            //     )
            //     .fetch_all($pool)
            //     .await
            //     .unwrap();
            //     should_reverse = false;
            //     // has_previous_page = total_count.0 as usize > last as usize;
            //     // has_next_page = false
            // }
            // _ =>
            _ => {
                return Ok(UserResult::PaginationIncorrect(
                    PaginationIncorrect::default(), // message: "n".to_string(),
                ));
            } // Ok(UserResult::UserNotFound2(UserNotFound2 {
              //     message: "Not Found".to_string(),
              // }))
        }

        let mut edges = rows
            .into_iter()
            .enumerate()
            .map(|(index, item)| Edge::new(item.id as usize, item))
            .collect::<Vec<Edge<usize, $entity, EmptyFields>>>();
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
        if should_reverse {
            edges.reverse();
        }
        let mut connection = Connection::with_additional_fields(
            has_previous_page,
            has_next_page,
            ConnectionFields {
                total_count: total_count.0,
            },
        );

        connection.append(edges);
        // Ok::<MyConnection<$entity>, Error>(connection)
        Ok(UserResult::Connection(connection))
    }};
}

macro_rules! cool_stuff {
    ($t: expr) => {
        "posts"
    };
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
            .await?;
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

extern crate derive_more;

// use the derives that you want in the file
use derive_more::Display;
use log::kv::ToValue;
use regex::Regex;

#[derive(Display)]
#[display(
    fmt = "{}{}",
    r#"match username {
Some(Cond{eq}) => ("username".to_owned()+"="+eq).to_string(),
None => "".to_string()
}"#,
    r#"match and {
Some(Stuff{username}) => (" AND username".to_owned()+"="+&username.eq).to_string(),
None => "".to_string()
}"#
)]
pub struct StuffAndOr {
    username: Option<Cond>,
    and: Option<Stuff>,
    or: Option<Stuff>, // bla: Option<i32>,
}

pub struct Stuff {
    username: Cond,
    // bla: Option<i32>,
}

pub struct Cond {
    eq: String,
}

#[derive(async_graphql::InputObject)]
// #[display(
//     fmt = "{}{}",
// //     r#"match id {
// // Some(Condition{equals}) => "id = 10".to_string(),
// // None => "".to_string()
// // }"#,
// //     r#"match username {
// // Some(Condition{equals}) => ("username".to_owned()+"="+equals).to_string(),
// // None => "".to_string()
// // }"#,
//     r#"match and {
// Some(MyInput{username:Some(Condition{equals:Some(equals)}),..}) => (" AND username".to_owned()+" = '" + &equals + "'").to_string(),
// Some(MyInput{id:Some(Condition{equals:Some(equals)}),..}) => (" AND id".to_owned()+" = "+ &equals.to_string()).to_string(),
// _ => "".to_string()
// }"#,
//     r#"match or {
// Some(MyInput{username:Some(Condition{equals:Some(equals)}),..}) => (" OR username".to_owned()+" = '" + &equals + "'").to_string(),
// Some(MyInput{id:Some(Condition{equals:Some(equals)}),..}) => (" OR id".to_owned()+" = "+ &equals.to_string()).to_string(),
// _ => "".to_string()
// }"#
// )]
pub struct MyInputWithAndOr {
    // id: Option<Condition<i32>>,
    // username: Option<Condition<String>>,
    and: Option<MyInput>,
    or: Option<MyInput>,
}

use field_types::{FieldName, FieldType};

#[derive(async_graphql::InputObject, FieldType, FieldName)]
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
    pub less_than: Option<T>, // #[display(fmt = ">", greater_than)]
                              // greater_than: T
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
            write!(f, "  id = '{}' AND ", n)?;
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
                    write!(f, "{}", &x)?;
                }
            }
            _ => write!(f, "")?,
        }
        write!(f, "")
        // write!(f, "{}", self)
    }

    // fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    //     println!("{:?}", self);
    //     match self {
    //         UsersFilterInput { and: Some(and), .. } => {
    //             // let x = and
    //             //     .iter()
    //             //     .map(|val| self.fmt(f))
    //             //     .collect::<Vec<String>>()
    //             //     .join("");
    //             // write!(f, "blatest {}", x.to_string())
    //             // self.fmt(f)
    //             // self.fmt("", f)
    //             write!(f, "cooltest")
    //         }
    //         UsersFilterInput {
    //             usernameLike: Some(usernameLike),
    //             ..
    //         } => write!(f, "username like {}", usernameLike),
    //         UsersFilterInput {
    //             idEq: Some(idEq), ..
    //         } => write!(f, "id = {}", idEq),
    //         _ => write!(f, "tes"),
    //     }
    //     // write!(f, "{:?}", self)
    //     // match self {
    //     //     UsersFilterInput { and: Some(and), .. } => write!(f, "bla"),
    //     //     UsersFilterInput {
    //     //         usernameLike: Some(usernameLike),
    //     //         ..
    //     //     } => write!(f, "WHERE username LIKE '{}'", usernameLike),
    //     //     _ => write!(f, ""),
    //     // }
    // }
}

#[derive(async_graphql::InputObject)]
// #[display(fmt = "id")]
pub struct MyInput {
    id: Option<Condition<i32>>,
    username: Option<Condition<String>>,
}

impl std::fmt::Display for MyInputWithAndOr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MyInputWithAndOr {
                and: Some(MyInput { id: Some(id), .. }),
                ..
            } => {
                write!(f, " AND id{}", id)
            }
            MyInputWithAndOr {
                and:
                    Some(MyInput {
                        username: Some(username),
                        ..
                    }),
                ..
            } => {
                // write!(f, " AND id = {}", equals)
                write!(f, " AND username{}", username)
            }
            MyInputWithAndOr {
                or: Some(MyInput { id: Some(id), .. }),
                ..
            } => {
                write!(f, " OR id{}", id)
            }
            MyInputWithAndOr {
                or:
                    Some(MyInput {
                        username: Some(username),
                        ..
                    }),
                ..
            } => {
                // write!(f, " AND id = {}", equals)
                write!(f, " OR username{}", username)
            }
            _ => write!(f, ""),
        }
        // write!(f, "{}", self.and)
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
            } => write!(f, " like '{}'", like),
            Condition {
                greater_than: Some(greater_than),
                ..
            } => write!(f, " > '{}'", greater_than),
            Condition {
                less_than: Some(less_than),
                ..
            } => write!(f, " less_than '{}'", less_than),
            _ => write!(f, ""),
        }
    }
}

// impl std::fmt::Display for MyInputWithAndOr {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         let mut x = 0;
//         match &self {
//             MyInputWithAndOr {
//                 username: Some(username),
//                 ..
//             } => {
//                 x += 1;
//                 write!(f, "username {} ", username)
//             }
//             MyInputWithAndOr { and: Some(and), .. } => write!(f, "andnnn"),
//             _ => {
//                 x += 1;
//                 println!("{:?}", x);
//                 write!(f, "")
//             }
//         }
//     }
// }

// impl<T: InputType + std::fmt::Display> std::fmt::Display for Condition<T> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         match &self {
//             Condition {
//                 equals: Some(equals),
//                 ..
//             } => {
//                 write!(f, "== '{}'", equals)
//             }
//             _ => {
//                 write!(f, "")
//             }
//         }
//         // write!(f, "{}", self.equals)
//     }
// }

pub enum Ozy {
    A,
}

// async fn query_fn<T>(
//     table_name: &str,
//     pool: &PgPool,
// ) -> Result<Connection<usize, T, ConnectionFields, EmptyFields>> {
//     let mut rows: Vec<T> = vec![];
//     let mut has_previous_page = false;
//     let mut has_next_page = false;
//
//     let mut edges = rows
//         .into_iter()
//         .enumerate()
//         .map(|(index, item)| Edge::new(item.id as usize, item))
//         .collect::<Vec<Edge<usize, T, EmptyFields>>>();
//
//     let total_count: (i64,) = sqlx::query_as(&*format!("SELECT COUNT(*) from {}", table_name))
//         .fetch_one(pool)
//         .await
//         .unwrap();
//
//     let mut connection = Connection::with_additional_fields(
//         has_previous_page,
//         has_next_page,
//         ConnectionFields {
//             total_count: total_count.0,
//         },
//     );
//
//     connection.append(edges);
//     // Ok::<MyConnection<$entity>, Error>(connection)
//     Ok(connection)
// }

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

    // async fn numbers2(
    //     &self,
    //     context: &Context<'_>,
    // ) -> Result<Connection<usize, User, ConnectionFields, EmptyFields>> {
    //     let pool = context.data::<PgPool>().unwrap();
    //     query_fn::<User>("users", pool).await
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
    ) -> Result<UserResult> {
        let pool = context.data::<PgPool>().unwrap();
        //         let bbb = ormx::conditional_query_as!(
        //             User,
        //             r#"SELECT users.id as id, created_at, role_name as "role: Role", username, updated_at
        // FROM users
        //          INNER JOIN user_roles ON users.id = user_roles.user_id
        //          INNER JOIN roles on user_roles.user_id = roles.id
        //         "# //?(user_id)
        //         )
        //         .fetch_all(&*pool)
        //         .await;
        //         sqlx::query_as!(
        //             User,
        //             r#"select * from ( SELECT users.id as "id!", created_at as "created_at!", role_name as "role!: Role", username as "username!", updated_at as "updated_at!"
        // FROM users
        //          INNER JOIN user_roles ON users.id = user_roles.user_id
        //          INNER JOIN roles on user_roles.user_id = roles.id
        //         ) as tt"#
        //         )
        // println!("{:?}", filter.unwrap().id.unwrap().to_string());
        // let mut v = vec![];
        // v.push(Some(StuffAndOr {
        //     username: Some(Cond {
        //         eq: "an".to_string(),
        //     }),
        //     and: Option::from(Stuff {
        //         username: Cond {
        //             eq: "ander".to_string(),
        //         },
        //     }),
        //     or: None,
        // }));
        // println!("{:?}", filter.unwrap());

        // let _filter_string = filter
        //     .unwrap_or_else(|| vec![])
        //     .iter()
        //     .map(|x| x.to_string())
        //     .collect::<Vec<String>>()
        //     .join("");

        // let xxxx = v
        //     .iter()
        //     .map(|x| x.as_ref().unwrap().to_string())
        //     .collect::<Vec<String>>()
        //     .join("");
        // assert_eq!(xxxx, "tn");
        // assert_eq!(s, "tn");

        let _filter_string: String = filter.map(|c| c.to_string()).unwrap_or_default();
        let regex = Regex::new(r"(AND|OR)$")?;
        // let filter_string = format!(
        //     "WHERE {}",
        //     regex.replace(&_filter_string.trim(), "").to_string()
        // );
        let filter_string = if _filter_string.len() > 0 {
            format!(
                "WHERE {}",
                regex.replace(&_filter_string.trim(), "").to_string()
            )
        } else {
            "".to_string()
        };

        // let filter_string = _filter_string.replacen(Regex::new(r"(AND|OR)"), "WHERE ", 1);
        // println!("{:?}", x.trim());
        // let filter_string = "WHERE ".to_string() + &filter.unwrap().to_string();
        // filter_string.push_str("WHERE");
        query_with!(
            User,
            pool,
            r#"
        SELECT users.id as id , created_at ,role_name as role, username , updated_at
                "#,
            "users",
            "users.id",
            after,
            before,
            first,
            last,
            r#" INNER JOIN user_roles ON users.id = user_roles.user_id INNER JOIN roles on user_roles.role_id = roles.id "#,
            &order_by_column.unwrap_or(UserColumns::Id).to_string(),
            &order_by.unwrap_or(OrderBy::ASC).to_string(),
            filter_string
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
        println!("upload: filename={}", file.value(ctx).unwrap().filename);
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
