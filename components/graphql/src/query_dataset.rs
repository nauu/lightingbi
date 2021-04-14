use async_graphql::{Context, FieldResult, Object};
use crud_crait::CRUD;
use sqlx::MySqlPool;

#[derive(Default)]
pub struct QueryDataset;


#[Object]
impl QueryDataset{

    async fn datasets(&self, ctx: &Context<'_>) -> FieldResult<Vec<String>> {
        let pool = ctx.data_unchecked::<MySqlPool>();
        // let datasets = Dataset::find_all(pool).await?;
        //println!("datasets: {:?}", datasets);
        Ok(vec![])
    }
}