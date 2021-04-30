use crate::engine::EngineType::ClickHouse;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[async_trait]
pub trait Engine {
    type Block;

    async fn ddl_str(&self, ddl: &str) -> Result<(), Box<dyn Error>>;

    async fn query_str(&self, sql: &str) -> Result<Self::Block, Box<dyn Error>>;
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub enum EngineType {
    ClickHouse,
    ElasticSearch,
}

impl EngineType {
    pub fn get_type(&self) -> String {
        match self {
            EngineType::ClickHouse => "ClickHouse".to_string(),
            EngineType::ElasticSearch => "ElasticSearch".to_string(),
        }
    }
}

// pub fn engine_name(engine_type : Engine_Type) -> String{
//     let s = Engine_Type{
//         ClickHouse:"123",
//     };
//     match engine_type {
//         Engine_Type::ClickHouse => "ClickHouse".to_string(),
//         Engine_Type::ElasticSearch => "ElasticSearch".to_string()
//     }
// }
