use std::collections::HashMap;
use std::convert::Infallible;
use warp::{Filter, Rejection};

pub fn route() -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    warp::path!("query")
        .and(warp::post())
        // Only accept bodies smaller than 16kb...
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and_then(list_sample)
}

async fn list_sample(data: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
    debug!("{:?}", data);
    Ok(warp::reply::json(&data))
}
