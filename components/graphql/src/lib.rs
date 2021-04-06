mod handler;
pub mod mutation_root;
pub mod query_root;

pub use self::query_root::QueryRoot;
use crate::handler::{graphql, graphql_playground};
use actix_web::web;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use sqlx::MySqlPool;

pub type RootSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn create_schema(pool: &MySqlPool) -> RootSchema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(pool.clone())
        .finish()
}

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.route("/graphql", web::post().to(graphql))
        .route("/playground", web::get().to(graphql_playground));
}
