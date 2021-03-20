use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait Engine<T> {
    type block;

    async fn ddl_str(&self, ddl: &str) -> Result<(), Box<dyn Error>>;

    async fn query_str(&self, sql: &str) -> Result<(Self::block), Box<dyn Error>>;

    fn query_qb(&self, query_builder: T) -> Result<(), Box<dyn Error>>;
}
