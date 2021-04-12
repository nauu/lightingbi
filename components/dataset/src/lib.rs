use anyhow::Result;
use async_trait::async_trait;
use crud_crait::CRUD;
use engine_craits::Engine_Type;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

///The struct of Dataset
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Dataset {
    ///primary key
    pub id: String,
    ///the physical field name
    pub name: String,
    ///the alias name to display
    pub display_name: String,
    ///the fields
    pub fields: Vec<Field>,
    pub engine_type: Engine_Type,
    pub size: u64,
    pub count: u64,
}

::async_graphql::scalar!(Dataset);

impl Dataset {
    pub fn new() -> Self {
        Self {
            id: "".to_string(),
            name: "".to_string(),
            display_name: "".to_string(),
            fields: vec![],
            engine_type: Engine_Type::ClickHouse,
            size: 0,
            count: 0,
        }
    }
}

#[async_trait]
impl CRUD for Dataset {
    type Result = Dataset;
    type Pool = MySqlPool;

    async fn create(dataset: Dataset, pool: &Self::Pool) -> Result<u64> {
        Ok(1)
    }

    async fn update(dataset: Dataset, pool: &Self::Pool) -> Result<bool> {
        todo!()
    }

    async fn delete(id: String, pool: &Self::Pool) -> Result<bool> {
        todo!()
    }

    async fn find_all(pool: &Self::Pool) -> Result<Vec<Self::Result>> {
        todo!()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Field {
    pub id: String,
    pub name: String,
    pub data_type: DataType,
    pub field_type: String,
    pub display_name: String,
    pub formula: String,
}

impl Field {}

#[async_trait]
impl CRUD for Field {
    type Result = Field;
    type Pool = MySqlPool;

    async fn create(model: Self::Result, pool: &Self::Pool) -> Result<u64> {
        todo!()
    }

    async fn update(model: Self::Result, pool: &Self::Pool) -> Result<bool> {
        todo!()
    }

    async fn delete(id: String, pool: &Self::Pool) -> Result<bool> {
        todo!()
    }

    async fn find_all(pool: &Self::Pool) -> Result<Vec<Self::Result>> {
        todo!()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum DataType {
    Text,
    Number,
    Date,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_crud() {
        dotenv::dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let db_pool = MySqlPool::connect(&database_url).await?;

        let ds = Dataset {
            id: "".to_string(),
            name: "".to_string(),
            display_name: "".to_string(),
            fields: vec![],
            engine_type: Engine_Type::ClickHouse,
            size: 0,
            count: 0,
        };
        let ds1 = ds.clone();

        let id = Dataset::create(ds, db_pool).await?;

        let is_ok = Dataset::update(ds1, db_pool).await?;

        let result = Dataset::find_all(db_pool).await?;

        let is_ok = Dataset::delete(db_pool).await?;

        assert_eq!(2 + 2, 4);
    }
}
