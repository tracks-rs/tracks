use axum::{routing::{get, post, put, delete}, Router};
use crate::app::controllers::user;


pub fn app_routes() -> Router {
    Router::new()
        .route("/", get(crate::root))
        .route("/users", get(user::index))
        .route("/users", post(user::create))
        .route("/users/:id", get(user::show))
        .route("/users/:id", put(user::update))
        .route("/users/:id", delete(user::delete))
        
}
