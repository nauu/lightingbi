use async_graphql::{Context, FieldResult, Object};
use crud_crait::entity::{Page, PageRequest};
use crud_crait::CRUD;
use dataset::{DataSetInputObject, DataSetOutObject, DataSetResolver, Dataset};
use sqlx::MySqlPool;
use std::collections::BTreeMap;

#[derive(Default)]
pub struct QueryDataset;

#[Object]
impl QueryDataset {
    async fn datasets(&self, ctx: &Context<'_>) -> FieldResult<Vec<String>> {
        // let pool = ctx.data_unchecked::<MySqlPool>();
        // let datasets = Dataset::find_all(pool).await?;
        // println!("datasets: {:?}", datasets);
        // Ok(datasets)
        Ok(vec![])
    }

    async fn find_dataset_by_id(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> FieldResult<DataSetOutObject> {
        let pool = ctx.data_unchecked::<MySqlPool>();

        let output = DataSetResolver::find_by_id(&id, pool).await?;
        Ok(output)
    }

    async fn find_dataset_page(
        &self,
        ctx: &Context<'_>,
        page: PageRequest,
    ) -> FieldResult<Page<DataSetOutObject>> {
        let pool = ctx.data_unchecked::<MySqlPool>();
        let params = BTreeMap::new();

        let output = DataSetResolver::find_by_page(&page, &params, pool).await?;
        Ok(output)
    }
}

#[derive(Default)]
pub struct MutationDataset;

#[Object]
impl MutationDataset {
    async fn create_dataset(
        &self,
        ctx: &Context<'_>,
        dataset_object: DataSetInputObject,
    ) -> FieldResult<DataSetOutObject> {
        let pool = ctx.data_unchecked::<MySqlPool>();
        let output = DataSetResolver::create(&dataset_object, pool).await?;
        Ok(output)
    }
}
