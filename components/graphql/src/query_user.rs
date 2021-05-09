use async_graphql::{Context, FieldResult, Object};
use sqlx::MySqlPool;
use user::User;

#[derive(Default)]
pub struct QueryUser;

#[Object]
impl QueryUser {
    async fn users(&self, ctx: &Context<'_>) -> FieldResult<Vec<User>> {
        let pool = ctx.data_unchecked::<MySqlPool>();
        let users = User::find_all(pool).await?;
        println!("users: {:?}", users);
        Ok(users)
    }
}
