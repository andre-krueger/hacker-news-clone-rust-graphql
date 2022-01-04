extern crate strum;
#[macro_use]
use crate::error::{Forbidden, UserNotFound};
// use async_graphql::guard::Guard;
use crate::graphql::resolvers::ConnectionFields;
use async_graphql::connection::{Connection, EmptyFields};
use async_graphql::{scalar, Context, Enum, FieldResult, Guard, SimpleObject};
use chrono::{DateTime, Utc};
use serde;
use serde::{Deserialize, Serialize};
use sqlx::database::HasValueRef;
use sqlx::error::BoxDynError;
use sqlx::{Decode, Error, FromRow, PgPool, Postgres};
use std::string::ToString;
use std::sync::Arc;
use strum::Display;
use tokio::sync::{RwLock, RwLockReadGuard};
use warp_sessions::Session;

#[derive(sqlx::Type, Enum, Copy, Clone, Eq, PartialEq, Debug)]
#[sqlx(rename_all = "lowercase")]
#[graphql(rename_items = "PascalCase")]
pub enum Role {
    Admin,
    User,
    Guest,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Display)]
#[graphql(rename_items = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum UserColumns {
    Id,
    Username,
    Role,
    CreatedAt,
    UpdatedAt,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Display)]
pub enum OrderBy {
    ASC,
    DESC,
}

pub struct RoleGuard {
    pub role: Role,
}

impl RoleGuard {
    pub fn new(role: Role) -> Self {
        Self { role }
    }
}

#[async_trait::async_trait]
impl Guard for RoleGuard {
    async fn check(&self, ctx: &Context<'_>) -> FieldResult<()> {
        if ctx.data_opt::<Role>() == Some(&self.role) {
            Ok(())
        } else {
            Err(Forbidden.into())
        }
    }
}

#[derive(SimpleObject, sqlx::FromRow, Clone)]
#[graphql(complex)]
pub struct User {
    #[graphql(skip)]
    pub id: i32,
    pub username: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub role: Role,
}

#[derive(SimpleObject)]
pub struct PageCursor {
    pub cursor: String,
    pub page_number: i32,
    pub is_current: bool,
}

#[derive(SimpleObject)]
pub struct PageCursors {
    pub first: PageCursor,
    pub around: Vec<PageCursor>,
    pub last: PageCursor,
    pub previous: PageCursor,
}

// macro_rules! choose_fields {
//     (
//         $parent:ident,
//         $StructName:ident { $($manual_fields:tt)* },
//         $($field:ident),+ $(,)?
//     ) => {
//         $StructName {
//         $(
//             $field: choose(self.$field().clone(), $parent.$field().clone()),
//         )+
//             $($manual_fields)*
//         }
//     }
// }
//
// choose_fields!(User, username);

#[async_graphql::ComplexObject]
impl User {
    pub async fn id(&self) -> async_graphql::ID {
        self.id.into()
    }
}

trait HasId {
    fn id(&self) -> i32;
}

impl HasId for User {
    fn id(&self) -> i32 {
        self.id
    }
}

#[derive(async_graphql::SimpleObject)]
pub struct UserData {
    pub id: i32,
}

#[derive(async_graphql::SimpleObject)]
pub struct PaginationVecString {
    pub val: String,
}

#[derive(async_graphql::SimpleObject)]
pub struct UserNotFound2 {
    pub message: String,
}

#[derive(async_graphql::SimpleObject)]
pub struct PaginationIncorrect {
    pub message: String,
}

#[derive(async_graphql::Union)]
pub enum UserResult {
    UserData(UserData),
    UserNotFound2(UserNotFound2),
    // PaginationIncorrect{"n"},
    PaginationIncorrect(PaginationIncorrect),
    // Connection(Connection<usize, User, ConnectionFields, EmptyFields>),
}

impl Default for PaginationIncorrect {
    fn default() -> Self {
        //     UserResult {
        //     // PaginationIncorrect {
        //     //     message: "".to_string(),
        //     // }
        //
        // }
        PaginationIncorrect {
            message: "Incorrect pagination".to_string(),
        }
    }
}
