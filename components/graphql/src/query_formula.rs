use async_graphql::{Context, FieldResult, Object};
use crud_crait::CRUD;
use formula::formula_engine::FormulaEngine;
use formula::formula_node::*;
use formula::*;
use neo4rs::{query, Graph, Node, Result, Row, RowStream};
use sqlx::MySqlPool;
use std::sync::Arc;
use user::User;

#[derive(Default)]
pub struct QueryFormula;

#[Object]
impl QueryFormula {
    async fn formula_tree_by_id(&self, ctx: &Context<'_>, id: String) -> FieldResult<FormulaTree> {
        let graph = ctx.data_unchecked::<std::sync::Arc<Graph>>(); //Arc<neo4rs::graph::Graph>
        let ft = FormulaEngine::tree_by_id(&id, graph).await.unwrap();
        Ok(ft)
    }
}
