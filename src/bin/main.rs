#[macro_use]
extern crate log;

use actix_session::CookieSession;
use actix_web::dev::ServiceRequest;
use actix_web::App;
use actix_web::{guard, middleware, web, Error, HttpResponse, HttpServer};
use actix_web_httpauth::extractors::basic::BasicAuth;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenv;
use formula::neo4j_session::Neo4jSession;
use lightingbi::handler::default::p404;
use lightingbi::init_config;
use listenfd::ListenFd;
use sqlx::MySqlPool;
use std::env;

// async fn validator(req: ServiceRequest, _credentials: BearerAuth) -> Result<ServiceRequest, Error> {
//     println!("_credentials:{:?} ", _credentials);
//     // Err(Error::from(HttpResponse::Forbidden()))
//     Ok(req)
// }
async fn validator(req: ServiceRequest, _credentials: BasicAuth) -> Result<ServiceRequest, Error> {
    println!("_credentials:{:?} ", _credentials);
    // Err(Error::from(HttpResponse::Forbidden()))
    Ok(req)
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    // this will enable us to keep application running during recompile: systemfd --no-pid -s http::5000 -- cargo watch -x run
    let mut listenfd = ListenFd::from_env();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = MySqlPool::connect(&database_url).await?;
    let neo4j_graph = Neo4jSession::get_graph().await.unwrap();
    let schema = graphql::create_schema(&db_pool, &neo4j_graph);

    let mut server = HttpServer::new(move || {
        // let auth = HttpAuthentication::bearer(validator);
        let auth = HttpAuthentication::basic(validator);
        App::new()
            .data(schema.clone())
            .data(db_pool.clone()) // pass database pool to application so we can access it inside handlers
            .data(neo4j_graph.clone())
            //cookie session middleware
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // .wrap(auth)
            // default
            .configure(init_config::config_app) // init routes app
            .configure(init_config::config_static) // init routes static
            .configure(graphql::route) // init routes static
            .default_service(
                // 404 for GET request
                web::resource("")
                    .route(web::get().to(p404))
                    // all requests that are not `GET`
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(HttpResponse::MethodNotAllowed),
                    ),
            )
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("HOST is not set in .env file");
            let port = env::var("PORT").expect("PORT is not set in .env file");

            if env::var("WORKERS_NUM").is_ok() {
                println!("set works");
                server
                    .workers(env::var("WORKERS_NUM").unwrap().parse().unwrap())
                    .bind(format!("{}:{}", host, port))?
            } else {
                server.bind(format!("{}:{}", host, port))?
            }
        }
    };

    info!("Lightingbi Starting Server");
    server.run().await?;

    Ok(())
}
