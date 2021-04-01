use actix_web::error::BlockingError;
use actix_web::{web, HttpResponse, Result};
use juniper::graphiql::graphiql_source;
use juniper::http::playground::playground_source;
use juniper::http::GraphQLRequest;
use serde_json;
use std::sync::Arc;
use std::task::Context;
use user::models::human::Schema;

pub async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, BlockingError<serde_json::Error>> {
    let user = web::block(move || {
        let res = data.execute(&st, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(user))
}

pub async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(graphiql_source("/graphql"))
}
