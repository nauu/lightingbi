use crate::RootSchema;
use actix_web::{web, HttpRequest, HttpResponse, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{Request, Response};
use std::sync::Arc;

pub async fn graphql(schema: web::Data<RootSchema>, data: Request) -> Response {
    schema.execute(data.into_inner()).await.into()
}

pub async fn graphql_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
        )))
}
