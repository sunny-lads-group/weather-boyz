use serde::{Deserialize, Serialize};
use sqlx::types::time::PrimitiveDateTime;

#[derive(Serialize, Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: Option<PrimitiveDateTime>,
    pub updated_at: Option<PrimitiveDateTime>,
}

#[derive(Deserialize, Debug)]
pub struct SignInData {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct CurrentUser {
    pub email: String,
    pub name: String,
    pub password_hash: String,
}
