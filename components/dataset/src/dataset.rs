use anyhow::Result;
use async_trait::async_trait;
use crud_crait::CRUD;
use engine_craits::Engine_Type;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use sqlx::Done;
use util_crait::uuid_util;
use std::rc::Rc;
use std::cell::RefCell;


///The struct of Dataset
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Dataset {
    ///primary key
    pub id: String,
    ///the physical table name
    pub name: String,
    ///the alias name to display
    pub display_name: String,
    ///the fields
    pub fields: Vec<Field>,
    pub engine_type: Engine_Type,
    pub size: f64,
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
            size: 0.0,
            count: 0,
        }
    }
}

#[async_trait]
impl CRUD for Dataset {
    type Result = Dataset;
    type Pool = MySqlPool;

    async fn create(dataset: Dataset, pool: &Self::Pool) -> Result<Dataset> {
        let mut new_dataset = dataset.clone();
        // let id = Rc::new(uuid_util::get_uuid());
        // let ref_ = RefCell::new(id);
        let id = uuid_util::get_uuid();
        new_dataset.id = id.clone();
        let table_name = "t_".to_string() + &uuid_util::get_short_uuid();
        new_dataset.name = table_name;

        let mut tx = pool.begin().await?;
        sqlx::query(" INSERT INTO t_lighting_dataset (id , name, display_name , engine_type , size , count) VALUES (?, ? , ? ,? ,? ,?)")
            .bind(&new_dataset.id)
            .bind(&new_dataset.name)
            .bind(&new_dataset.display_name)
            .bind(&new_dataset.engine_type.getType())
            .bind(&new_dataset.size)
            .bind(&new_dataset.count)
            .execute(&mut tx)
            .await?;

        if !new_dataset.fields.is_empty() {
            for mut field in new_dataset.fields.iter_mut(){
                if field.id.is_empty(){
                    field.id = uuid_util::get_uuid();
                }
                if field.name.is_empty(){
                    field.name = "f_".to_string() + &uuid_util::get_short_uuid();
                }
                // field.dataset_id = ref_.borrow().clone();
                field.dataset_id = id.clone();
                println!("{:?}" , field);

                sqlx::query(" INSERT INTO t_lighting_field (id , name , dataset_id , data_type , field_type , display_name , formula) VALUES (?, ? , ? ,? ,? ,? , ?)")
                    .bind(&field.id)
                    .bind(&field.name)
                    .bind(&field.dataset_id)
                    .bind(&data_type_name(field.data_type))
                    .bind(&field.field_type)
                    .bind(&field.display_name)
                    .bind(&field.formula)
                    .execute(&mut tx)
                    .await?;
            }
        }
        println!("{:?}" , new_dataset);
        tx.commit().await?;
        Ok(new_dataset)
    }




    async fn update(dataset: Dataset, pool: &Self::Pool) -> Result<bool> {
        assert!(!dataset.id.is_empty());
        let mut tx = pool.begin().await?;
        let rows_affected = sqlx::query("UPDATE t_lighting_dataset SET display_name = ?,engine_type = ? , size = ? ,  count = ? WHERE id = ?")
            .bind(dataset.display_name)
            .bind(dataset.engine_type.getType())
            .bind(dataset.size)
            .bind(dataset.count)
            .bind(dataset.id)
            .execute(&mut tx)
            .await?
            .rows_affected();

        tx.commit().await?;
        Ok(rows_affected > 0)
    }

    async fn delete(id: String, pool: &Self::Pool) -> Result<bool> {
        let mut tx = pool.begin().await?;
        let field_affected = sqlx::query("DELETE FROM t_lighting_field WHERE dataset_id = ?")
            .bind(&id)
            .execute(&mut tx)
            .await?
            .rows_affected();
        println!("{}" , field_affected);

        let dataset_affected = sqlx::query("DELETE FROM t_lighting_dataset WHERE id = ?")
            .bind(&id)
            .execute(&mut tx)
            .await?
            .rows_affected();
        println!("{}" , dataset_affected);
        tx.commit().await?;
        Ok(true)
    }

    async fn find_all(pool: &Self::Pool) -> Result<Vec<Self::Result>> {
        todo!()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Field {
    pub id: String,
    pub name: String,
    pub dataset_id: String,
    pub data_type: DataType,
    pub field_type: String,
    pub display_name: String,
    pub formula: String,
}

impl Field {
    fn new() -> Self{
        Self {
            id:"".to_string(),
            name:"".to_string(),
            dataset_id:"".to_string(),
            data_type:DataType::Text,
            field_type:"".to_string(),
            display_name:"".to_string(),
            formula:"".to_string()
        }
    }

    fn dataset_id(mut self, dataset_id : String) -> Self{
        self.dataset_id = dataset_id;
        self
    }


}

#[async_trait]
impl CRUD for Field {
    type Result = Field;
    type Pool = MySqlPool;

    async fn create(model: Self::Result, pool: &Self::Pool) -> Result<Field> {
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

#[derive(Debug, Deserialize, Serialize, Clone , Copy)]
pub enum DataType {
    Text,
    Number,
    Date,
}

pub fn data_type_name(dataType : DataType) -> String{
    match dataType {
        Text=> "Test".to_string(),
        Number=> "Number".to_string(),
        Date=> "Date".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use serde::de::DeserializeOwned;
    use serde::Serialize;
    use serde_json::{Map, Value};
    use util_crait::uuid_util;

    #[test]
    fn test_ref(){
        let id_ref = Rc::new(RefCell::new("str".to_string()));
        let s1 = id_ref.borrow().clone();
        let s2 = id_ref.borrow().clone();
        println!("{} , {}" , s1 , s2);
    }

    #[test]
    fn test_uuid(){
        let id = uuid_util::get_uuid();
        let ref_ = RefCell::new(id);
        let mut field1 = Field::new();
        let mut field2 = Field::new();
        field1.dataset_id = ref_.borrow().clone();
        field2.dataset_id = ref_.borrow().clone();
        println!("{:?}" , field1);
        println!("{:?}" , field2);
    }


    #[test]
    fn test_json(){
        let json = serde_json::json!(Dataset :: new());
        let object  = json.as_object().unwrap();
        let mut columns = vec![];
        for(k , v) in object{
            columns.push(k)
        }
        println!("{:?}" , columns)

    }



    #[tokio::test]
    async fn test_crud() ->  Result<()> {
        dotenv::dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        println!("db url is {} " , database_url);
        let db_pool = MySqlPool::connect(&database_url).await?;

        let mut  ds = Dataset {
            id: "".to_string(),
            name: "".to_string(),
            display_name: "测试数据集".to_string(),
            fields: vec![],
            engine_type: Engine_Type::ClickHouse,
            size: 0.0,
            count: 0,
        };

        let field1 = Field {
            id:"".to_string(),
            name:"".to_string(),
            dataset_id:"".to_string(),
            data_type:DataType::Text,
            field_type:"".to_string(),
            display_name:"组织".to_string(),
            formula:"".to_string()
        };

        let field2 = Field {
            id:"".to_string(),
            name:"".to_string(),
            dataset_id:"".to_string(),
            data_type:DataType::Date,
            field_type:"".to_string(),
            display_name:"时间".to_string(),
            formula:"".to_string()
        };

        let field3 = Field {
            id:"".to_string(),
            name:"".to_string(),
            dataset_id:"".to_string(),
            data_type:DataType::Number,
            field_type:"".to_string(),
            display_name:"本期数".to_string(),
            formula:"".to_string()
        };

        let fields = vec![field1 , field2 , field3];
        ds.fields = fields;

        let ds1 = ds.clone();

        // add dataset
        let dataset : Dataset = Dataset::create(ds, &db_pool).await?;
        println!("{:?}" , dataset);

        let mut update_ds = dataset.clone();
        update_ds.display_name = "新的名称".to_string();
        let is_ok:bool = Dataset::update(update_ds, &db_pool).await?;
        assert!(is_ok);
        //
        // let result = Dataset::find_all(&db_pool).await?;
        //
        let is_ok = Dataset::delete(dataset.id , &db_pool).await?;
        assert!(is_ok);

        Ok(())
    }


}