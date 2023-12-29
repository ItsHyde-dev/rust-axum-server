use std::convert::Infallible;

use axum::{
    http::HeaderMap,
    routing::{get, post},
    Json,
};
use serde_json::Value;
use serde::Serialize;

#[tokio::main]
async fn main() {
    let base_router = axum::Router::new()
        .route("/hello", get(handle_hello))
        .route("/hello", post(handle_hello_post));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    axum::serve(listener, base_router).await.unwrap();
}

async fn handle_hello() -> String {
    println!("Hello world should be printed");
    "hello world".to_string()
}

#[derive(Debug, Serialize)]
struct ResponseForHello {
    hello: String,
}

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
