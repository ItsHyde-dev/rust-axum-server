use axum::{
    routing::{get, post},
    Json,
};
use serde::Deserialize;
use serde_json::{json, Value};

pub fn user_router() -> axum::Router {
    let router = axum::Router::new()
        .route("/", get(get_users))
        .route("/create", post(create_user));

    return router;
}

async fn get_users() -> String {
    println!("Get users");
    return "users".to_string();
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct CreateUserDto {
    name: String,
    username: String,
    password: String,
}

async fn create_user(Json(body): Json<CreateUserDto>) -> Json<Value> {
    let response = json!({
        "status": 200,
        "data": json!({
                "name": body.name,
                "username": body.username,
                "password": body.password,
            })
    });

    return Json(response);
}
