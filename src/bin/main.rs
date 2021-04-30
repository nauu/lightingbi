#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use dotenv;
use formula::neo4j_session;
use formula::neo4j_session::Neo4jSession;
use graphql::RootSchema;
use lightingbi::handler::{default, query};
use sqlx::MySqlPool;
use std::convert::Infallible;
use std::env;
use std::net::ToSocketAddrs;
use warp::http::Response as HttpResponse;
use warp::Filter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = MySqlPool::connect(&database_url).await?;
    let neo4j_graph = Neo4jSession::get_graph().await.unwrap();

    let schema = graphql::create_schema(&db_pool);

    let address = env::var("ADDRESS").expect("ADDRESS is not set in .env file");

    let index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("static/welcome.html"));

    let static_assets = warp::get()
        .and(warp::path("static"))
        .and(warp::fs::dir("static"));

    let graphql_post = async_graphql_warp::graphql(schema).and_then(
        |(schema, request): (RootSchema, async_graphql::Request)| async move {
            // Execute query
            let resp = schema.execute(request).await;
            // Return result
            Ok::<_, Infallible>(async_graphql_warp::Response::from(resp))
        },
    );

    let graphql_playground = warp::get()
        .and(warp::path("playground"))
        .and(warp::get())
        .map(|| {
            HttpResponse::builder()
                .header("content-type", "text/html")
                .body(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
        });

    let routes = graphql_playground
        .or(graphql_post)
        .or(query::route())
        // GET /
        .or(index)
        //GET /static/xxx
        .or(static_assets)
        .with(warp::log("lightingbi"))
        .recover(default::handle_rejection);

    info!("Lightingbi Starting Server");
    warp::serve(routes)
        .run(address.to_socket_addrs().unwrap().next().unwrap())
        .await;

    Ok(())
}
