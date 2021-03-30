#[macro_use]
extern crate log;

use actix_session::CookieSession;
use actix_web::App;
use actix_web::{guard, middleware, web, HttpResponse, HttpServer};
use dotenv;
use lightingbi::handler::default::p404;
use lightingbi::init_config;
use listenfd::ListenFd;
use sqlx::MySqlPool;
use std::{env, io};

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    // this will enable us to keep application running during recompile: systemfd --no-pid -s http::5000 -- cargo watch -x run
    let mut listenfd = ListenFd::from_env();

    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    //let db_pool = MySqlPool::new(&database_url).await?;

    let mut server = HttpServer::new(move || {
        App::new()
            //  .data(db_pool.clone()) // pass database pool to application so we can access it inside handlers
            //cookie session middleware
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // default
            .configure(init_config::config_app) // init routes app
            .configure(init_config::config_static) // init routes static
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
