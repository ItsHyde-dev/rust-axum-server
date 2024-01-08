use std::sync::{Arc, RwLock};

use sqlx::{mysql::MySqlPool, Error, MySql, Pool};

pub mod routes;

#[tokio::main]
async fn main() {
    println!("Initialized Application");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    let base_router = axum::Router::new()
        .merge(routes::get_router());

    axum::serve(listener, base_router).await.unwrap();

    println!("Server Started");
}

async fn connect_database() -> Result<Pool<MySql>, Error> {
    return MySqlPool::connect("mysql://root:password@localhost:3306/test").await;
}

#[derive(Clone)]
#[allow(dead_code)]
struct DatabaseConnection {
    connection: Pool<MySql>,
}
