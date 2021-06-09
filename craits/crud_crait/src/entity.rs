use anyhow::{anyhow, Result};
use async_graphql::{InputObject, OutputType, SimpleObject};
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use sqlx::mysql::{MySqlArguments, MySqlRow};
use sqlx::Arguments;
use sqlx::{Database, FromRow, MySql, MySqlPool, Pool, Row};
use std::collections::BTreeMap;
use std::env;
use std::fmt::Debug;
use std::future::Future;
use std::iter::Map;
use std::net::ToSocketAddrs;
use std::rc::Rc;

///page request
#[derive(InputObject)]
pub struct PageRequest {
    pub size: i64,
    pub num: i64,
    pub sort: String,
}

impl Default for PageRequest {
    fn default() -> Self {
        Self {
            size: 10,
            num: 0,
            sort: "".to_string(),
        }
    }
}

///page response
#[derive(Debug, SimpleObject)]
pub struct Page<T: OutputType> {
    // size of each page
    pub size: i64,
    // number of page
    pub num: i64,
    // total elements
    pub count: i64,
    // total page
    pub total_page: i64,
    // data
    pub context: Vec<T>,
}

impl<T: OutputType> Page<T> {
    pub fn new(size: i64, num: i64, count: i64, total_page: i64, context: Vec<T>) -> Self {
        Page {
            size,
            num,
            count,
            total_page,
            context,
        }
    }
}

impl<T: OutputType> Default for Page<T> {
    fn default() -> Self {
        Page {
            size: 0,
            num: 0,
            count: 0,
            total_page: 0,
            context: vec![],
        }
    }
}

pub async fn to_snake_name(name: &str) -> Result<String> {
    let chs = name.chars();
    let mut new_name = String::new();
    let mut index = 0;
    let chs_len = name.len();
    for x in chs {
        if x.is_uppercase() {
            if index != 0 && (index + 1) != chs_len {
                new_name.push_str("_");
            }
            new_name.push_str(x.to_lowercase().to_string().as_str());
        } else {
            new_name.push(x);
        }
        index += 1;
    }
    Ok(new_name)
}

#[async_trait]
pub trait Entity: Send + Sync + Serialize + DeserializeOwned + Debug + OutputType {
    async fn table_name() -> Result<String> {
        let type_name = std::any::type_name::<Self>();
        let mut name = type_name.to_string();
        let names: Vec<&str> = name.split("::").collect();
        name = names.get(names.len() - 1).unwrap_or(&"").to_string();
        let mut pre = env::var("TABLE_NAMESPACE").unwrap_or("t_lighting".to_string());
        name = to_snake_name(&name).await?;
        Ok(pre + "_" + name.as_str())
    }

    async fn id_name() -> Result<String> {
        Ok("id".to_string())
    }

    async fn columns() -> Result<Vec<String>> {
        let json = serde_json::json!("{}");
        let object = json.as_object().unwrap();
        let mut columns = vec![];
        for (k, v) in object {
            columns.push(k.clone())
        }
        Ok(columns)
    }
}

pub struct MySqlRepository {}

impl MySqlRepository {
    // async fn find_by_id_1<'e, 'c: 'e, T, E>(id : &String, executor: &E) -> Result<Option<T>>
    //     where
    //         T: for<'r> FromRow<'r, MySqlRow> + Send + Unpin + Debug + Entity,
    //         E: 'e + Executor<'c, Database = MySql>,
    // {
    //     if id.is_empty(){
    //         return Ok(None);
    //     }
    //     let table_name = T::table_name().await?;
    //     let id_name = T::id_name().await?;
    //     // let columns : Vec<String> = T::columns().await?;
    //     let sql = format!("select * from {} where {} = ? " , table_name , id_name);
    //
    //     let entity : Option<T> = sqlx::query_as::<_ , T>(sql.as_str()).bind(&id).fetch_optional(executor).await?;
    //
    //     Ok(entity)
    // }

    pub async fn find_by_id<T>(id: &String, pool: &MySqlPool) -> Result<Option<T>>
    where
        T: for<'r> FromRow<'r, MySqlRow> + Send + Unpin + Debug + Entity,
    {
        if id.is_empty() {
            return Ok(None);
        }
        let table_name = T::table_name().await?;
        let id_name = T::id_name().await?;
        // let columns : Vec<String> = T::columns().await?;
        let sql = format!("select * from {} where {} = ? ", table_name, id_name);

        let entity: Option<T> = sqlx::query_as::<_, T>(sql.as_str())
            .bind(&id)
            .fetch_optional(pool)
            .await?;

        Ok(entity)
    }

    pub async fn query<T>(params: &BTreeMap<String, String>, pool: &MySqlPool) -> Result<Vec<T>>
    where
        T: for<'r> FromRow<'r, MySqlRow> + Send + Unpin + Debug + Entity,
    {
        let table_name = T::table_name().await?;
        let mut sql = format!("select * from {} ", table_name);
        let (mut sql_with_param, mut param_values) = Self::add_params_to_sql(sql, params).await?;
        let arg = Self::covert_param_values_to_arg(param_values).await?;
        let mut result = sqlx::query_as_with::<_, T, MySqlArguments>(&sql_with_param, arg)
            .fetch_all(pool)
            .await?;
        Ok(result)
    }

    pub async fn query_page<T>(
        params: &BTreeMap<String, String>,
        page_request: &PageRequest,
        pool: &MySqlPool,
    ) -> Result<Page<T>>
    where
        T: for<'r> FromRow<'r, MySqlRow> + Send + Unpin + Debug + Entity,
    {
        let table_name = T::table_name().await?;
        // total count
        let count = Self::query_count(&table_name, params, pool).await?;

        let mut sql = format!("select *  from {} ", table_name);
        let (mut sql_with_param, mut param_values) = Self::add_params_to_sql(sql, params).await?;

        if !page_request.sort.is_empty() {
            sql_with_param.push_str(" order by ");
            sql_with_param.push_str(page_request.sort.as_str());
        }
        sql_with_param.push_str(" limit ?,?");

        let mut arg = Self::covert_param_values_to_arg(param_values).await?;
        let start = page_request.num * page_request.size;
        arg.add(start);
        arg.add(page_request.size);

        println!("page sql is {} ", sql_with_param);

        let context: Vec<T> = sqlx::query_as_with::<_, T, MySqlArguments>(&sql_with_param, arg)
            .fetch_all(pool)
            .await?;
        let total_page = (count + page_request.size - 1) / page_request.size;

        Ok(Page {
            size: page_request.size,
            num: page_request.num,
            count,
            total_page,
            context,
        })
    }

    pub async fn query_count(
        table_name: &String,
        params: &BTreeMap<String, String>,
        pool: &MySqlPool,
    ) -> Result<i64> {
        let mut sql = format!("select count(*) as count from {} ", table_name);
        let (mut sql_with_param, mut param_values) = Self::add_params_to_sql(sql, params).await?;
        let arg = Self::covert_param_values_to_arg(param_values).await?;
        let row: MySqlRow = sqlx::query_with::<_, MySqlArguments>(&sql_with_param, arg)
            .fetch_one(pool)
            .await?;

        Ok(row.get(0))
    }

    async fn covert_param_values_to_arg(params: Vec<&String>) -> Result<MySqlArguments> {
        let mut arg = MySqlArguments::default();
        if !params.is_empty() {
            params.iter().for_each(|v| {
                arg.add(v);
                ()
            });
        }
        Ok(arg)
    }

    async fn add_params_to_sql(
        mut sql: String,
        params: &BTreeMap<String, String>,
    ) -> Result<(String, Vec<&String>)> {
        let mut param_values = vec![];
        if !params.is_empty() {
            sql.push_str(" where ");
            for (k, v) in params {
                sql.push_str(k.as_str());
                sql.push_str(" = ? and ");
                param_values.push(v);
            }
            sql.truncate(sql.len() - 4);
        }
        println!("sql is {} ", sql);
        Ok((sql, param_values))
    }

    async fn covert_object_to_arg(params: Vec<&Value>) -> Result<MySqlArguments> {
        let mut arg = MySqlArguments::default();
        for value in params {
            match value {
                Value::Bool(_) => {
                    arg.add(value.as_bool().unwrap_or(false));
                    ()
                }
                Value::Number(_) => {
                    if value.is_i64() {
                        arg.add(value.as_i64().unwrap_or(0));
                    } else if value.is_u64() {
                        arg.add(value.as_u64().unwrap_or(0));
                    } else if value.is_f64() {
                        arg.add(value.as_f64().unwrap_or(0.0));
                    }
                    ()
                }
                Value::String(_) => {
                    arg.add(value.as_str().unwrap_or(""));
                    ()
                }
                _ => (),
            };
        }
        Ok(arg)
    }

    pub async fn add<T>(entity: &T, pool: &MySqlPool) -> Result<bool>
    where
        T: for<'r> FromRow<'r, MySqlRow> + Send + Unpin + Debug + Entity,
    {
        let table_name = T::table_name().await?;
        let json = serde_json::json!(entity);
        if !json.is_object() {
            return Err(anyhow!("save error , not a entity object"));
        }

        let object = json.as_object().unwrap();
        let mut columns = String::new();
        let mut place_holder = String::new();
        let mut values: Vec<&Value> = vec![];
        for (column, value) in object {
            columns.push_str(column);
            columns.push(',');

            place_holder.push('?');
            place_holder.push(',');

            values.push(value);
        }
        columns.pop();
        place_holder.pop();
        let insert_sql = format!(
            "insert into {} ({}) values ({})",
            table_name, columns, place_holder
        );
        println!("insert sql is : {}", insert_sql);
        let arg = Self::covert_object_to_arg(values).await?;
        let rows_affected = sqlx::query_with(insert_sql.as_str(), arg)
            .execute(pool)
            .await?
            .rows_affected();

        println!("{}", rows_affected);
        Ok(rows_affected > 0)
    }

    pub async fn update<T>(entity: &T, pool: &MySqlPool) -> Result<bool>
    where
        T: for<'r> FromRow<'r, MySqlRow> + Send + Unpin + Debug + Entity,
    {
        let table_name = T::table_name().await?;
        let id_name = T::id_name().await?;
        let json = serde_json::json!(entity);
        if !json.is_object() {
            return Err(anyhow!("save error , not a entity object"));
        }

        let object = json.as_object().unwrap();
        let id_value_op = object.get(&id_name);
        if id_value_op == None {
            return Err(anyhow!("save error , {} is null", id_name));
        }
        let id_value = id_value_op.unwrap();
        let mut columns = String::new();
        let mut values: Vec<&Value> = vec![];
        for (column, value) in object {
            if column.eq(&id_name) {
                continue;
            }
            columns.push_str(column);
            columns.push_str(" = ?,");
            values.push(value);
        }
        columns.pop();
        values.push(id_value);
        let insert_sql = format!(
            "update {} set {} where {} = ? ",
            table_name, columns, id_name
        );
        println!("insert sql is : {}", insert_sql);
        let arg = Self::covert_object_to_arg(values).await?;
        let rows_affected = sqlx::query_with(insert_sql.as_str(), arg)
            .execute(pool)
            .await?
            .rows_affected();
        println!("{}", rows_affected);
        Ok(rows_affected > 0)
    }

    pub async fn delete_by_id<T>(id: &String, pool: &MySqlPool) -> Result<bool>
    where
        T: for<'r> FromRow<'r, MySqlRow> + Send + Unpin + Debug + Entity,
    {
        if id.is_empty() {
            return Ok(true);
        }
        let table_name = T::table_name().await?;
        let id_name = T::id_name().await?;
        // let columns : Vec<String> = T::columns().await?;
        let delete_sql = format!("delete from {} where {} = ? ", table_name, id_name);
        println!("delete sql is : {}", delete_sql);
        let rows_affected = sqlx::query(delete_sql.as_str())
            .bind(&id)
            .execute(pool)
            .await?
            .rows_affected();
        Ok(rows_affected > 0)
    }
}

#[cfg(test)]
mod tests {
    use crate::entity::{Entity, MySqlRepository, PageRequest};
    use anyhow::Result;
    use async_trait::async_trait;
    use serde::{Deserialize, Serialize};
    use sqlx::mysql::{MySqlArguments, MySqlDatabaseError};
    use sqlx::Arguments;
    use sqlx::{Encode, MySql, MySqlPool, Type};
    use std::collections::BTreeMap;
    use std::env;

    ///
    /// CREATE TABLE IF NOT EXISTS t_lighting_good (
    // `id` varchar(128) NOT NULL,
    //     `name` varchar(128) NOT NULL,
    //     `size` double,
    //     `count` int,
    //     PRIMARY KEY (`id`)
    // )ENGINE=InnoDB;
    // insert into t_lighting_good(id , name , size , count) VALUES ('good1' , '洗碗机',100.0 , 2000);
    ///

    #[derive(sqlx::FromRow, Debug, Deserialize, Serialize, Clone)]
    struct Good {
        id: String,
        name: String,
        size: f32,
        count: i32,
    }

    impl Entity for Good {}

    #[tokio::test]
    async fn test_crud() -> Result<()> {
        dotenv::dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let db_pool = MySqlPool::connect(&database_url).await?;

        let good = Good {
            id: "good_test".to_string(),
            name: "洗衣机".to_string(),
            size: 89.98,
            count: 120,
        };
        // add
        MySqlRepository::add(&good, &db_pool).await?;

        // find
        let good1 = MySqlRepository::find_by_id::<Good>(&String::from("good_test"), &db_pool)
            .await?
            .unwrap();
        println!("{:?}", good1);

        // update
        let good_update = Good {
            id: "good_test".to_string(),
            name: "洗衣机123".to_string(),
            size: 189.98,
            count: 130,
        };
        MySqlRepository::update(&good_update, &db_pool).await?;

        // find
        let good2 = MySqlRepository::find_by_id::<Good>(&String::from("good_test"), &db_pool)
            .await?
            .unwrap();
        println!("{:?}", good2);

        // delete
        MySqlRepository::delete_by_id::<Good>(&String::from("good_test"), &db_pool).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_find_by_id() -> Result<()> {
        dotenv::dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let db_pool = MySqlPool::connect(&database_url).await?;

        let good1 = MySqlRepository::find_by_id::<Good>(&String::from("good1"), &db_pool)
            .await?
            .unwrap();
        println!("{:?}", good1);
        Ok(())
    }

    #[tokio::test]
    async fn test_query() -> Result<()> {
        let sql = "select * from t_lighting_good where id = ?";
        dotenv::dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let db_pool = MySqlPool::connect(&database_url).await?;
        let id = String::from("good1");
        let mut cursor: Vec<Good> = sqlx::query_as::<_, Good>(sql)
            .bind(&id)
            .fetch_all(&db_pool)
            .await?;
        println!("{:?}", cursor);
        Ok(())
    }

    #[tokio::test]
    async fn test_add() -> Result<()> {
        dotenv::dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let db_pool = MySqlPool::connect(&database_url).await?;

        let good = Good {
            id: "goo1002".to_string(),
            name: "洗衣机".to_string(),
            size: 89.98,
            count: 120,
        };

        MySqlRepository::add(&good, &db_pool).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_update() -> Result<()> {
        dotenv::dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let db_pool = MySqlPool::connect(&database_url).await?;

        let good = Good {
            id: "goo1002".to_string(),
            name: "洗衣机123".to_string(),
            size: 100.02,
            count: 120,
        };

        MySqlRepository::update(&good, &db_pool).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_delete() -> Result<()> {
        dotenv::dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let db_pool = MySqlPool::connect(&database_url).await?;

        MySqlRepository::delete_by_id::<Good>(&"goo1002".to_string(), &db_pool).await?;
        Ok(())
    }

    // fn addParam<'q, T: 'q + Send + Encode<'q, Database> + Type<Database>>( params: &mut Vec<T> , param:&T){
    //     params.push(param);
    // }

    #[tokio::test]
    async fn test_query_string() -> Result<()> {
        let sql = "select * from t_lighting_good where id = ? and count = ?";
        dotenv::dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let db_pool = MySqlPool::connect(&database_url).await?;
        let id = String::from("good1,2000");
        // let mut cursor : Vec<Good> = sqlx::query_as::<_, Good>(sql).bind(&id).bind(&"2001".to_string()).fetch_all(&db_pool).await?;
        // let mut cursor : Vec<Good> = sqlx::query_as!(Good ,
        //     "select * from t_lighting_good where id = ? and count = ?" , "good1" , 2000).fetch_all(&db_pool).await?;

        let mut a = MySqlArguments::default();
        a.add("good1");
        a.add("2000");
        let mut cursor: Vec<Good> = sqlx::query_as_with::<_, Good, MySqlArguments>(sql, a)
            .fetch_all(&db_pool)
            .await?;

        println!("{:?}", cursor);
        Ok(())
    }

    #[tokio::test]
    async fn test_query_params() -> Result<()> {
        dotenv::dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let db_pool = MySqlPool::connect(&database_url).await?;

        let mut params = BTreeMap::new();
        params.insert(String::from("id"), String::from("good1"));
        params.insert(String::from("count"), String::from("2000"));

        let goods = MySqlRepository::query::<Good>(&params, &db_pool).await?;
        println!("{:?}", goods);
        Ok(())
    }

    #[tokio::test]
    async fn test_query_count() -> Result<()> {
        dotenv::dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let db_pool = MySqlPool::connect(&database_url).await?;

        let mut params = BTreeMap::new();
        params.insert(String::from("id"), String::from("good1"));
        params.insert(String::from("count"), String::from("2000"));

        let goods =
            MySqlRepository::query_count(&"t_lighting_good".to_string(), &params, &db_pool).await?;
        println!("{:?}", goods);
        Ok(())
    }

    #[tokio::test]
    async fn test_query_page() -> Result<()> {
        dotenv::dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let db_pool = MySqlPool::connect(&database_url).await?;

        let mut params = BTreeMap::new();
        params.insert(String::from("id"), String::from("good1"));
        params.insert(String::from("count"), String::from("2000"));

        let request = PageRequest {
            size: 10,
            num: 0,
            sort: "id desc".to_string(),
        };

        let goods = MySqlRepository::query_page::<Good>(&params, &request, &db_pool).await?;
        println!("{:?}", goods);
        Ok(())
    }
}
