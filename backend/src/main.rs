use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, Schema};
// use async_graphql_warp::Response;
use async_redis_session::RedisSessionStore;
use backend::auth::get_role;
use backend::database::pool::init_pool;
use backend::graphql::resolvers::{MutationRoot, QueryRoot};
use dotenv::dotenv;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::reply::Response;
use warp::{http::Response as HttpResponse, Filter};
use warp_sessions::{CookieOptions, SameSiteCookieOption, SessionWithStore};

#[tokio::main]
async fn main() {
    dotenv().ok();
    #[cfg(debug_assertions)]
    pretty_env_logger::init();

    let pool = init_pool().await;
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish();
    let session_store = RedisSessionStore::new("redis://127.0.0.1:6379/").unwrap();

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
                domain: Some("localhost".to_string()),
                max_age: Some(600),
                path: Some("/".to_string()),
                same_site: Some(SameSiteCookieOption::Strict),
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
        .and_then(warp_sessions::reply::with_session);

    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
    });

    #[cfg(debug_assertions)]
    let routes = graphql_playground.or(graphql_post);
    #[cfg(not(debug_assertions))]
    let routes = graphql_post;
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
