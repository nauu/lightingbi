pub mod mutation_root;
pub mod query_dataset;
pub mod query_root;
pub mod query_user;

pub use self::query_root::QueryRoot;
use crate::query_root::MutationRoot;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use sqlx::MySqlPool;

pub type RootSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema(pool: &MySqlPool) -> RootSchema {
    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(pool.clone())
    .finish()
}
