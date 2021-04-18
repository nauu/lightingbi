use std::env;
use std::error::Error;
use std::net::ToSocketAddrs;
use sqlx::{Pool, Database, FromRow, Row, MySqlPool, MySql};
use std::fs::read;
use sqlx::mysql::{MySqlRow, MySqlArguments};
use anyhow::{Result,anyhow};
use async_trait::async_trait;
use std::future::Future;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use tokio::stream::StreamExt;
use serde_json::Value;
use sqlx::Done;
use serde_json::map::Values;
use std::iter::Map;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::cell::RefCell;
use sqlx::query::QueryAs;


///page request
pub struct PageRequest {
    pub size:u32,
    pub num:u32,
    pub sort:String,
}

impl Default for PageRequest{
    fn default() -> Self {
        Self{
            size: 10,
            num: 0,
            sort: "".to_string()
        }
    }
}

///page response
pub struct Page<T> {
    // size of each page
    pub size:u32,
    // number of page
    pub num:u32,
    // total elements
    pub count:u32,
    // total page
    pub total_page:u32,
    // data
    pub context:Vec<T>

}

impl <T> Default for Page<T>{
    fn default() -> Self {
        Page{
            size: 0,
            num: 0,
            count: 0,
            total_page: 0,
            context: vec![]
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
pub trait Entity : Send + Sync + Serialize + DeserializeOwned + Debug   {


    async fn table_name() -> Result<String>{
        let type_name = std::any::type_name::<Self>();
        let mut name = type_name.to_string();
        let names: Vec<&str> = name.split("::").collect();
        name = names.get(names.len() - 1).unwrap_or(&"").to_string();
        let mut pre = env::var("TABLE_NAMESPACE").unwrap_or("t_lighting".to_string());
        name = to_snake_name(&name).await?;
        Ok(pre + "_" + name.as_str())
    }

    async fn id_name() -> Result<String>{
        Ok("id".to_string())
    }

    async fn columns() -> Result<Vec<String>>{
        let json = serde_json::json!("{}");
        let object  = json.as_object().unwrap();
        let mut columns = vec![];
        for(k , v) in object{
            columns.push(k.clone())
        };
        Ok(columns)
    }


}


pub struct MySqlRepository{

}


impl MySqlRepository{

    async fn find_by_id<'q,T> (id : &String , pool : &MySqlPool) -> Result<Option<T>>
        where
            T: for<'r> FromRow<'r, MySqlRow> + Send + Unpin + Debug + Entity,
    {
        if id.is_empty(){
            return Ok(None);
        }
        let table_name = T::table_name().await?;
        let id_name = T::id_name().await?;
        // let columns : Vec<String> = T::columns().await?;
        let sql = format!("select * from {} where {} = ? " , table_name , id_name);

        let entity : Option<T> = sqlx::query_as::<_ , T>(sql.as_str()).bind(&id).fetch_optional(pool).await?;

        Ok(entity)
    }


    async fn query<'q,T>(params:&BTreeMap<String , String> , pool : &MySqlPool) -> Result<Vec<T>>
        where
            T: for<'r> FromRow<'r, MySqlRow> + Send + Unpin + Debug + Entity,
    {
        let table_name = T::table_name().await?;
        let mut sql = format!("select * from {} " , table_name);
        if !params.is_empty(){
            sql.push_str(" where ");
            params.keys()
                .for_each(|param| {
                    sql.push_str(param.as_str());
                    sql.push_str(" = ? and ");
                });
            sql.pop();
            sql.pop();
            sql.pop();
            sql.pop();
        }
        println!(" sql is : {}" , sql);


        let mut query = sqlx::query_as::<_ , T>(sql.as_str());

        if !params.is_empty(){
            // params.values().for_each(|v| {query.bind(v);()});

            // for(_ , v) in params{
            //     query.bind(v);
            // }
        }
        let result = query.fetch_all(pool).await?;


        Ok(result)
    }




    async fn add<T>(entity : &T , pool : &MySqlPool) -> Result<bool>
        where
            T: for<'r> FromRow<'r, MySqlRow> + Send + Unpin + Debug + Entity,
    {
        let table_name = T::table_name().await?;
        let id_name = T::id_name().await?;
        let json = serde_json::json!(entity);
        if !json.is_object(){
            return Err(anyhow!("save error , not a entity object"));
        }

        let object = json.as_object().unwrap();
        let mut columns = String::new();
        let mut place_holder = String::new();
        let mut values:Vec<&Value> = vec![];
        for(column , value) in object{
            columns.push_str(column);
            columns.push(',');


            place_holder.push('?');
            place_holder.push(',');

            values.push(value);

        }
        columns.pop();
        place_holder.pop();
        let insert_sql = format!("insert into {} ({}) values ({})" , table_name , columns , place_holder) ;
        println!("insert sql is : {}" , insert_sql);

        // let mut tx = pool.begin().await?;
        // let mut query = sqlx::query(insert_sql.as_str());
        // for value in values {
        //     match value {
        //         Value::Bool(_) => {query.bind(value.as_bool().unwrap_or(false));()}
        //         Value::Number(_) => {query.bind(value.as_f64().unwrap_or(0.0));()}
        //         Value::String(_) => {query.bind(value.as_str().unwrap_or(""));()}
        //         _ => ()
        //     } ;
        // }

        // values.iter_mut().for_each(move  |value| {
        //    match value {
        //        Value::Bool(_) => {query.bind(value.as_bool().unwrap_or(false))}
        //        Value::Number(_) => {query.bind(value.as_f64().unwrap_or(0.0))}
        //        Value::String(_) => {query.bind(value.as_str().unwrap_or(""))}
        //        _ => query
        //    } ;
        // });
        //
        // let affected = query
        //     .execute(&mut tx)
        //     .await?
        //     .rows_affected();
        //
        // tx.commit().await?;
        //
        // Ok(affected > 0)
        Ok(true)

    }




}



#[cfg(test)]
mod tests {
    use crate::entity::{Entity,MySqlRepository};
    use serde::{Deserialize, Serialize};
    use std::env;
    use sqlx::{MySqlPool, MySql, Database, Encode, Type};
    use anyhow::Result;
    use async_trait::async_trait;
    use sqlx::mysql::MySqlDatabaseError;
    use std::collections::BTreeMap;

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

    #[derive(sqlx::FromRow , Debug, Deserialize, Serialize, Clone)]
    struct Good {
        id:String,
        name : String,
        size : f32,
        count : i32,
    }

    impl Entity for Good{}

    #[tokio::test]
    async fn test_find_by_id() -> Result<()>{
        dotenv::dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let db_pool = MySqlPool::connect(&database_url).await?;

        let good1 = MySqlRepository::find_by_id::<Good>(&String::from("good1") , &db_pool).await?.unwrap();
        println!("{:?}" , good1);
        Ok(())
    }

    #[tokio::test]
    async fn test_query() -> Result<()>{
        let sql = "select * from t_lighting_good where id = ?";
        dotenv::dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let db_pool = MySqlPool::connect(&database_url).await?;
        let id = String::from("good1");
        let mut cursor : Vec<Good> = sqlx::query_as::<_, Good>(sql).bind(&id).fetch_all(&db_pool).await?;
        println!("{:?}" , cursor);
        Ok(())
    }

    #[tokio::test]
    async fn test_add() -> Result<()>{
        dotenv::dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let db_pool = MySqlPool::connect(&database_url).await?;

        let good = Good{
            id: "goo1002".to_string(),
            name: "洗衣机".to_string(),
            size: 89.98,
            count: 120
        };

        MySqlRepository::add(&good , &db_pool);

        Ok(())

    }

    // fn addParam<'q, T: 'q + Send + Encode<'q, Database> + Type<Database>>( params: &mut Vec<T> , param:&T){
    //     params.push(param);
    // }

    #[tokio::test]
    async fn test_query_string() -> Result<()>{
        let sql = "select * from t_lighting_good where id = ? and count = ?";
        dotenv::dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let db_pool = MySqlPool::connect(&database_url).await?;
        let id = String::from("good1");
        let mut cursor : Vec<Good> = sqlx::query_as::<_, Good>(sql).bind(&id).bind(&"2001".to_string()).fetch_all(&db_pool).await?;
        println!("{:?}" , cursor);
        Ok(())
    }

    #[tokio::test]
    async fn test_query_params()-> Result<()>{
        dotenv::dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let db_pool = MySqlPool::connect(&database_url).await?;

        let mut params = BTreeMap :: new();
        params.insert(String::from("id") , String::from("good1"));
        params.insert(String::from("count") , String::from("2000"));

        // let goods = MySqlRepository::query(&params , &db_pool);
        // println!("{:?}" , goods);
        Ok(())
    }
}