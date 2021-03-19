use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait Engine<T> {
    async fn ddl_str(&self, ddl: &str) -> Result<(), Box<dyn Error>>;

    async fn query_str(&self, sql: &str) -> Result<(T), Box<dyn Error>>;
}
