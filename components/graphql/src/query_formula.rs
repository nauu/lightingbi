use async_graphql::{Context, FieldResult, Object, OutputJson};
use formula::formula_engine::FormulaEngine;
use formula::formula_node::*;
use neo4rs::Graph;
use std::collections::HashMap;

#[derive(Default)]
pub struct QueryFormula;

#[Object]
impl QueryFormula {
    async fn formula_tree_by_id(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> FieldResult<OutputJson<FormulaTree>> {
        let graph = ctx.data_unchecked::<std::sync::Arc<Graph>>(); //Arc<neo4rs::graph::Graph>
        let ft = FormulaEngine::tree_by_id(&id, graph).await.unwrap();
        Ok(ft.into())
    }

    async fn formula_calculate(&self, ctx: &Context<'_>, formula: String) -> FieldResult<String> {
        let graph = ctx.data_unchecked::<std::sync::Arc<Graph>>(); //Arc<neo4rs::graph::Graph>

        let mut fe = FormulaEngine::formula_format(&*formula.to_string(), &"".to_string(), &graph)
            .await
            .unwrap();
        let params = HashMap::<String, String>::new();
        let v = fe.run(params, &graph).await.unwrap();

        Ok(v)
    }
}
