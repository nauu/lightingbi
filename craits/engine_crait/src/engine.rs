use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error;
use crate::engine::Engine_Type::ClickHouse;

#[async_trait]
pub trait Engine {
    type Block;

    async fn ddl_str(&self, ddl: &str) -> Result<(), Box<dyn Error>>;

    async fn query_str(&self, sql: &str) -> Result<Self::Block, Box<dyn Error>>;
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub enum Engine_Type {
    ClickHouse,
    ElasticSearch,
}

impl Engine_Type{
    pub fn getType(&self) -> String{
        match self {
            Engine_Type::ClickHouse => "ClickHouse".to_string(),
            Engine_Type::ElasticSearch => "ElasticSearch".to_string()
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