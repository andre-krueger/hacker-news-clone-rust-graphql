use crate::error::{Forbidden, UserNotFound};
// use async_graphql::guard::Guard;
use async_graphql::{scalar, Context, Enum, FieldResult, Guard, SimpleObject};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::database::HasValueRef;
use sqlx::error::BoxDynError;
use sqlx::{Decode, Error, FromRow, PgPool, Postgres};
use std::sync::Arc;
use tokio::sync::{RwLock, RwLockReadGuard};
use warp_sessions::Session;

#[derive(sqlx::Type, Enum, Copy, Clone, Eq, PartialEq, Debug)]
#[sqlx(rename_all = "lowercase")]
pub enum Role {
    Admin,
    User,
    Guest,
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

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct User {
    #[graphql(skip)]
    pub id: i32,
    pub username: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub role: Role,
}

#[async_graphql::ComplexObject]
impl User {
    pub async fn id(&self) -> async_graphql::ID {
        self.id.into()
    }
}

#[derive(async_graphql::SimpleObject)]
pub struct UserData {
    pub id: i32,
}

#[derive(async_graphql::SimpleObject)]
pub struct UserNotFound2 {
    pub message: String,
}

#[derive(async_graphql::Union)]
pub enum UserResult {
    UserData(UserData),
    UserNotFound2(UserNotFound2),
}
