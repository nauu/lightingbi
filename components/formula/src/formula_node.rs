use crate::neo4j_session::Node_Source_Type;
use async_graphql::{Object, OutputJson};
use serde::{Deserialize, Serialize};
use std::error::Error;

///计算节点
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FormulaNode {
    ///节点名称
    key: String,
    ///公式
    formula: String,
    ///所属的数据源
    node_type: Node_Source_Type,
    ///所以完整公式的ID
    formula_id: String,
}

impl FormulaNode {
    pub fn new(key: String, formula: String, formula_id: String) -> Self {
        Self {
            key,
            formula,
            node_type: Node_Source_Type::Formula,
            formula_id,
        }
    }
}

///节点关系
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FormulaNodeRelation {
    ///源ID
    source_index: String,
    ///目标ID
    target_index: String,
    ///计算公式ID
    formula_id: String,
}

impl FormulaNodeRelation {
    pub fn new(source_index: String, target_index: String, formula_id: String) -> Self {
        Self {
            source_index,
            target_index,
            formula_id,
        }
    }
}

///表达式查询结果树
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FormulaTree {
    relations: Vec<FormulaNodeRelation>,
    nodes: Vec<FormulaNode>,
}

impl FormulaTree {
    pub fn new(relations: Vec<FormulaNodeRelation>, nodes: Vec<FormulaNode>) -> Self {
        Self { relations, nodes }
    }
}
