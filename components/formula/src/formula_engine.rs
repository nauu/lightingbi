use crate::formula_function_default::*;
use crate::formula_node::*;
use crate::neo4j_session::NodeSourceType;
use evalexpr::*;
use neo4rs::{query, Graph, Node, Result, Row, RowStream};
use regex::Regex;
use std::collections::HashMap;
use util_crait::uuid_util;

pub struct FormulaEngine {
    /// id
    pub id: String,
    /// 表达式
    pub formula_strs: String,
}

impl FormulaEngine {
    ///创建一个随机id的新实例
    fn new() -> Self {
        //random id
        let id = uuid_util::get_uuid();
        Self {
            id,
            formula_strs: "".to_string(),
        }
    }

    ///根据id返回一个已经存在的实例
    pub fn form(id: String) -> Self {
        Self {
            id,
            formula_strs: "".to_string(),
        }
    }

    /// 设置单个节点表达式
    pub async fn vals(&mut self, val: String) -> &mut FormulaEngine {
        if !self.formula_strs.is_empty() {
            self.formula_strs.push_str(";");
        }
        self.formula_strs.push_str(&val);

        self
    }

    ///保存
    pub async fn save(&mut self, graph: &Graph) -> core::result::Result<Self, String> {
        FormulaEngine::formula_format(&self.formula_strs, &self.id, graph).await
    }

    ///保存公式
    pub async fn formula_format(
        formula: &str,
        formula_id: &String,
        graph: &Graph,
    ) -> core::result::Result<Self, String> {
        let mut node_str = String::new();
        let mut relations = HashMap::<String, Vec<String>>::new();

        let mut id = uuid_util::get_uuid();
        let nodes: Vec<&str> = formula.split(";").collect();
        if formula_id.trim().len() > 0 {
            id = formula_id.clone();
        }

        let mut me = Self {
            id: id.clone(),
            formula_strs: formula.to_string(),
        };

        if me.check_by_id(&graph).await {
            me.delete_by_id(&graph).await;
        }

        for node in nodes {
            // let str = String::from(&node);
            let index = match node.find("=") {
                Some(num) => num,
                None => 0,
            };

            if index <= 0 {
                println!("表达式错误，等号不存在：{}", node);
                return Err("表达式错误，等号不存在".to_string());
            };
            // let vs = node.split_once("=").unwrap();
            let mut iter = node.splitn(2, '=');
            let key = iter.next().unwrap();
            let value = iter.next().unwrap();
            node_str = me
                .get_node_sql(node_str, &(key.to_string()), &(value.to_string()), &id)
                .await;
            relations = me.get_relation(&(key.to_string()), value, relations).await;
        }
        println!("node_str：{}", &node_str);
        let node_str = me.get_relations_sql(node_str, &relations).await;
        println!("last：{}", &node_str);
        &graph.run(query(&*node_str)).await;
        // println!("{}", relations);
        println!("node_str：{}", node_str);
        let fs = formula.clone();
        Ok(me)
    }

    ///获取创建节点的sql
    async fn get_node_sql(
        &mut self,
        mut neo4j_sql: String,
        key: &String,
        value: &String,
        id: &String,
    ) -> String {
        let sql = format!(
            "CREATE ({0}:Formula {{name:'{1}',formula:'{2}',node_type:'{3}',formula_id:'{4}'}})\r",
            key,
            key,
            value,
            NodeSourceType::Formula.get_type(),
            id
        );
        neo4j_sql.push_str(&sql);
        neo4j_sql
    }

    ///获取节点关系的sql
    async fn get_relations_sql(
        &mut self,
        mut neo4j_sql: String,
        relations: &HashMap<String, Vec<String>>,
    ) -> String {
        for (key, value) in relations {
            for right in value {
                let create_sql = format!(
                    "create ({})-[:relation {{node_type:'{}'}}]->({})\r",
                    key,
                    NodeSourceType::Formula.get_type(),
                    right
                );
                neo4j_sql.push_str(&create_sql);
            }
        }
        neo4j_sql
    }

    ///执行计算
    pub async fn run(&mut self, params: HashMap<String, String>, graph: &Graph) -> Result<String> {
        if self.check_cycle(graph).await {
            return Ok("".to_string());
        }
        let q = query("MATCH p = ((leftNode)-[rel:relation*]->(rightNode)) where leftNode.formula_id=$formula_id  RETURN leftNode,rel,rightNode ,length(p) as depth order by depth desc").param("formula_id",self.id.clone());
        let mut result = graph.execute(q).await.unwrap();

        let firstRowOption = result.next().await.unwrap();
        if !firstRowOption.is_some() {
            return Ok("".to_string());
        }

        let first_row: Row = firstRowOption.unwrap();
        let mut params = self.node_calculation(params, &first_row).await;

        while let Ok(Some(row)) = result.next().await {
            let node: Node = row.get("leftNode").unwrap();
            let name: String = node.get("name").unwrap();

            params = self.node_calculation(params, &row).await;
        }

        let firstNode: Node = first_row.get("leftNode").unwrap();
        let mut first_formula = firstNode.get("formula").unwrap();

        let result: f64 = self.eval_formula(&params, first_formula).await.unwrap();
        println!("result:{}", result);
        Ok(result.to_string())
    }

    ///节点计算
    async fn node_calculation(
        &mut self,
        mut params: HashMap<String, String>,
        row: &Row,
    ) -> HashMap<String, String> {
        let rightNode: Node = row.get("rightNode").unwrap();
        let rightName: String = rightNode.get("name").unwrap();

        if !params.contains_key(&rightName) {
            let mut formula: String = rightNode.get("formula").unwrap();
            let v: f64 = self.eval_formula(&params, formula).await.unwrap();
            params.insert(rightName.clone(), v.to_string());
        }
        params
    }

    async fn val(list: Vec<String>) {}

    ///运行表达式
    async fn eval_formula(
        &mut self,
        vaules_map: &HashMap<String, String>,
        formula: String,
    ) -> Result<f64> {
        let mut formula = formula;
        for (key, value) in vaules_map {
            let mut key_str = String::new();
            key_str.push_str("[");
            key_str.push_str(key);
            key_str.push_str("]");
            formula = str::replace(&formula, key_str.as_str(), &value.to_string());
        }
        let mut context = FormulaFunctionDefault::get_fn_context_map().await;
        println!("formula:{}", &formula);
        let eval_result = eval_with_context(&formula.clone(), &mut context).unwrap();
        let eval_result_str = eval_result.to_string();
        let result = eval_result_str.parse::<f64>().unwrap();
        Ok(result)
    }

    ///获取节点关系
    async fn get_relation(
        &mut self,
        key: &String,
        formula: &str,
        mut relation: HashMap<String, Vec<String>>,
    ) -> HashMap<String, Vec<String>> {
        let r = Regex::new(r"\[\w+\]").unwrap();
        let mut list = Vec::<String>::new();
        for (i, c) in r.captures_iter(&formula).enumerate() {
            for j in 0..c.len() {
                let node = String::from(&c[j]);
                let node = node.replace("[", "");
                let rn = node.replace("]", "");
                if !list.contains(&rn) {
                    list.push((&rn.as_str()).to_string());
                }
            }
        }
        relation.insert(key.clone(), list);
        relation
    }

    ///检查公式是否存在
    async fn check_by_id(&mut self, graph: &Graph) -> bool {
        let q = query("MATCH (n:Formula) where n.formula_id = $formula_id RETURN n LIMIT 1")
            .param("formula_id", self.id.clone());

        let mut result: RowStream = graph.execute(q).await.unwrap();

        result.next().await.unwrap().is_some()
    }
    ///删除公式
    async fn delete_by_id(&mut self, graph: &Graph) -> Result<()> {
        let q = query("MATCH (n:Formula) where n.formula_id = $formula_id and n.node_type=$node_type  DETACH DELETE n")
            .param("formula_id", self.id.clone())
            .param("node_type", NodeSourceType::Formula.get_type());
        graph.run(q).await;
        Ok(())
    }

    ///返回公式的树形依赖结构
    async fn tree(&mut self, graph: &Graph) -> Result<FormulaTree> {
        FormulaEngine::tree_by_id(&self.id, graph).await
    }

    ///返回公式的树形依赖结构
    pub async fn tree_by_id(formula_id: &String, graph: &Graph) -> Result<FormulaTree> {
        let q = query("MATCH p = ((leftNode)-[rel:relation*]->(rightNode)) where leftNode.formula_id=$formula_id and length(p) =1  RETURN leftNode,rel,rightNode ,length(p) as depth order by depth desc").param("formula_id",formula_id.clone());
        let mut result = graph.execute(q).await.unwrap();
        let mut nodes = Vec::<FormulaNode>::new();
        let mut relations = Vec::<FormulaNodeRelation>::new();
        let mut nodesMap = HashMap::<String, i32>::new();
        let mut relationKeys = Vec::<String>::new();
        while let Ok(Some(row)) = result.next().await {
            let leftNode: Node = row.get("leftNode").unwrap();
            let leftName: String = leftNode.get("name").unwrap();

            let mut source_index = 1;
            if nodesMap.contains_key(&leftName) {
                source_index = nodesMap.get(&leftName).unwrap().clone();
            } else {
                source_index = nodesMap.len() as i32;

                nodesMap.insert(leftName.clone(), source_index + 1);

                nodes.push(FormulaNode::new(
                    leftName,
                    leftNode.get("formula").unwrap(),
                    formula_id.clone(),
                ));
            }

            let rightNode: Node = row.get("rightNode").unwrap();
            let rightName: String = rightNode.get("name").unwrap();

            let mut target_index = 1;
            if nodesMap.contains_key(&rightName) {
                target_index = nodesMap.get(&rightName).unwrap().clone();
            } else {
                target_index = nodesMap.len() as i32;

                nodesMap.insert(rightName.clone(), target_index + 1);

                nodes.push(FormulaNode::new(
                    rightName,
                    leftNode.get("formula").unwrap(),
                    formula_id.clone(),
                ));
            }
            let relationKey = format!("{}->{}", source_index, target_index);
            if !relationKeys.contains(&relationKey) {
                relations.push(FormulaNodeRelation::new(
                    source_index.to_string(),
                    target_index.to_string(),
                    formula_id.clone(),
                ))
            }
        }

        Ok(FormulaTree::new(relations, nodes))
    }

    ///检查公式是否有循环依赖 true 存在依赖
    async fn check_cycle(&mut self, graph: &Graph) -> bool {
        let q = query("MATCH path = ((x:Formula)-[:relation*]->(y:Formula)) where x.formula_id = $formula_id and x.name = y.name and length(path) >1 RETURN length(path)").param("formula_id", self.id.clone());

        let mut result = graph.execute(q).await.unwrap();

        let firstRowOption = result.next().await.unwrap();

        firstRowOption.is_some()
    }

    ///检查公式是否有循环依赖 true 存在依赖
    pub async fn check_cycle_by_id(formula_id: &String, graph: &Graph) -> bool {
        let q = query("MATCH path = ((x:Formula)-[:relation*]->(y:Formula)) where x.formula_id = $formula_id and x.name = y.name and length(path) >1 RETURN length(path)").param("formula_id", formula_id.clone());

        let mut result = graph.execute(q).await.unwrap();

        let firstRowOption = result.next().await.unwrap();

        firstRowOption.is_some()
    }

    ///打印公式信息
    async fn print(&self) {
        println!("{}", self.formula_strs);
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::rc::Rc;
    use std::thread;
    use std::time::Duration;
    use tokio::task;

    use crate::formula_function_default::*;
    use crate::neo4j_session::Neo4jSession;
    use std::time::{SystemTime, UNIX_EPOCH};

    // a=10;b=20;f=getvalue(1,2,3,4,'abc')+1;c=$a+$b;g=$c*$f;;
    #[tokio::test]
    async fn it_works() -> Result<()> {
        let graph = Neo4jSession::get_graph().await?;
        let formula = "a=10;b=20;f=avg([a],[b],[c],4)+1;c=[a]*[b];g=[c]*[f]";
        let mut fe = FormulaEngine::formula_format(
            &*formula.to_string(),
            &"test_formula_id".to_string(),
            &graph,
        )
        .await
        .unwrap();
        let mut params = HashMap::<String, String>::new();
        let v = fe.run(params, &graph).await;
        Ok(())
    }

    #[tokio::test]
    async fn test_vals() -> Result<()> {
        let graph = Neo4jSession::get_graph().await?;
        let formula = "a=10;b=20;f=avg([a],[b],[c],4)+1;c=[a]*[b];g=[c]*[f]";
        let mut fe = FormulaEngine::form((&"test_formula_id_2").to_string());
        fe.vals("a=10".to_string()).await;
        fe.vals("b=20".to_string()).await;
        fe.vals("f=avg([a],[b],[c],4)+1".to_string()).await;
        fe.vals("c=[a]*[b]+[e]".to_string()).await;
        fe.vals("g=[c]*[f]".to_string()).await;
        fe.vals("e=[g]/3".to_string()).await;
        fe.save(&graph).await;
        let mut params = HashMap::<String, String>::new();
        params.insert("a".to_string(), "10".to_string());
        params.insert("b".to_string(), "20".to_string());
        let v = fe.run(params, &graph).await;
        println!("value:{}", v.unwrap());
        Ok(())
    }

    #[tokio::test]
    async fn test_run() -> Result<()> {
        let mut fe = FormulaEngine::form((&"test_formula_id_1").to_string());
        let graph = Neo4jSession::get_graph().await?;
        let mut params = HashMap::<String, String>::new();
        // params.insert("a".to_string(), "10".to_string());
        // params.insert("b".to_string(), "20".to_string());
        let v = fe.run(params, &graph).await;
        println!("value:{}", v.unwrap());
        Ok(())
    }

    #[tokio::test]
    async fn test_check_by_id() -> Result<()> {
        let mut fe = FormulaEngine::form((&"test_formula_id").to_string());
        let graph = Neo4jSession::get_graph().await?;
        let v = fe.check_by_id(&graph).await;
        println!("value:{}", v);

        Ok(())
    }

    #[tokio::test]
    async fn test_thread_run() -> Result<()> {
        let start = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        println!("start:{}", &start);
        // let mut handles = vec![];
        let graph = Neo4jSession::get_graph().await?;
        for j in 0..100 {
            println!("------{}", j);
            let mut fe = FormulaEngine::form((&"test_formula_id").to_string());
            let mut params = HashMap::<String, String>::new();
            params.insert("a".to_string(), "10".to_string());
            params.insert("b".to_string(), "20".to_string());
            let v = fe.run(params, &graph).await;
            println!("value:{}", v.unwrap());
        }

        let end = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        println!("end:{}", &end);
        println!("耗时：{}", (end - start));
        Ok(())
    }

    #[tokio::test]
    async fn test_eval_formula() -> Result<()> {
        let start = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        println!("start:{}", &start);
        for j in 0..1000 {
            println!("------{}", j);
            let handle = tokio::spawn(async {
                let mut fe = FormulaEngine::new();
                let mut params = HashMap::<String, String>::new();
                params.insert("a".to_string(), "10".to_string());
                params.insert("b".to_string(), "30".to_string());
                params.insert("c".to_string(), "40".to_string());
                let formula = "avg([a],[b],[c],4)+1".to_string();
                let v = fe.eval_formula(&params, (&formula).to_string()).await;
                println!("公式：{},的计算结果：{}", formula, v.unwrap());
            })
            .await;
        }
        let end = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        println!("end:{}", &end);
        println!("耗时：{}", (end - start));
        Ok(())
    }

    #[tokio::test]
    async fn test_check_cycle() -> Result<()> {
        let graph = Neo4jSession::get_graph().await?;
        let b = FormulaEngine::check_cycle_by_id(&("test_formula_id_2".to_string()), &graph).await;
        println!("check_cycle_by_id:{}", b);

        let mut fe = FormulaEngine::form((&"test_formula_id").to_string());
        let b = fe.check_cycle(&graph).await;
        println!("check_cycle:{}", b);

        Ok(())
    }

    #[tokio::test]
    async fn test_tree() -> Result<()> {
        let graph = Neo4jSession::get_graph().await?;

        let mut fe = FormulaEngine::form((&"test_formula_id").to_string());

        let tree = fe.tree(&graph).await.unwrap();
        println!("test_tree:{}", serde_json::to_string(&tree).unwrap());

        Ok(())
    }
}
