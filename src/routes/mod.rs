use std::sync::{Arc, RwLock};

use axum::Router;

use crate::{connect_database, DatabaseConnection};
mod users;

pub async fn get_router() -> Router {
    let pool: DatabaseConnection = DatabaseConnection {
        connection: connect_database().await.unwrap(),
    };

    let pool = Arc::new(RwLock::new(pool));
    let router = Router::new()
        .with_state(Arc::clone(&pool))
        .nest("/users", users::user_router());

    return router;
}
