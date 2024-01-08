use std::sync::{Arc, RwLock};

use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::Executor;

use crate::DatabaseConnection;

pub fn user_router() -> Router<Arc<RwLock<DatabaseConnection>>> {
    let router = Router::new()
        .route("/", get(get_users))
        .route("/create", post(create_user));

    return router;
}

async fn get_users(State(database_connection): State<Arc<RwLock<DatabaseConnection>>>) -> String {
    println!("Get users");

    let result = database_connection
        .read()
        .unwrap()
        .connection
        .execute("SELECT * FROM users")
        .await
        .unwrap();

    println!("{:?}", result);

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
