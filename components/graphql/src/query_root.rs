use async_graphql::{Context, FieldResult, Object};
use crud_crait::CRUD;
use dataset::Dataset;
use sqlx::MySqlPool;
use user::User;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn users(&self, ctx: &Context<'_>) -> FieldResult<Vec<User>> {
        let pool = ctx.data_unchecked::<MySqlPool>();
        let users = User::find_all(pool).await?;
        println!("users: {:?}", users);
        Ok(users)
    }

    async fn datasets(&self, ctx: &Context<'_>) -> FieldResult<Vec<Dataset>> {
        let pool = ctx.data_unchecked::<MySqlPool>();
        // let datasets = Dataset::find_all(pool).await?;
        //println!("datasets: {:?}", datasets);
        Ok(vec![])
    }
}
