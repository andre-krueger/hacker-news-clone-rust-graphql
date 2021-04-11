use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_warp::Response;
use backend::database::pool::init_pool;
use backend::graphql::resolvers::{MutationRoot, QueryRoot};
use std::convert::Infallible;
use warp::{http::Response as HttpResponse, Filter};

#[tokio::main]
async fn main() {
    let pool = init_pool().await;
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish();

    let graphql_post = warp::post()
        .and(warp::path("graphql"))
        .map(move || pool.clone())
        .and(async_graphql_warp::graphql(schema.clone()))
        .and_then(
            |pool,
             (schema, mut request): (
                Schema<QueryRoot, MutationRoot, EmptySubscription>,
                async_graphql::Request,
            )| async move {
                request = request.data(pool);
                let resp = schema.execute(request).await;
                Ok::<_, Infallible>(Response::from(resp))
            },
        );

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
