use axum::Router;
mod users;

pub fn get_router() -> Router {

    let router = Router::new()
        .nest("/users", users::user_router());

    return router;
}
