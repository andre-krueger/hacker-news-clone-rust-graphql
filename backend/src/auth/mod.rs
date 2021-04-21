use crate::graphql::schema::Role;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::{RwLock, RwLockReadGuard};
use warp_sessions::Session;

pub async fn get_role(pool: &PgPool, session: &Arc<RwLock<Session>>) -> Option<Role> {
    let session: RwLockReadGuard<Session> = session.read().await;
    if let Some(id) = session.get::<i32>("user") {
        match sqlx::query_file!("src/database/queries/user_by_id_with_role.sql", id)
            .fetch_one(&*pool)
            .await
        {
            Ok(user) => Some(user.role),
            _ => Some(Role::Guest),
        }
    } else {
        Some(Role::Guest)
    }
}
