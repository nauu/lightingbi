use anyhow::Result;
use async_trait::async_trait;
use crud_crait::CRUD;
use engine_craits::EngineType;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};
use util_crait::uuid_util;

use async_graphql::{InputObject, ServerResult, SimpleObject};
use crud_crait::entity::{Entity, MySqlRepository, Page, PageRequest};
use std::collections::BTreeMap;

///The entity of Dataset
#[derive(Debug, Deserialize, Serialize, Clone, FromRow)]
pub struct Dataset {
    ///primary key
    pub id: String,
    ///the physical table name
    pub name: String,
    ///the alias name to display
    pub display_name: String,
    ///the fields
    pub engine_type: String,
    pub size: f64,
    pub count: i32,
}

::async_graphql::scalar!(Dataset);

impl Entity for Dataset {}

impl Default for Dataset {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            name: "".to_string(),
            display_name: "".to_string(),
            engine_type: EngineType::ClickHouse.get_type(),
            size: 0.0,
            count: 0,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum DataType {
    Text,
    Number,
    Date,
}

impl DataType {
    pub fn get_type_name(&self) -> String {
        match self {
            Text => "Test".to_string(),
            Number => "Number".to_string(),
            Date => "Date".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, FromRow)]
pub struct Field {
    pub id: String,
    pub name: String,
    pub dataset_id: String,
    pub data_type: String,
    pub field_type: String,
    pub display_name: String,
    pub formula: String,
}

::async_graphql::scalar!(Field);

impl Entity for Field {}

impl Default for Field {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            name: "".to_string(),
            dataset_id: "".to_string(),
            data_type: DataType::Text.get_type_name(),
            field_type: "".to_string(),
            display_name: "".to_string(),
            formula: "".to_string(),
        }
    }
}

#[derive(InputObject, Debug)]
pub struct DataSetInputObject {
    pub dataset: Dataset,
    pub fields: Vec<Field>,
}

#[derive(SimpleObject, Debug)]
pub struct DataSetOutObject {
    pub dataset: Dataset,
    pub fields: Vec<Field>,
}

pub struct DataSetResolver;

impl DataSetResolver {
    pub async fn create(
        dataset: &DataSetInputObject,
        pool: &MySqlPool,
    ) -> Result<DataSetOutObject> {
        let mut new_dataset = dataset.dataset.clone();
        let id = uuid_util::get_uuid();
        new_dataset.id = id.clone();
        if new_dataset.name.is_empty() {
            new_dataset.name = "t_".to_string() + &uuid_util::get_short_uuid();
        }

        MySqlRepository::add(&new_dataset, &pool).await?;

        let mut new_fields = vec![];

        dataset
            .fields
            .iter()
            .map(|field| field.clone())
            .map(|mut field| {
                if field.id.is_empty() {
                    field.id = uuid_util::get_uuid();
                }
                if field.name.is_empty() {
                    field.name = "f_".to_string() + &uuid_util::get_short_uuid();
                }
                field.dataset_id = id.clone();
                field
            })
            .for_each(|field| {
                new_fields.push(field);
            });
        for field in &new_fields {
            MySqlRepository::add(field, &pool).await?;
        }
        Ok(DataSetOutObject {
            dataset: new_dataset,

            fields: new_fields,
        })
    }

    pub async fn find_by_id(id: &String, pool: &MySqlPool) -> Result<DataSetOutObject> {
        let dataset = MySqlRepository::find_by_id::<Dataset>(id, pool)
            .await?
            .unwrap();

        let mut params = BTreeMap::new();
        params.insert(String::from("dataset_id"), id.clone());
        let fields = MySqlRepository::query::<Field>(&params, pool).await?;

        Ok(DataSetOutObject { dataset, fields })
    }

    pub async fn find_by_page(
        page_request: &PageRequest,
        params: &BTreeMap<String, String>,
        pool: &MySqlPool,
    ) -> Result<Page<DataSetOutObject>> {
        let dataset_page =
            MySqlRepository::query_page::<Dataset>(&params, &page_request, pool).await?;

        let mut out_object = vec![];
        for dataset in dataset_page.context {
            let mut params = BTreeMap::new();
            params.insert(String::from("dataset_id"), dataset.id.clone());
            let fields = MySqlRepository::query::<Field>(&params, pool).await?;
            out_object.push(DataSetOutObject { dataset, fields })
        }
        let page = Page::new(
            dataset_page.size,
            dataset_page.num,
            dataset_page.count,
            dataset_page.total_page,
            out_object,
        );
        Ok(page)
    }
}

// #[async_trait]
// impl CRUD for Dataset {
//     type Result = Dataset;
//     type Pool = MySqlPool;
//
//     async fn create(dataset: Dataset, pool: &Self::Pool) -> Result<Dataset> {
//         let mut new_dataset = dataset.clone();
//         // let id = Rc::new(uuid_util::get_uuid());
//         // let ref_ = RefCell::new(id);

//
//         let mut tx = pool.begin().await?;
//         sqlx::query(" INSERT INTO t_lighting_dataset (id , name, display_name , engine_type , size , count) VALUES (?, ? , ? ,? ,? ,?)")
//             .bind(&new_dataset.id)
//             .bind(&new_dataset.name)
//             .bind(&new_dataset.display_name)
//             .bind(&new_dataset.engine_type.get_type())
//             .bind(&new_dataset.size)
//             .bind(&new_dataset.count)
//             .execute(&mut tx)
//             .await?;
//
//         if !new_dataset.fields.is_empty() {

//                 // field.dataset_id = ref_.borrow().clone();
//                 field.dataset_id = id.clone();
//                 println!("{:?}" , field);
//
//                 sqlx::query(" INSERT INTO t_lighting_field (id , name , dataset_id , data_type , field_type , display_name , formula) VALUES (?, ? , ? ,? ,? ,? , ?)")
//                     .bind(&field.id)
//                     .bind(&field.name)
//                     .bind(&field.dataset_id)
//                     .bind(&data_type_name(field.data_type))
//                     .bind(&field.field_type)
//                     .bind(&field.display_name)
//                     .bind(&field.formula)
//                     .execute(&mut tx)
//                     .await?;
//             }
//         }
//         println!("{:?}" , new_dataset);
//         tx.commit().await?;
//         Ok(new_dataset)
//     }
//
//
//
//
//     async fn update(dataset: Dataset, pool: &Self::Pool) -> Result<bool> {
//         assert!(!dataset.id.is_empty());
//         let mut tx = pool.begin().await?;
//         let rows_affected = sqlx::query("UPDATE t_lighting_dataset SET display_name = ?,engine_type = ? , size = ? ,  count = ? WHERE id = ?")
//             .bind(dataset.display_name)
//             .bind(dataset.engine_type.get_type())
//             .bind(dataset.size)
//             .bind(dataset.count)
//             .bind(dataset.id)
//             .execute(&mut tx)
//             .await?
//             .rows_affected();
//
//         tx.commit().await?;
//         Ok(rows_affected > 0)
//     }
//
//     async fn delete(id: String, pool: &Self::Pool) -> Result<bool> {
//         let mut tx = pool.begin().await?;
//         let field_affected = sqlx::query("DELETE FROM t_lighting_field WHERE dataset_id = ?")
//             .bind(&id)
//             .execute(&mut tx)
//             .await?
//             .rows_affected();
//         println!("{}" , field_affected);
//
//         let dataset_affected = sqlx::query("DELETE FROM t_lighting_dataset WHERE id = ?")
//             .bind(&id)
//             .execute(&mut tx)
//             .await?
//             .rows_affected();
//         println!("{}" , dataset_affected);
//         tx.commit().await?;
//         Ok(true)
//     }
//
//     async fn find_all(pool: &Self::Pool) -> Result<Vec<Self::Result>> {
//         todo!()
//     }
// }

// #[derive(Debug, Deserialize, Serialize, Clone)]
// pub struct Field {
//     pub id: String,
//     pub name: String,
//     pub dataset_id: String,
//     pub data_type: DataType,
//     pub field_type: String,
//     pub display_name: String,
//     pub formula: String,
// }
//
// impl Field {
//     fn new() -> Self{
//         Self {
//             id:"".to_string(),
//             name:"".to_string(),
//             dataset_id:"".to_string(),
//             data_type:DataType::Text,
//             field_type:"".to_string(),
//             display_name:"".to_string(),
//             formula:"".to_string()
//         }
//     }
//
//     fn dataset_id(mut self, dataset_id : String) -> Self{
//         self.dataset_id = dataset_id;
//         self
//     }
//
//
// }

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

#[cfg(test)]
mod tests {
    use super::*;
    use engine_craits::EngineType;
    use serde::de::DeserializeOwned;
    use serde::Serialize;
    use serde_json::{Map, Value};
    use std::env;
    use util_crait::uuid_util;

    #[tokio::test]
    async fn test_add() -> Result<()> {
        dotenv::dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        println!("db url is {} ", database_url);
        let db_pool = MySqlPool::connect(&database_url).await?;

        let mut ds = Dataset {
            id: "".to_string(),
            name: "".to_string(),
            display_name: "测试数据集".to_string(),
            engine_type: EngineType::ClickHouse.get_type(),
            size: 0.0,
            count: 0,
        };

        let field1 = Field {
            id: "".to_string(),
            name: "".to_string(),
            dataset_id: "".to_string(),
            data_type: DataType::Text.get_type_name(),
            field_type: "".to_string(),
            display_name: "组织".to_string(),
            formula: "".to_string(),
        };

        let field2 = Field {
            id: "".to_string(),
            name: "".to_string(),
            dataset_id: "".to_string(),
            data_type: DataType::Date.get_type_name(),
            field_type: "".to_string(),
            display_name: "时间".to_string(),
            formula: "".to_string(),
        };

        let field3 = Field {
            id: "".to_string(),
            name: "".to_string(),
            dataset_id: "".to_string(),
            data_type: DataType::Number.get_type_name(),
            field_type: "".to_string(),
            display_name: "本期数".to_string(),
            formula: "".to_string(),
        };

        let fields = vec![field1, field2, field3];

        let datasetInput = DataSetInputObject {
            dataset: ds,
            fields,
        };

        let output = DataSetResolver::create(&datasetInput, &db_pool).await?;
        println!("{:?}", output);

        Ok(())
    }

    //
    // #[test]
    // fn test_ref(){
    //     let id_ref = Rc::new(RefCell::new("str".to_string()));
    //     let s1 = id_ref.borrow().clone();
    //     let s2 = id_ref.borrow().clone();
    //     println!("{} , {}" , s1 , s2);
    // }
    //
    // #[test]
    // fn test_uuid(){
    //     let id = uuid_util::get_uuid();
    //     let ref_ = RefCell::new(id);
    //     let mut field1 = Field::new();
    //     let mut field2 = Field::new();
    //     field1.dataset_id = ref_.borrow().clone();
    //     field2.dataset_id = ref_.borrow().clone();
    //     println!("{:?}" , field1);
    //     println!("{:?}" , field2);
    // }
    //
    //
    // #[test]
    // fn test_json(){
    //     let json = serde_json::json!(Dataset :: new());
    //     let object  = json.as_object().unwrap();
    //     let mut columns = vec![];
    //     for(k , v) in object{
    //         columns.push(k)
    //     }
    //     println!("{:?}" , columns)
    //
    // }
    //
    //
    //
    // #[tokio::test]
    // async fn test_crud() ->  Result<()> {
    //     dotenv::dotenv().ok();
    //     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    //     println!("db url is {} " , database_url);
    //     let db_pool = MySqlPool::connect(&database_url).await?;
    //
    //     let mut  ds = Dataset {
    //         id: "".to_string(),
    //         name: "".to_string(),
    //         display_name: "测试数据集".to_string(),
    //         size: 0.0,
    //         count: 0,
    //     };
    //
    //     let field1 = Field {
    //         id:"".to_string(),
    //         name:"".to_string(),
    //         dataset_id:"".to_string(),
    //         data_type:DataType::Text,
    //         field_type:"".to_string(),
    //         display_name:"组织".to_string(),
    //         formula:"".to_string()
    //     };
    //
    //     let field2 = Field {
    //         id:"".to_string(),
    //         name:"".to_string(),
    //         dataset_id:"".to_string(),
    //         data_type:DataType::Date,
    //         field_type:"".to_string(),
    //         display_name:"时间".to_string(),
    //         formula:"".to_string()
    //     };
    //
    //     let field3 = Field {
    //         id:"".to_string(),
    //         name:"".to_string(),
    //         dataset_id:"".to_string(),
    //         data_type:DataType::Number,
    //         field_type:"".to_string(),
    //         display_name:"本期数".to_string(),
    //         formula:"".to_string()
    //     };
    //
    //     let fields = vec![field1 , field2 , field3];
    //     ds.fields = fields;
    //
    //     let ds1 = ds.clone();
    //
    //     // add dataset
    //     let dataset : Dataset = Dataset::create(ds, &db_pool).await?;
    //     println!("{:?}" , dataset);
    //
    //     let mut update_ds = dataset.clone();
    //     update_ds.display_name = "新的名称".to_string();
    //     let is_ok:bool = Dataset::update(update_ds, &db_pool).await?;
    //     assert!(is_ok);
    //     //
    //     // let result = Dataset::find_all(&db_pool).await?;
    //     //
    //     let is_ok = Dataset::delete(dataset.id , &db_pool).await?;
    //     assert!(is_ok);
    //
    //     Ok(())
    // }
}
