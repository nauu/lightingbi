use crate::query_dataset::{MutationDataset, QueryDataset};
use crate::query_user::QueryUser;
use async_graphql::{Context, FieldResult, MergedObject, Object};
use crud_crait::CRUD;
use sqlx::MySqlPool;
use user::User;

#[derive(MergedObject, Default)]
pub struct QueryRoot(QueryUser, QueryDataset);

#[derive(MergedObject, Default)]
pub struct MutationRoot(MutationDataset);
