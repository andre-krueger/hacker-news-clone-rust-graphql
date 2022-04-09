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
use std::fmt::Error;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::reply::{html, Response, WithHeader};
use warp::{header, http, http::Response as HttpResponse, reject, reply, Filter, Rejection};
use warp_sessions::{
    CookieOptions, MemoryStore, SameSiteCookieOption, SessionStore, SessionWithStore,
};

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

async fn handlestuff(
    x: async_redis_session::RedisSessionStore,
    t: String,
) -> Result<String, Error> {
    Ok("t".to_string())
}

// async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, Infallible> {
//     let code;
//     let message;
//     if err.is_not_found() {
//         code = StatusCode::NOT_FOUND;
//         message = "NOT_FOUND";
//     } else if let Some(Unauthorized) = err.find() {
//         return Ok(warp::redirect(Uri::from_static("/admin/login/")));
//         // return Ok(warp::reply::with_header("", "Location", "/admin/login/"));
//         // return Ok(warp::http::Response::builder()
//         //     .header("Location", "/login/")
//         //     // .status(final_status)
//         //     .body("".to_string()));
//     } else {
//         code = StatusCode::INTERNAL_SERVER_ERROR;
//         message = "UNHANDLED_REJECTION";
//     }
//     // Ok(warp::reply::with_status(message, code))
//     // return Ok(warp::reply::with_header("", "Location", "/admin/login/"));
//     return Ok(warp::http::Response::builder()
//         // .header("Location", locati)
//         // .status(final_status)
//         .body("".to_string()));
// }

const FILE_DATA: &str = "test";
#[derive(Debug)]
struct Unauthorized;
impl reject::Reject for Unauthorized {}

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
            session_store.clone(),
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

    // let admin_routes = warp::path("admin")
    //     // .and(warp::path::end())
    //     // .recover(|_| async {
    //     //     Ok::<_, Rejection>(warp::redirect(Uri::from_static("/admin/login/")))
    //     // })
    //     // .and(warp::path!("login"))
    //     .and(warp::path("login").map(|| "test:"))
    //     .and(warp::cookie::<String>("sid"))
    //     // .or(warp::path("login").map(|| Ok("login")))
    //     // .unify()
    //     .map(|_, t: String| {
    //         // warp::redirect(Uri::from_static("https://www.google.com"));
    //         // http::Response::builder()
    //         // Ok(warp::http::Response::builder()
    //         //     .status(303)
    //         //     .header("Location", Uri::from_static("/admin/login/").to_string())
    //         //     .body(""))
    //         // // reply(warp::redirect(Uri::from_static("nn")));
    //         let template = AdminTemplate {};
    //         let res = template.render().unwrap();
    //         Ok(html(res))
    //         // // Ok("Admin")
    //     })
    //     // .unify()
    //     .recover(|_| async {
    //         //
    //         // Ok::<String, Rejection>("test".to_string())
    //         Ok::<_, Rejection>(warp::redirect(Uri::from_static("/admin/login/")))
    //     });
    // let login_route = warp::path!("admin" / "login").map(|| "Ok");
    // let admin_routes = login_route
    //     .or(warp::path("admin")
    //         .map(move || session_store.clone())
    //         .and(warp::cookie::optional::<String>("sid"))
    //         // .and(warp_sessions::request::with_session(
    //         //     session_store.clone(),
    //         //     None
    //         //     // Some(CookieOptions {
    //         //     //     http_only: true,
    //         //     //     cookie_name: "sid",
    //         //     //     secure: false,
    //         //     //     cookie_value: None,
    //         //     //     // domain: Some("localhost".to_string()),
    //         //     //     domain: None,
    //         //     //     max_age: Some(600),
    //         //     //     // path: Some("/".to_string()),
    //         //     //     path: None,
    //         //     //     same_site: Some(SameSiteCookieOption::None),
    //         //     // }),
    //         // ))
    //         .map(
    //             |
    //         x: async_redis_session::RedisSessionStore,
    //         t: Option<String>
    //               // ,_
    //           |async move
    //               {
    //             //
    //             // Err(warp::reject())
    //
    //             if let Some(cookie) = t {
    //                 // session_store.clone();
    //                 let bb = x.load_session(cookie).await;
    //                 println!("ozy");
    //                 println!("test{:?}", bb);
    //                 // let shared_session = Arc::new(RwLock::new(session_store));
    //                 // let x = &session_store.clone().load_session(cookie).await;
    //
    //                 Ok("hascoo")
    //             } else {
    //                 // Ok(warp::redirect(Uri::from_static("/admin/login/")))
    //                 Err(warp::reject())
    //             }
    //             // warp::reply(warp::reject())
    //             // warp::reject()
    //             // Err(reject::custom(Error))
    //         },
    //         ))
    //     .recover(|_| async {
    //         Ok::<_, Rejection>(warp::redirect(Uri::from_static("/admin/login/")))
    //     });
    // let login_route = warp::path("login")
    //     // .and(warp_sessions::request::with_session(
    //     //     session_store.clone(),
    //     //     None,
    //     // ))
    //     // .and(warp::cookie::optional::<String>("sid"))
    //     // .and_then(|_, _| async {
    //     .map(|| {
    //         // Ok::<_, Rejection>(warp::redirect(Uri::from_static("/")))
    //         // warp::redirect(Uri::from_static("/admin"))
    //         // Ok::<_, Rejection>("")
    //         let template = AdminTemplate {};
    //         let res = template.render().unwrap();
    //         Ok::<_, Rejection>(html(res))
    //     });
    // .recover(|_| async {
    //     //
    //     // Ok::<_, Rejection>(warp::redirect(Uri::from_static("/admin")))
    //     Ok::<_,Rejc>("test")
    // });
    // let admin_routes =
    //     warp::path("admin")
    //         // .or(login_route)
    //         // .and(warp::path::end())
    //         .and(warp_sessions::request::with_session(
    //             session_store.clone(),
    //             None,
    //         ))
    //         .and(warp::cookie::optional::<String>("sid"))
    //         .and_then(
    //             |
    //              x: SessionWithStore<async_redis_session::RedisSessionStore>,
    //              t: Option<String>| async move {
    //                 if let Some(coo) = t {
    //                     let xxx = x.session_store.load_session(coo).await.unwrap();
    //                     match xxx {
    //                         Some(_) => {
    //                             let template = AdminTemplate {};
    //                             let res = template.render().unwrap();
    //                             Ok(html(res))
    //                         }
    //                         _ => {
    //                             Err(warp::reject::custom(Unauthorized))
    //                             // let template = AdminTemplate {};
    //                             // let res = template.render().unwrap();
    //                             // Ok(html(res))
    //                         } //     Ok::<_,Rejection>(warp::http::Response::builder()
    //                           //         // .header("content-type", "ent")
    //                           //         // .status(100)
    //                           //         .header("Location","/admin/login/").body(html("".to_string())))
    //                           // }
    //                     }
    //                 } else {
    //                     Err(warp::reject::custom(Unauthorized))
    //                     // let template = AdminTemplate {};
    //                     // let res = template.render().unwrap();
    //                     // Ok::<_, Rejection>(html(res))
    //                 }
    //                 //
    //                 // }
    //                 // async {
    //                 // Ok::<_,Rejection>("t")
    //                 // Ok::<_,warp::http::Error>( warp::http::Response::builder().header("Location", "/admin/login/").body("".to_string()))
    //                 // Ok::<_, Rejection>("")
    //
    //                 //     //     println!("ozy");
    //                 //     // Ok::<_,Rejection>("")
    //                 // //     return match xxx {
    //                 // //         Some(_) => {
    //                 // //             let template = AdminTemplate {};
    //                 // //             let res = template.render().unwrap();
    //                 // //             Ok(html(res))
    //                 // //         }
    //                 // //         _ => Err(warp::reject()),
    //                 // //     };
    //                 // //     // println!("{:?}", xxx.unwrap().get::<String>("id"));
    //                 // //     println!("{:?}", xxx.unwrap().id());
    //                 // //     // let n = xxx.unwrap().get::<String>("id").unwrap();
    //                 // } else {
    //                 //     warp::http::Response::builder().header("Location","/admin/login/")
    //                 //         // Ok::<_, Rejection>(warp::redirect(Uri::from_static("/admin/login/")))
    //                 //
    //                 // }
    //                 //     Err(warp::reject())
    //                 // }
    //                 // Ok::<_, Rejection>(xxx)
    //                 // };
    //                 // Ok("test")
    //             },
    //         )
    //         .recover(|_| async {
    //             Ok::<_, Rejection>(warp::redirect(Uri::from_static("/admin/login/")))
    //         });
    // .recover(|ex: | async {
    //     //
    //     if (ex) {
    //         Ok::<_, Rejection>(warp::redirect(Uri::from_static("/admin/login/")))
    //     }
    // });
    // .recover(handle_rejection);
    // .or(login_route);
    // .and_then(|_| async {
    //     //
    //     // Ok::<_, Rejection>(warp::redirect(Uri::from_static("/admin/login/")))
    // }));
    // .recover(|_| async {
    //     //
    //     Ok(warp::redirect(Uri::from_static("/admin/login/")))
    // }));
    // .recover(reply::with_header("L", "", ()));
    // let login_route = warp::path("login").map(|| Ok("test"));

    let admin_routes = warp::path!("admin")
        // .and(warp::path!("login").map(|| async {
        //     //
        //     Ok::<_, Rejection>("test")
        // }))
        .and(warp_sessions::request::with_session(
            session_store.clone(),
            None,
        ))
        .and(warp::cookie::optional::<String>("sid"))
        .and_then(
            |x: SessionWithStore<RedisSessionStore>, t: Option<String>| async move {
                let template = AdminTemplate {};
                let res = template.render().unwrap();
                // let xxxx = html("");
                if let Some(cookie) = t {
                    let xxx = x.session_store.load_session(cookie).await.unwrap();
                    // Ok::<_, Rejection>("ot")
                    // Err::<String, Rejection>(warp::reject())
                    return match xxx {
                        Some(_) => {
                            Ok::<_, Rejection>(
                                warp::http::response::Builder::new()
                                    // .header("Location", "/admin/login/")
                                    .body(res),
                            )
                        }
                        _ => Ok::<_, Rejection>(
                            warp::http::response::Builder::new()
                                .status(301)
                                .header("Location", "/admin/login/")
                                .body(res),
                        ),
                    };

                    // Ok(html(res))
                    // Ok(warp::http::response::Builder::new()
                    //     .status(StatusCode::FOUND)
                    //     // .header("Location", "/")
                    //     // .header(
                    //     //     header::SET_COOKIE,
                    //     //     format!("EXAUTH={}; SameSite=Strict; HttpOpnly", cookie),
                    //     // )
                    //     .body(Ok(html(FILE_DATA))))
                    // return Ok::<_, Rejection>(
                    //     warp::http::response::Builder::new()
                    //         .header("content-type", "text/html; charset=utf-8")
                    //         .body(x),
                    // );
                    // return Ok::<_, WithHeader<&str>>(warp::reply::with_header("", "", ""));
                } else {
                    // Err::<String, Rejection>(warp::reject())
                    // Ok(warp::http::response::Builder::new()
                    //     .status(StatusCode::FOUND)
                    //     .header("Location", "/admin/login/")
                    //     // .header(
                    //     //     header::SET_COOKIE,
                    //     //     format!("EXAUTH={}; SameSite=Strict; HttpOpnly", cookie),
                    //     // )
                    //     .body(Ok(html(FILE_DATA))))
                }
                // Ok::<

                //     Result<warp::http::Response<warp::reply::Html<&str>>, Rejection>,
                //     warp::http::Error,

                // >
                // Ok::<_, >(warp::reply::with_header("test", "", ""))
                return Ok::<_, Rejection>(
                    warp::http::response::Builder::new()
                        .status(301)
                        .header("Location", "/admin/login/")
                        .body(res),
                );
                // Ok::<_, Rejection>(warp::reply::html(FILE_DATA))
                // Ok::<_, Rejection>(
                //     warp::http::response::Builder::new()
                //         .status(StatusCode::FOUND)
                //         .header("Location", "/admin/login/")
                //         // .header(
                //         //     header::SET_COOKIE,
                //         //     format!("EXAUTH={}; SameSite=Strict; HttpOpnly", cookie),
                //         // )
                //         .body("n"),
                // )
            },
        );
    // .and(warp::path::end())
    // .recover(|_| async { Ok::<_, Rejection>("reject") });
    //     .recover(|_| async {
    //     //
    //     Ok::<_, Rejection>("nn")
    // }));
    // let backend_routes = admin_routes.or(warpfilters::api(redis_pool));
    let backend_routes = warpfilters::api(redis_pool)
        // .or(login_route)
        // .or(warp::path("test").map(|| "tent").and(warp::path::end()))
        // .and(warp::path::end())
        .or(warp::path!("admin" / "login")
            .and(warp_sessions::request::with_session(
                session_store.clone(),
                None,
            ))
            .and(warp::cookie::optional::<String>("sid"))
            .and_then(
                |x: SessionWithStore<RedisSessionStore>, t: Option<String>| async move {
                    let template = AdminTemplate {};
                    let res = template.render().unwrap();

                    if let Some(cookie) = t {
                        let xxx = x.session_store.load_session(cookie).await.unwrap();
                        return match xxx {
                            Some(_) => Ok::<_, Rejection>(
                                warp::http::response::Builder::new()
                                    .status(301)
                                    .header("Location", "/admin/")
                                    .body(res),
                            ),
                            None => {
                                Ok::<_, Rejection>(
                                    warp::http::response::Builder::new()
                                        // .header("Location", "/admin/login/")
                                        .body(res),
                                )
                            }
                        };
                    } else {
                        Ok::<_, Rejection>(
                            warp::http::response::Builder::new()
                                // .header("Location", "/admin/login/")
                                .body(res),
                        )
                    }

                    // Ok::<_, Rejection>(
                    //     warp::http::response::Builder::new()
                    //         .status(301)
                    //         .header("Location", "/admin/")
                    //         .body(res),
                    // )
                    // Ok(html(res))
                },
            ))
        .or(admin_routes);
    // .or(admin_routes.or(warp::path!("login").map(|| Ok("rn"))));
    // .recover(|_| async { Ok::<_, Rejection>("rejec.") })));
    // .and(warp::path("login"))
    // .recover(|_| async { Ok::<_, Rejection>("n") }));
    // .untuple_one()
    // .or(login_route);

    #[cfg(debug_assertions)]
    let routes = warp::path("static")
        .and(warp::fs::dir("static"))
        .or(graphql_playground.or(graphql_post).or(backend_routes));
    // .or(warp::path("static").and(warp::fs::dir("static")));
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
        // let template = AdminTemplate {};
        // let res = template.render().unwrap();
        Ok("home")
        // Ok(html(res))
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
                }), // .and(warp::path::end()), // .and(warp::path::end()),
        )
        // .and(warp::path::end())
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

use backend::graphql::resolvers::ResolverError::UserNotFound;
use serde::Deserialize;
use warp::http::request::Builder;
use warp::http::{StatusCode, Uri};

#[derive(Deserialize)]
pub struct GuestBookQuery {
    pub no_js: Option<String>,
}
