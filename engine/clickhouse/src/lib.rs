use async_trait::async_trait;
use clickhouse_rs::types::Complex;
use clickhouse_rs::{Block, Pool};
use engine_crait::Engine;
use query::{MeasureFn, QueryBuilder};
use std::error::Error;

pub struct ClickHouseEngine {
    pool: Pool,
}

#[async_trait]
impl Engine for ClickHouseEngine {
    type Block = Block<Complex>;

    async fn ddl_str(&self, ddl: &str) -> Result<(), Box<dyn Error>> {
        let mut client = self.pool.get_handle().await?;
        client.execute(ddl).await?;
        Ok(())
    }

    async fn query_str(&self, sql: &str) -> Result<Block<Complex>, Box<dyn Error>> {
        let mut client = self.pool.get_handle().await?;
        let block = client.query(sql).fetch_all().await?;
        Ok(block)
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

    fn do_transfer_to_sql<T>(&self, fields: Vec<T>, call: Box<dyn Fn(&T) -> String>) -> String {
        let mut d_fields = String::new();

        fields.iter().for_each(|x| {
            let s = call(x);
            d_fields.push_str(&(s + ","));
        });
        d_fields.pop();
        d_fields
    }

    fn transfer_to_sql(&self, mut qb: QueryBuilder) -> String {
        let rows_and_cols = qb.get_rows_and_cols();

        let select = self.do_transfer_to_sql(
            rows_and_cols.to_vec(),
            Box::new(|d| d.field.field_name.clone()),
        );

        let group = select.clone();

        let select = format!("select {}", select);
        // println!("select: {}", select);

        let meas = self.do_transfer_to_sql(
            qb.get_meas().to_vec(),
            Box::new(|d| {
                let f = d.field.field_name.clone();
                match d.measure_type {
                    MeasureFn::SUM => format!("sum({}) as {}", f, f),
                    MeasureFn::MAX => format!("max({}) as {}", f, f),
                    MeasureFn::MIN => format!("min({}) as {}", f, f),
                    MeasureFn::AVG => format!("avg({}) as {}", f, f),
                    MeasureFn::COUNT => format!("count({}) as {}", f, f),
                }
            }),
        );

        let table = qb.get_table();
        let table = format!(" from {}", table);
        let group = format!(" group by {}", group);
        let sql = select + "," + &meas + table.as_str() + " " + group.as_str();
        println!("sql: {}", sql);
        sql
    }

    async fn query_qb(
        &self,
        query_builder: QueryBuilder,
    ) -> Result<Block<Complex>, Box<dyn Error>> {
        let sql = self.transfer_to_sql(query_builder);
        let block = self.query_str(sql.as_str()).await?;
        Ok(block)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    async fn print_row(block: Block<Complex>) -> Result<(), Box<dyn Error>> {
        println!("count:{} ", block.rows().count());
        for row in block.rows() {
            let id: u32 = row.get("customer_id")?;
            let amount: u64 = row.get("amount")?;
            let name: Option<&str> = row.get("account_name")?;
            println!("Found  {}: {} {:?}", id, amount, name);
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_qb() -> Result<(), Box<dyn Error>> {
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

        let ce = ClickHouseEngine::new(database_url);
        ce.ddl_str(ddl).await?;
        ce.insert_block("payment1", block).await?;

        let block = ce.query_str("SELECT * FROM payment1").await?;
        print_row(block).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_to_sql() -> Result<(), Box<dyn Error>> {
        let f1 = Field::new(String::from("field1"), DataType::Text, String::from("单位"));
        let f2 = Field::new(String::from("field2"), DataType::Text, String::from("员工"));
        let f3 = Field::new(String::from("field3"), DataType::Date, String::from("时间"));
        let f4 = Field::new(
            String::from("field4"),
            DataType::Number,
            String::from("人数"),
        );
        let f5 = Field::new(
            String::from("field5"),
            DataType::Number,
            String::from("价格"),
        );
        let f6 = Field::new(
            String::from("field6"),
            DataType::Number,
            String::from("数量"),
        );

        let qb = QueryBuilder::new()
            .table(String::from("table1"))
            .row(&mut vec![Dimension::new_row(f1), Dimension::new_row(f3)])
            .col(&mut vec![Dimension::new_col(f2), Dimension::new_col(f4)])
            .meas(&mut vec![
                Measure::new(f5, MeasureFn::MAX),
                Measure::new(f6.clone(), MeasureFn::SUM),
            ])
            .order(&mut vec![Order::new(f6)]);

        //  transfer_to_sql1(qb);
        // transfer_to_sql(Box::new(|qb| {}));

        let database_url = "tcp://10.37.129.9:9000/default?compression=lz4&ping_timeout=42ms";

        let ce = ClickHouseEngine::new(database_url);
        let sql = ce.transfer_to_sql(qb);
        // println!("sql: {}", sql);

        Ok(())
    }

    #[tokio::test]
    async fn test_query_qb() -> Result<(), Box<dyn Error>> {
        let f1 = Field::new(
            String::from("customer_id"),
            DataType::Text,
            String::from("单位ID"),
        );
        let f2 = Field::new(
            String::from("account_name"),
            DataType::Text,
            String::from("账号名称"),
        );
        let f3 = Field::new(
            String::from("amount"),
            DataType::Number,
            String::from("价格"),
        );

        let qb = QueryBuilder::new()
            .table(String::from("payment1"))
            .row(&mut vec![Dimension::new_row(f1)])
            .col(&mut vec![Dimension::new_col(f2)])
            .meas(&mut vec![Measure::new(f3, MeasureFn::SUM)]);

        let database_url = "tcp://10.37.129.9:9000/default?compression=lz4&ping_timeout=42ms";
        let ce = ClickHouseEngine::new(database_url);
        let block = ce.query_qb(qb).await?;
        print_row(block).await?;
        Ok(())
    }
}
