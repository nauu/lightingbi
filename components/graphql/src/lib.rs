mod handler;
pub mod mutation_root;
pub mod query_dataset;
mod query_formula;
pub mod query_root;
pub mod query_user;

pub use self::query_root::QueryRoot;
use crate::handler::{graphql, graphql_playground};
use crate::query_root::MutationRoot;
use actix_web::web;
use actix_web::web::ServiceConfig;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use neo4rs::Graph;
use sqlx::MySqlPool;
use std::sync::Arc;

pub type RootSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema(pool: &MySqlPool, neo4j_graph: &Arc<Graph>) -> RootSchema {
    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(pool.clone())
    .data(neo4j_graph.clone())
    .finish()
}

pub fn route(cfg: &mut ServiceConfig) {
    cfg.route("/graphql", web::post().to(graphql))
        .route("/playground", web::get().to(graphql_playground));
}
