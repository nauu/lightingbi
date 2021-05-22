use dotenv;
use neo4rs::*;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;

// azy_static! {
//     static ref Neo4jSession: Neo4jSession = {
//         get_graph();
//     };
// }

pub struct Neo4jSession {
    graph: Graph,
}
// /// Returns a [`Query`] which provides methods like [`Query::param`] to add parameters to the query
// pub fn query(q: &str) -> Query {
//     Query::new(q.to_owned())
// }

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub enum NodeSourceType {
    Formula,
}

impl NodeSourceType {
    pub fn getType(&self) -> String {
        match self {
            NodeSourceType::Formula => "Formula".to_string(),
        }
    }
}

impl Neo4jSession {
    pub async fn get_graph() -> Result<Arc<Graph>> {
        let config = Neo4jSession::get_config().await;
        let graph = Arc::new(Graph::connect(config).await.unwrap());
        println!("获取连接");
        Ok(graph)
    }

    pub async fn get_config() -> Config {
        dotenv::dotenv().ok();
        let neo4j_url = env::var("NEO4J_URL").expect("NEO4J_URL is not set in .env file");
        let neo4j_db = env::var("NEO4J_DB").expect("NEO4J_DB is not set in .env file");
        let neo4j_user = env::var("NEO4J_USER").expect("NEO4J_USER is not set in .env file");
        let neo4j_password =
            env::var("NEO4J_PASSWORD").expect("NEO4J_PASSWORD is not set in .env file");
        let neo4j_fetch_size =
            env::var("NEO4J_FETCH_SIZE").expect("NEO4J_FETCH_SIZE is not set in .env file");
        let neo4j_connections = env::var("NEO4J_MAX_CONNECTIONS")
            .expect("NEO4J_MAX_CONNECTIONS is not set in .env file");
        let config = config()
            .uri(&neo4j_url)
            .user(&neo4j_user)
            .password(&neo4j_password)
            .db(&neo4j_db)
            .fetch_size(neo4j_fetch_size.parse().unwrap())
            .max_connections(neo4j_connections.parse().unwrap())
            .build()
            .unwrap();
        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_graph() -> Result<()> {
        let graph = Neo4jSession::get_graph().await.unwrap();
        let mut result = graph.execute(query("RETURN 1")).await.unwrap();
        let row = result.next().await.unwrap().unwrap();
        let value: i64 = row.get("1").unwrap();
        assert_eq!(1, value);
        assert!(result.next().await.unwrap().is_none());
        Ok(())
    }
}
