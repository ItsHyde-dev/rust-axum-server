use std::convert::Infallible;
pub mod routes;

use axum::{http::HeaderMap, Json};
use serde::Serialize;
use serde_json::Value;

#[tokio::main]
async fn main() {
    let base_router = axum::Router::new().merge(routes::get_router());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    axum::serve(listener, base_router).await.unwrap();
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
struct ResponseForHello {
    hello: String,
}

#[allow(dead_code)]
async fn handle_hello_post(
    headers: HeaderMap,
    payload: Json<Value>,
) -> Result<Json<ResponseForHello>, Infallible> {
    dbg!(headers.get("content-type").unwrap());

    println!(
        "hello value: {}",
        &payload.get("hello").unwrap().to_string()
    );

    return Ok(Json(ResponseForHello {
        hello: "world".to_string(),
    }));
}
