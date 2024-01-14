// Import necessary modules, e.g., Axum types, SeaORM entity
use axum::{
    http::StatusCode,
    extract::{Json,Path},
    response::IntoResponse
};
use serde::Deserialize;
// use crate::app::models::prelude::User;

// Retrieve a list of all users
pub async fn index() -> impl IntoResponse {
    // Implementation to fetch and return users
    todo!()
}

// Retrieve a specific user by ID
pub async fn show(Path(user_id): Path<i32>) -> impl IntoResponse {
    // Implementation to fetch and return a specific user
    todo!()
}

// Create a new user
pub async fn create(Json(params): Json<CreateUserParams>) -> impl IntoResponse {
    // Implementation to create a new user
    // StatusCode::CREATED
    // (StatusCode::CREATED, Json(user))
    todo!()
}

// Update an existing user by ID
pub async fn update(Path(user_id): Path<i32>, Json(user): Json<UpdateUserParams>) -> impl IntoResponse {
    // Implementation to update a user
    // StatusCode::OK
    todo!()
}

// Delete a user by ID
pub async fn delete(Path(user_id): Path<i32>) -> impl IntoResponse {
    // Implementation to delete a user
    StatusCode::NO_CONTENT
}




#[derive(Deserialize)]
pub struct CreateUserParams {
    username: String,
    email: String,
    password: String,
}
#[derive(Deserialize)]
pub struct UpdateUserParams {
    password: String,
}
