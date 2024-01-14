use sea_orm::{entity::*, Database};
use axum::{routing::get, Router};

// Imports
mod app;
mod config;
use tracks::serve_application;

#[tokio::main]
async fn main() {
    // Set up the database connection
    // let database_url = "sqlite::memory:";
    // let db_conn = Database::connect(database_url).await.unwrap();

    // let app = config::routes::app_routes();
    
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // axum::serve(listener, app).await.unwrap();
    serve_application().await;

}


async fn root() -> &'static str {
    "Hello, World!"
}