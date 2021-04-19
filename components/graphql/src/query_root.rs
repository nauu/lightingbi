use async_graphql::{Context, FieldResult, Object , MergedObject};
use crud_crait::CRUD;
use sqlx::MySqlPool;
use user::User;
use crate::query_user::QueryUser;
use crate::query_dataset::{QueryDataset, MutationDataset};


#[derive(MergedObject, Default)]
pub struct QueryRoot(QueryUser, QueryDataset);

#[derive(MergedObject, Default)]
pub struct MutationRoot(MutationDataset);


