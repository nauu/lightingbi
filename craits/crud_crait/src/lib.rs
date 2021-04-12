use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait CRUD {
    type Result;
    type Pool;

    async fn create(model: Self::Result, pool: &Self::Pool) -> Result<u64>;
    async fn update(model: Self::Result, pool: &Self::Pool) -> Result<bool>;
    async fn delete(id: String, pool: &Self::Pool) -> Result<bool>;
    async fn find_all(pool: &Self::Pool) -> Result<Vec<Self::Result>>;
}
