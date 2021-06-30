use crate::query_dataset::{MutationDataset, QueryDataset};
use crate::query_formula::QueryFormula;
use crate::query_user::QueryUser;
use async_graphql::{FieldResult, MergedObject};

#[derive(MergedObject, Default)]
pub struct QueryRoot(QueryUser, QueryDataset, QueryFormula);

#[derive(MergedObject, Default)]
pub struct MutationRoot(MutationDataset);
