pub mod mutation_root;
pub mod query_dataset;
pub mod query_formula;
pub mod query_root;
pub mod query_user;

pub use self::query_root::QueryRoot;
use crate::query_root::MutationRoot;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use neo4rs::Graph;
use sqlx::MySqlPool;
use std::sync::Arc;

pub type RootSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema(pool: &MySqlPool, neo4j_pool: &Arc<Graph>) -> RootSchema {
    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(pool.clone())
    .data(neo4j_pool.clone())
    .finish()
}
