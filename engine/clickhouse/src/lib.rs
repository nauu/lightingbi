use clickhouse_rs::types::Complex;
use clickhouse_rs::{Block, Pool};
use query::QueryBuilder;
use std::error::Error;

pub struct ClickHouseEngine {
    pool: Pool,
}

impl ClickHouseEngine {
    fn new(database_url: &str) -> Self {
        let pool = Pool::new(database_url);
        ClickHouseEngine { pool }
    }

    pub async fn ddl_str(&self, ddl: &str) -> Result<(), Box<dyn Error>> {
        let mut client = self.pool.get_handle().await?;
        client.execute(ddl).await?;
        Ok(())
    }

    pub async fn insert_block(&self, table_name: &str, block: Block) -> Result<(), Box<dyn Error>> {
        let mut client = self.pool.get_handle().await?;
        client.insert(table_name, block).await?;
        Ok(())
    }

    pub async fn query_str(&self, ddl: &str) -> Result<(Block<Complex>), Box<dyn Error>> {
        let mut client = self.pool.get_handle().await?;
        let block = client.query("SELECT * FROM payment").fetch_all().await?;

        Ok((block))
    }
}

#[tokio::test]
async fn main() -> Result<(), Box<dyn Error>> {
    let ddl = r"
        CREATE TABLE IF NOT EXISTS payment1 (
            customer_id  UInt32,
            amount       UInt32,
            account_name Nullable(FixedString(3))
        ) Engine=Memory";

    let block = Block::new()
        .column("customer_id", vec![1_u32, 3, 5, 7, 9])
        .column("amount", vec![2_u32, 4, 6, 8, 10])
        .column(
            "account_name",
            vec![Some("foo"), None, None, None, Some("bar")],
        );

    let database_url = "tcp://10.37.129.9:9000/default?compression=lz4&ping_timeout=42ms";

    let qe = ClickHouseEngine::new(database_url);
    qe.ddl_str(ddl).await?;
    qe.insert_block("payment1", block).await?;

    let block = qe.query_str("SELECT * FROM payment1").await?;
    for row in block.rows() {
        let id: u32 = row.get("customer_id")?;
        let amount: u32 = row.get("amount")?;
        let name: Option<&str> = row.get("account_name")?;
        println!("Found payment1 {}: {} {:?}", id, amount, name);
    }
    Ok(())
}
