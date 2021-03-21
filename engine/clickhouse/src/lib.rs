use async_trait::async_trait;
use clickhouse_rs::types::Complex;
use clickhouse_rs::{Block, Pool};
use engine_crait::Engine;
use query::QueryBuilder;
use std::error::Error;

pub struct ClickHouseEngine {
    pool: Pool,
}

#[async_trait]
impl Engine<QueryBuilder> for ClickHouseEngine {
    type block = Block<Complex>;

    async fn ddl_str(&self, ddl: &str) -> Result<(), Box<dyn Error>> {
        let mut client = self.pool.get_handle().await?;
        client.execute(ddl).await?;
        Ok(())
    }

    async fn query_str(&self, sql: &str) -> Result<(Block<Complex>), Box<dyn Error>> {
        let mut client = self.pool.get_handle().await?;
        let block = client.query(sql).fetch_all().await?;
        Ok((block))
    }

    fn query_qb(&self, query_builder: QueryBuilder) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

impl ClickHouseEngine {
    fn new(database_url: &str) -> Self {
        let pool = Pool::new(database_url);
        ClickHouseEngine { pool }
    }

    async fn insert_block(&self, table_name: &str, block: Block) -> Result<(), Box<dyn Error>> {
        let mut client = self.pool.get_handle().await?;
        client.insert(table_name, block).await?;
        Ok(())
    }
}

fn transfer(query_builder: QueryBuilder) {
    let select_tmpl = " select {} ";
    let from_tmpl = " from {} ";
    let group_tmpl = " group by {} ";
    let order_tmpl = " order by {} ";
    let where_tmpl = " where {} ";

    for d in query_builder.get_rows().iter() {
        println!("{}", d.field.field_name);
    }

    // let select_tmpl = format!(select_tmpl,query_builder.row())
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
    println!("count:{} ", block.rows().count());
    for row in block.rows() {
        let id: u32 = row.get("customer_id")?;
        let amount: u32 = row.get("amount")?;
        let name: Option<&str> = row.get("account_name")?;
        println!("Found payment1 {}: {} {:?}", id, amount, name);
    }
    Ok(())
}
