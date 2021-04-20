use crate::neo4j_session::Node_Source_Type;
use std::error::Error;

///计算节点
struct FormulaNode {
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
    pub fn new() -> Self {
        Self {
            key: "".to_string(),
            formula: "".to_string(),
            node_type: Node_Source_Type::Formula,
            formula_id: "".to_string(),
        }
    }
}
