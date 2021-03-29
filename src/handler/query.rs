use actix_web::{post, web, Error, HttpRequest, HttpResponse};
use json::JsonValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MyObj {
    name: String,
    number: i32,
}

/// This handler uses json extractor with limit
#[post("/query")]
pub async fn query(item: web::Json<MyObj>, req: HttpRequest) -> HttpResponse {
    println!("request: {:?}", req);
    println!("model: {:?}", &item);
    HttpResponse::Ok().json(item.0) // <- send response
}

/// This handler manually load request payload and parse json-rust
#[post("/query_str")]
pub async fn query_str(body: web::Bytes) -> Result<HttpResponse, Error> {
    // body is loaded, now we can deserialize json-rust
    let result = json::parse(std::str::from_utf8(&body).unwrap()); // return Result
    let v_json: JsonValue = match result {
        Ok(v) => v,
        Err(e) => json::object! {"err" => e.to_string() },
    };
    println!("model: {:?}", v_json);

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(v_json.dump()))
}
