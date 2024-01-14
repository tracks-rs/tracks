// use sea_orm::entity::prelude::*;

// #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
// #[sea_orm(table_name = "cake")]
pub struct User {
    // #[sea_orm(primary_key)]
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: Option<String>,
    pub encrypted_password: String,
    pub created_at: String,
    pub updated_at: String,
}