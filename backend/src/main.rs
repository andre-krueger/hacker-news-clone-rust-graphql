use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, Schema};
// use async_graphql_warp::Response;
use async_redis_session::RedisSessionStore;
use backend::auth::get_role;
use backend::database::pool::{init_pool, init_redis_pool};
use backend::graphql::resolvers::{MutationRoot, QueryRoot};
use deadpool_redis::{Connection, Pool};
use dotenv::dotenv;
use redis::cmd;
use std::convert::Infallible;
use std::env;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::reply::{html, Response};
use warp::{header, http::Response as HttpResponse, Filter, Rejection};
use warp_sessions::{CookieOptions, SameSiteCookieOption, SessionWithStore};

// pub static MANIFEST: &'static str = include_str!("../../build-admin/asset-manifest.json");

pub mod filters {
    // use crate::MANIFEST;
    use serde_json::{to_value, Value};

    pub fn load_static(s: &str) -> ::askama::Result<String> {
        // let manifest: Value = serde_json::from_str(MANIFEST).unwrap();
        // let n = manifest.get("files").unwrap().get(s).unwrap().to_string();
        // Ok(n)
        Ok("".to_string())
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    #[cfg(debug_assertions)]
    pretty_env_logger::init();

    let pool = init_pool().await;

    sqlx::migrate!().run(&pool).await;
    let mut redis_pool = init_redis_pool().await;
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish();
    let redis_url = env::var("REDIS_URL").unwrap();
    let session_store = RedisSessionStore::new(redis_url).unwrap();

    let graphql_post = warp::post()
        .and(warp::path("graphql"))
        .map(move || pool.clone())
        .and(async_graphql_warp::graphql(schema.clone()))
        .and(warp_sessions::request::with_session(
            session_store,
            Some(CookieOptions {
                http_only: true,
                cookie_name: "sid",
                secure: false,
                cookie_value: None,
                // domain: Some("localhost".to_string()),
                domain: None,
                max_age: Some(600),
                // path: Some("/".to_string()),
                path: None,
                same_site: Some(SameSiteCookieOption::None),
            }),
        ))
        .and_then(
            |pool,
             (schema, mut request): (
                Schema<QueryRoot, MutationRoot, EmptySubscription>,
                async_graphql::Request,
            ),
             mut session_with_store: SessionWithStore<RedisSessionStore>| async move {
                let shared_session = Arc::new(RwLock::new(session_with_store.session));
                let maybe_role = get_role(&pool, &shared_session).await;
                if let Some(role) = maybe_role {
                    request = request.data(role);
                }
                request = request.data(shared_session.clone());
                request = request.data(pool);
                let resp = schema.execute(request).await;
                session_with_store.session = Arc::try_unwrap(shared_session).unwrap().into_inner();
                Ok::<_, Infallible>((
                    async_graphql_warp::GraphQLResponse::from(resp),
                    session_with_store,
                ))
            },
        )
        .untuple_one()
        // .header("access-control-allow-credentials", "true")
        .and_then(warp_sessions::reply::with_session);

    let graphql_playground = warp::path("playground").and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
    });

    let admin_routes = warp::path("admin").map(|| {
        // let template = AdminIndexTemplate {};
        // let res = template.render().unwrap();
        // Ok(html(res))
        Ok("Admin")
    });

    let backend_routes = admin_routes.or(warpfilters::api(redis_pool));

    #[cfg(debug_assertions)]
    let routes = graphql_playground
        .or(graphql_post)
        .or(backend_routes)
        .or(warp::path("static").and(warp::fs::dir("static")));
    // .or(warp::path("static").and(warp::fs::dir("../build-admin/static")))
    // .or(warp::path("js").and(warp::fs::dir("../frontend_parcel/static/js")));
    #[cfg(not(debug_assertions))]
    let routes = graphql_post.or(backend_routes);
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

mod handlers {
    use crate::{AdminTemplate, GuestBookQuery, GuestbookTemplate};
    use askama::Template;
    use deadpool_redis::Pool;
    use redis::{cmd, AsyncCommands, Connection};
    use std::convert::Infallible;
    use warp::reply::html;
    use warp::test::request;
    use warp::{cookie, http, reject, Filter, Rejection};

    pub async fn cards(pool: Pool) -> Result<impl warp::Reply, Infallible> {
        Ok("cards")
    }

    pub async fn galleries(pool: Pool) -> Result<impl warp::Reply, Infallible> {
        Ok("galleries")
    }
    pub async fn home(pool: Pool) -> Result<impl warp::Reply, Infallible> {
        let template = AdminTemplate {};
        let res = template.render().unwrap();
        // Ok("home")
        Ok(html(res))
    }
    pub async fn guestbook(b: GuestBookQuery, pool: Pool) -> Result<impl warp::Reply, Infallible> {
        println!("{:?}", b.no_js);
        let mut template: GuestbookTemplate; // = GuestbookTemplate {};
        match b {
            GuestBookQuery { no_js: Some(no_js) } => {
                template = GuestbookTemplate {
                    entry: Some("test".to_string()),
                }
            }
            _ => template = GuestbookTemplate { entry: None },
        };
        let res = template.render().unwrap();
        Ok(html(res))
        // match b.as_deref() {
        //     Some("application/json") => Ok("some"),
        //     _ => Ok("guestbook"),
        // }
        // if Some(b) == "application/json".to_string() {
        //     Ok("some")
        // } else {
        //     Ok("guestbook")
        // }
    }
}

fn with_db(pool: Pool) -> impl Filter<Extract = (Connection,), Error = Rejection> + Clone {
    warp::any().and_then(move || {
        let pool = pool.clone();
        async move {
            match pool.get().await {
                Ok(db) => Ok(db),
                Err(_) => {
                    println!("rejeentct");
                    Err(warp::reject())
                }
            }
        }
    })
}

use askama::Template;
use serde_json::Value;

#[derive(Template)]
#[template(path = "admin.html")]
struct AdminTemplate;

// #[derive(Template)]
// #[template(path = "admin-index.html")]
// struct AdminIndexTemplate;

#[derive(Template)]
#[template(path = "guestbook.html")]
struct GuestbookTemplate {
    entry: Option<String>,
}

mod warpfilters {
    use crate::{handlers, with_db, GuestBookQuery};
    use deadpool_redis::{Connection, Pool};
    use redis::AsyncCommands;
    use sqlx::PgPool;
    use warp::{cookie, http, reply, Filter, Rejection};

    pub fn api(
        pool: Pool,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        auth_validation(pool.clone()).untuple_one().and(
            cards(pool.clone())
                .or(home(pool.clone()))
                .or(galleries(pool.clone()))
                .or(guestbook(pool.clone()))
                .map(|reply| {
                    warp::reply::with_header(
                        reply,
                        "set-cookie",
                        "visited=true; Secure; HttpOnly; SameSite=Lax",
                    )
                }),
        )
    }

    fn auth_validation(pool: Pool) -> impl Filter<Extract = ((),), Error = Rejection> + Clone {
        warp::cookie::optional("visited")
            .and(with_db(pool))
            .and_then(move |s: Option<String>, mut db: Connection| async move {
                if Some(s) != Some(Some("true".to_string())) {
                    db.incr::<_, _, i32>("hit_count", 1).await.unwrap();
                }
                Ok::<_, Rejection>(())
            })
    }

    pub fn galleries(
        pool: Pool,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("galleries")
            .and(warp::any().map(move || pool.clone()))
            .and_then(handlers::galleries)
    }

    pub fn home(
        pool: Pool,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path::end()
            .and(warp::any().map(move || pool.clone()))
            .and_then(handlers::home)
    }

    pub fn cards(
        pool: Pool,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("cards")
            .and(warp::any().map(move || pool.clone()))
            .and_then(handlers::cards)
    }

    pub fn guestbook(
        pool: Pool,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        // warp::path!("guestbook").and(
        //     warp::header::optional("test")
        //         .and(warp::any().map(move || pool.clone()))
        //         .and_then(handlers::guestbook),
        // )
        // warp::path!("guestbook")
        warp::query::<GuestBookQuery>()
            .and(warp::path!("guestbook"))
            .and(warp::any().map(move || pool.clone()))
            .and_then(handlers::guestbook)
        // warp::get()
        //     .and(warp::header::<String>("test").or(
        //         warp::path!("guestbook").map(|| ""), // .and(warp::header::optional::<String>("test"))
        //     ))
        //     .and(warp::any().map(move || pool.clone()))
        //     .and_then(handlers::guestbook)
    }
}

use serde::Deserialize;

#[derive(Deserialize)]
pub struct GuestBookQuery {
    pub no_js: Option<String>,
}
