#[cfg(test)]
mod tests {
    use eval::{eval, to_value, Value};
    use futures::poll;
    use futures::stream::*;
    use futures::Future;
    use neo4rs::*;
    use std::collections::HashMap;
    use std::error::Error;
    use std::str;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;

    async fn get_graph() -> Graph {
        let config = config()
            .uri("localhost:7687")
            .user("neo4j")
            .password("1234.abcd")
            .db("neo4j")
            .fetch_size(500)
            .max_connections(10)
            .build()
            .unwrap();
        // let graph = Arc::new(Graph::connect(config).await.unwrap());
        let graph = Graph::connect(config).await.unwrap();
        graph
    }

    async fn delete_all() {
        let graph = get_graph();

        let mut result = graph
            .await
            .run(query("MATCH (n {scope:'calculation'} ) DETACH DELETE n"))
            .await
            .unwrap();
    }

    async fn eval_formula(vaules_map: &HashMap<String, i32>, formula: String) -> Result<i32> {
        let mut formula = formula;
        for (key, value) in vaules_map {
            formula = str::replace(&formula, key, &value.to_string());
        }
        println!("formula{}", &formula);
        let eval_result = eval(&formula.clone());
        let eval_result_str = eval_result.unwrap().to_string();
        let result = eval_result_str.parse::<i32>().unwrap();
        Ok(result)
    }

    async fn node_calculation(
        mut vaules_map: HashMap<String, i32>,
        row: &Row,
    ) -> HashMap<String, i32> {
        let node: Node = row.get("a").unwrap();
        let name: String = node.get("name").unwrap();
        let rightNode: Node = row.get("b").unwrap();
        let rightName: String = rightNode.get("name").unwrap();
        println!("{}->{}->{}", name, "依赖", &rightName);
        if !vaules_map.contains_key(&rightName) {
            let mut formula: String = rightNode.get("formula").unwrap();
            let v: i32 = eval_formula(&vaules_map, formula).await.unwrap();
            vaules_map.insert(rightName.clone(), v);
        }
        vaules_map
    }

    #[tokio::test]
    async fn test_get_graph() -> Result<()> {
        let graph = get_graph();
        let mut result = graph.await.execute(query("RETURN 1")).await.unwrap();
        let row = result.next().await.unwrap().unwrap();
        let value: i64 = row.get("1").unwrap();
        assert_eq!(1, value);
        assert!(result.next().await.unwrap().is_none());
        Ok(())
    }

    /// a + b = c
    /// a + d = f
    /// a + c = e
    /// d + e = g
    /// c + g = h
    #[tokio::test]
    async fn test_create_date() -> Result<()> {
        delete_all();
        let graph = get_graph();

        let mut result = graph
            .await
            .run(query(
                "CREATE (f:Node {key:'f',name:'f',formula:'a+d',scope:'calculation'})\
                CREATE (g:Node {key:'g',name:'g',formula:'d+e',scope:'calculation'})\
                CREATE (h:Node {key:'h',name:'h',formula:'c+g',scope:'calculation'})\
                CREATE (a:Node {key:'a',name:'a',formula:'',scope:'calculation'})\
                CREATE (b:Node {key:'b',name:'b',formula:'',scope:'calculation'})\
                CREATE (c:Node {key:'c',name:'c',formula:'a+b',scope:'calculation'})\
                CREATE (d:Node {key:'d',name:'d',formula:'',scope:'calculation'})\
                CREATE (e:Node {key:'e',name:'e',formula:'a+c',scope:'calculation'})\
                create (c)-[:依赖 {scope:'calculation'}]->(a),\
                (c)-[:依赖 {scope:'calculation'}]->(b),\
                (e)-[:依赖 {scope:'calculation'}]->(a),\
                (e)-[:依赖 {scope:'calculation'}]->(c),\
                (f)-[:依赖 {scope:'calculation'}]->(a),\
                (f)-[:依赖 {scope:'calculation'}]->(d),\
                (g)-[:依赖 {scope:'calculation'}]->(d),\
                (g)-[:依赖 {scope:'calculation'}]->(e),\
                (h)-[:依赖 {scope:'calculation'}]->(c),\
                (h)-[:依赖 {scope:'calculation'}]->(g)",
            ))
            .await
            .unwrap();
        Ok(())
    }

    #[tokio::test]
    async fn test_query_date() -> Result<()> {
        let graph = get_graph();
        let mut vaules_map: HashMap<String, i32> = HashMap::<String, i32>::new();
        vaules_map.insert(String::from("a"), 10);
        vaules_map.insert(String::from("b"), 20);
        vaules_map.insert(String::from("d"), 30);
        let mut result = graph.await.execute(query("MATCH p = ((a)-[c:依赖*]->(b)) where a.name='g'  RETURN a,c,b ,length(p) as depth order by depth desc")).await.unwrap();
        let firstRow: Row = result.next().await.unwrap().unwrap();
        vaules_map = node_calculation(vaules_map, &firstRow).await;
        while let Ok(Some(row)) = result.next().await {
            let node: Node = row.get("a").unwrap();
            let name: String = node.get("name").unwrap();
            vaules_map = node_calculation(vaules_map, &row).await;
        }

        let firstNode: Node = firstRow.get("a").unwrap();
        let mut first_formula = firstNode.get("formula").unwrap();
        let v: i32 = eval_formula(&mut vaules_map, first_formula).await.unwrap();
        println!("{}", v);

        Ok(())
    }
}
